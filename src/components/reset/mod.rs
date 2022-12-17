// =============================================================================
//! - Component for the reset button
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-17
//! - Rust since: 2022-12-17
//! - Java version: 2008-04-19
//! - Java since: 1996-09-01
//!
//! # History
//! - Adapted from the Java package com.croftsoft.apps.evolve
//!   - In the Java-based [`CroftSoft Apps Library`]
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::world::World;
use futures::channel::mpsc::unbounded;
use futures::channel::mpsc::UnboundedReceiver;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{window, Document, Element, HtmlElement};

pub struct ResetComponent<const G: usize> {
  pub unbounded_receiver: UnboundedReceiver<()>,
}

impl<const G: usize> ResetComponent<G> {
  fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
    let (mut click_sender, click_receiver) = unbounded();
    let on_click = ResetComponent::<G>::closure_wrap(Box::new(move || {
      let _result: Result<(), futures::channel::mpsc::SendError> =
        click_sender.start_send(());
    })
      as Box<dyn FnMut()>);
    elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();
    click_receiver
  }

  fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
  }

  pub fn initialize() -> Option<Self> {
    let document: Document = window().unwrap().document().unwrap();
    let element_option: Option<Element> = document.get_element_by_id("reset");
    if let Some(element) = element_option {
      let html_element: HtmlElement = element.dyn_into().unwrap();
      let unbounded_receiver: UnboundedReceiver<()> =
        ResetComponent::<G>::add_click_handler(html_element);
      return Some(Self {
        unbounded_receiver,
      });
    }
    None
  }

  pub fn make_html() -> String {
    String::from("<button id=\"reset\">Reset</button>")
  }

  pub fn pressed(&mut self) -> bool {
    matches!(self.unbounded_receiver.try_next(), Ok(Some(())))
  }

  pub fn update(
    &mut self,
    world: &mut World<G>,
  ) {
    if !self.pressed() {
      return;
    }
    console::log_1(&JsValue::from_str("reset button pressed"));
    world.requested_reset = true;
  }
}
