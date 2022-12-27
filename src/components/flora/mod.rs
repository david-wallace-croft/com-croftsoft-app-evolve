// =============================================================================
//! - Component for the Flora growth rate input
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-25
//! - Rust since: 2022-12-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{FLORA_GROWTH_RATE_INIT, FLORA_GROWTH_RATE_MAX};
use crate::functions::web_sys::add_change_handler_by_id;
use crate::models::world::World;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct FloraComponent {
  pub id: String,
  pub unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
}

impl FloraComponent {
  pub fn init(&mut self) {
    self.unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      unbounded_receiver_option: None,
    }
  }

  pub fn make_html(&self) -> String {
    format!(
      "Food growth rate <input id=\"{}\" max=\"{}\" type=\"range\" value\"{}\">",
      self.id, FLORA_GROWTH_RATE_MAX, FLORA_GROWTH_RATE_INIT,
    )
  }

  pub fn update(
    &mut self,
    world: &mut World,
  ) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        let value: String = html_input_element.value();
        let v: Result<usize, _> = value.parse();
        world.requested_flora = Some(v.unwrap());
      }
    }
  }

  // private methods

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
}