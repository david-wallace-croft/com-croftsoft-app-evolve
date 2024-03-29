// =============================================================================
//! - Component for the Flora growth rate input
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-25
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{FLORA_GROWTH_RATE_INIT, FLORA_GROWTH_RATE_MAX};
use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::add_change_handler_by_id;
use com_croftsoft_lib_role::{InitializerMut, UpdaterMut};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct FloraComponent {
  id: String,
  inputs: Rc<RefCell<Inputs>>,
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
    inputs: Rc<RefCell<Inputs>>,
  ) -> Self {
    Self {
      id: String::from(id),
      inputs,
      unbounded_receiver_option: None,
    }
  }
}

impl Component for FloraComponent {
  fn make_html(&self) -> String {
    format!(
      "Food growth rate <input id=\"{}\" max=\"{}\" type=\"range\" value=\"{}\">",
      self.id, FLORA_GROWTH_RATE_MAX, FLORA_GROWTH_RATE_INIT,
    )
  }
}

impl InitializerMut for FloraComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl UpdaterMut for FloraComponent {
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
        self.inputs.borrow_mut().flora_growth_rate_change_requested =
          Some(v.unwrap());
      }
    }
  }
}
