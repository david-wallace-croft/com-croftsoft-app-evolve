// =============================================================================
//! - web-sys functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::components::evolve::EvolveComponent;
use anyhow::{anyhow, Result};
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window, Document, Element, HtmlElement, Window};

type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
  let (mut click_sender, click_receiver) = unbounded();
  let on_click = Closure::wrap(Box::new(move || {
    let _result: Result<(), futures::channel::mpsc::SendError> =
      click_sender.start_send(());
  }) as Box<dyn FnMut()>);
  elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
  on_click.forget();
  click_receiver
}

pub fn add_click_handler_by_id(id: &str) -> Option<UnboundedReceiver<()>> {
  let html_element = get_html_element_by_id(id);
  // TODO: return None if fails
  Some(add_click_handler(html_element))
}

pub fn get_html_element_by_id(id: &str) -> HtmlElement {
  let document: Document = window().unwrap().document().unwrap();
  let element: Element = document.get_element_by_id(id).unwrap();
  element.dyn_into().unwrap()
}

pub fn get_window() -> Result<Window> {
  web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn log(message: &str) {
  console::log_1(&JsValue::from_str(message));
}

pub fn request_animation_frame(
  callback: &Closure<dyn FnMut(f64)>
) -> Result<i32> {
  get_window()?
    .request_animation_frame(callback.as_ref().unchecked_ref())
    .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

// TODO: change this to an Updater / Looper trait
pub fn spawn_local_loop(evolve_component: EvolveComponent) {
  wasm_bindgen_futures::spawn_local(async move {
    start_looping(evolve_component).await.expect("loop start failed");
  });
}

pub async fn start_looping(
  mut evolve_component: EvolveComponent
) -> Result<()> {
  let f: Rc<RefCell<Option<LoopClosure>>> = Rc::new(RefCell::new(None));
  let g = f.clone();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move |update_time: f64| {
    evolve_component.update(update_time);
    let _result: Result<i32, anyhow::Error> =
      request_animation_frame(f.borrow().as_ref().unwrap());
  })));
  // TODO: Do we even need a looper? Might be able to loop just by
  // requesting animation frames.
  request_animation_frame(
    g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
  )?;
  Ok(())
}
