// =============================================================================
//! - Component for the Garden of Eden button
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-03
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

use crate::functions::web_sys::add_change_handler_by_id;
use crate::traits::InputWriter;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct GardenComponent {
  pub id: String,
  pub event_unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
}

impl GardenComponent {
  pub fn init(&mut self) {
    self.event_unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      event_unbounded_receiver_option: None,
    }
  }

  pub fn make_html(&self) -> String {
    format!(
      "Garden of Eden <input id=\"{}\" type=\"checkbox\" checked>",
      self.id
    )
  }

  pub fn update<I: InputWriter>(
    &mut self,
    input: &mut I,
  ) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        if html_input_element.checked() {
          input.request_garden_on();
        } else {
          input.request_garden_off();
        }
      }
    }
  }

  // private methods

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
}
