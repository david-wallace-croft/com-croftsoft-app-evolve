// =============================================================================
//! - Component for the reset button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-08
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

use crate::engine::functions::web_sys::add_click_handler_by_id;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use futures::channel::mpsc::UnboundedReceiver;

pub struct ResetComponent {
  id: String,
  unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl ResetComponent {
  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
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
  fn init(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }

  fn make_html(&self) -> String {
    format!("<button id=\"{}\">Reset</button>", self.id)
  }

  fn update(
    &mut self,
    input: &mut Input,
  ) {
    if self.pressed() {
      input.reset_requested = true;
    }
  }
}
