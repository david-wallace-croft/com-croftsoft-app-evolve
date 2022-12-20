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

use super::wasm_bindgen::{closure_wrap, LoopClosure};
use anyhow::{anyhow, Result};
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window, Document, Element, HtmlElement, Window};

pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
  let (mut click_sender, click_receiver) = unbounded();
  let on_click = closure_wrap(Box::new(move || {
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

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
  get_window()?
    .request_animation_frame(callback.as_ref().unchecked_ref())
    .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}
