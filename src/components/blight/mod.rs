// =============================================================================
//! - Component for the blight button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-14
//! - Updated: 2023-03-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::add_click_handler_by_id;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::UnboundedReceiver;
use std::rc::Rc;

pub struct BlightComponent {
  id: String,
  inputs: Rc<RefCell<Inputs>>,
  unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl BlightComponent {
  fn clicked(&mut self) -> bool {
    if self.unbounded_receiver.is_none() {
      return false;
    }
    matches!(
      self.unbounded_receiver.as_mut().unwrap().try_next(),
      Ok(Some(()))
    )
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

impl Component for BlightComponent {
  fn make_html(&self) -> String {
    format!("<button id=\"{}\">Blight</button>", self.id)
  }
}

impl Initializer for BlightComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }
}

impl Updater for BlightComponent {
  fn update(&mut self) {
    if self.clicked() {
      self.inputs.borrow_mut().blight_requested = true;
    }
  }
}
