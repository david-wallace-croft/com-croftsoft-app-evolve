// =============================================================================
//! - Component for the Flora growth rate input
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-21
//! - Since: 2022-12-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{FLORA_GROWTH_RATE_INIT, FLORA_GROWTH_RATE_MAX};
use crate::engine::functions::web_sys::add_change_handler_by_id;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct FloraComponent {
  id: String,
  input: Rc<RefCell<Input>>,
  unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
}

impl FloraComponent {
  fn changed(&mut self) -> Option<Event> {
    let unbounded_receiver: &mut UnboundedReceiver<Event> =
      self.unbounded_receiver_option.as_mut()?;
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
      unbounded_receiver_option: None,
    }
  }
}

impl Component for FloraComponent {
  fn make_html(&self) -> String {
    format!(
      "Food growth rate <input id=\"{}\" max=\"{}\" type=\"range\" value\"{}\">",
      self.id, FLORA_GROWTH_RATE_MAX, FLORA_GROWTH_RATE_INIT,
    )
  }
}

impl Initializer for FloraComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl Updater for FloraComponent {
  fn update(&mut self) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        let value: String = html_input_element.value();
        let v: Result<usize, _> = value.parse();
        self.input.borrow_mut().flora_growth_rate_change_requested =
          Some(v.unwrap());
      }
    }
  }
}
