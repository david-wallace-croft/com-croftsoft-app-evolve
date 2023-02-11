// =============================================================================
//! - Component for the speed button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-20
//! - Updated: 2023-02-11
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::functions::web_sys::add_change_handler_by_id;
use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct SpeedComponent {
  id: String,
  inputs: Rc<RefCell<Inputs>>,
  unbounded_receiver: Option<UnboundedReceiver<Event>>,
}

impl SpeedComponent {
  fn changed(&mut self) -> Option<Event> {
    let unbounded_receiver: &mut UnboundedReceiver<Event> =
      self.unbounded_receiver.as_mut()?;
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
      unbounded_receiver: None,
    }
  }
}

impl Component for SpeedComponent {
  fn make_html(&self) -> String {
    format!(
      "Speed <input id=\"{}\" min=\"1\" max=\"60\" type=\"range\" value=\"1\">",
      self.id
    )
  }
}

impl Initializer for SpeedComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver = add_change_handler_by_id(&self.id);
  }
}

impl Updater for SpeedComponent {
  fn update(&mut self) {
    if let Some(event) = self.changed() {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        let value: String = html_input_element.value();
        let v: Result<usize, _> = value.parse();
        self.inputs.borrow_mut().speed_change_requested = Some(v.unwrap());
      }
    }
  }
}
