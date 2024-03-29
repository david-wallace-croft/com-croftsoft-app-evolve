// =============================================================================
//! - Component for the reset button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-17
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::add_click_handler_by_id;
use com_croftsoft_lib_role::{InitializerMut, UpdaterMut};
use core::cell::RefCell;
use futures::channel::mpsc::UnboundedReceiver;
use std::rc::Rc;

pub struct ResetComponent {
  id: String,
  inputs: Rc<RefCell<Inputs>>,
  unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl ResetComponent {
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

  fn pressed(&mut self) -> bool {
    if self.unbounded_receiver.is_none() {
      return false;
    }
    matches!(
      self.unbounded_receiver.as_mut().unwrap().try_next(),
      Ok(Some(()))
    )
  }
}

impl Component for ResetComponent {
  fn make_html(&self) -> String {
    format!("<button id=\"{}\">Reset</button>", self.id)
  }
}

impl InitializerMut for ResetComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }
}

impl UpdaterMut for ResetComponent {
  fn update(&mut self) {
    if self.pressed() {
      self.inputs.borrow_mut().reset_requested = true;
    }
  }
}
