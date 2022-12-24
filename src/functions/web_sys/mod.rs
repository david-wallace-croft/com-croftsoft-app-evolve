// =============================================================================
//! - web-sys functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-12-24
//! - Since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// TODO: spin this off into its own crate and then pull it in as a dependency
// TODO: see https://github.com/rustwasm/gloo

use anyhow::{anyhow, Result};
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window, Document, Element, HtmlElement, Window};

type LoopClosure = Closure<dyn FnMut(f64)>;

// TODO: Move this to another crate and pull it back in as a dependency
pub trait LoopUpdater {
  fn update(
    &mut self,
    update_time: f64,
  );
}

pub fn add_change_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
  let (mut change_sender, change_receiver) = unbounded();
  let on_change = Closure::wrap(Box::new(move || {
    let _result: Result<(), futures::channel::mpsc::SendError> =
      change_sender.start_send(());
  }) as Box<dyn FnMut()>);
  elem.set_onchange(Some(on_change.as_ref().unchecked_ref()));
  on_change.forget();
  change_receiver
}

pub fn add_change_handler_by_id(id: &str) -> Option<UnboundedReceiver<()>> {
  let html_element = get_html_element_by_id(id);
  // TODO: return None if fails
  Some(add_change_handler(html_element))
}

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

pub fn spawn_local_loop<L: LoopUpdater + 'static>(loop_updater: L) {
  wasm_bindgen_futures::spawn_local(async move {
    start_looping(loop_updater).await.expect("loop start failed");
  });
}

pub async fn start_looping<L: LoopUpdater + 'static>(
  mut loop_updater: L
) -> Result<()> {
  let f: Rc<RefCell<Option<LoopClosure>>> = Rc::new(RefCell::new(None));
  let g = f.clone();
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move |update_time: f64| {
    loop_updater.update(update_time);
    let _result: Result<i32, anyhow::Error> =
      request_animation_frame(f.borrow().as_ref().unwrap());
  })));
  request_animation_frame(
    g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
  )?;
  Ok(())
}
