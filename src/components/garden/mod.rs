// =============================================================================
//! - Component for the Garden of Eden button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-21
//! - Rust since: 2022-12-16
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

use crate::engine::functions::web_sys::add_change_handler_by_id;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct GardenComponent {
  event_unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
  id: String,
  input: Rc<RefCell<Input>>,
}

impl GardenComponent {
  fn changed(&mut self) -> Option<Event> {
    let unbounded_receiver: &mut UnboundedReceiver<Event> =
      self.event_unbounded_receiver_option.as_mut()?;
    let result: Result<Option<Event>, TryRecvError> =
      unbounded_receiver.try_next();
    if let Ok(event_option) = result {
      return event_option;
    }
    None
  }

  pub fn new(
    id: &str,
    input: Rc<RefCell<Input>>,
  ) -> Self {
    Self {
      id: String::from(id),
      input,
      event_unbounded_receiver_option: None,
    }
  }
}

impl Component for GardenComponent {
  fn make_html(&self) -> String {
    format!(
      "Garden of Eden <input id=\"{}\" type=\"checkbox\" checked>",
      self.id
    )
  }
}

impl Initializer for GardenComponent {
  fn initialize(&mut self) {
    self.event_unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl Updater for GardenComponent {
  fn update(&mut self) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        self.input.borrow_mut().garden_change_requested =
          Some(html_input_element.checked());
      }
    }
  }
}
