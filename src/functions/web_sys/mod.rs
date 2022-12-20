// =============================================================================
//! - web-sys functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-19
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::loopers::world::WorldLooper;
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
pub fn spawn_local_loop<const G: usize>(world_looper: WorldLooper<G>) {
  wasm_bindgen_futures::spawn_local(async move {
    start_looping(world_looper).await.expect("loop start failed");
  });
}

pub async fn start_looping<const G: usize>(
  mut world_looper: WorldLooper<G>
) -> Result<()> {
  let mut last_update_time = get_window()?
    .performance()
    .ok_or_else(|| anyhow!("Performance object not found"))?
    .now();
  let f: Rc<RefCell<Option<LoopClosure>>> = Rc::new(RefCell::new(None));
  let g = f.clone();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move |performance: f64| {
    let frame_period_millis: f64 = world_looper.get_frame_period_millis();
    if performance - last_update_time > frame_period_millis {
      last_update_time = performance;
      world_looper.loop_once();
    }
    let _result: Result<i32, anyhow::Error> =
      request_animation_frame(f.borrow().as_ref().unwrap());
  })));
  request_animation_frame(
    g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
  )?;
  Ok(())
}
