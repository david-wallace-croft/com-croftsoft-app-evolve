// =============================================================================
//! - Component for the blight button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-21
//! - Rust since: 2022-12-14
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

use crate::engine::functions::web_sys::add_click_handler_by_id;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::UnboundedReceiver;
use std::rc::Rc;

pub struct BlightComponent {
  id: String,
  input: Rc<RefCell<Input>>,
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
    input: Rc<RefCell<Input>>,
  ) -> Self {
    Self {
      id: String::from(id),
      input,
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
      self.input.borrow_mut().blight_requested = true;
    }
  }
}
