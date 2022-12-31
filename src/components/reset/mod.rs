// =============================================================================
//! - Component for the reset button
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-31
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

use crate::functions::web_sys::add_click_handler_by_id;
use crate::models::input::Input;
use futures::channel::mpsc::UnboundedReceiver;

pub struct ResetComponent {
  pub id: String,
  pub unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl ResetComponent {
  pub fn init(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      unbounded_receiver: None,
    }
  }

  pub fn make_html(&self) -> String {
    format!("<button id=\"{}\">Reset</button>", self.id)
  }

  pub fn pressed(&mut self) -> bool {
    if self.unbounded_receiver.is_none() {
      return false;
    }
    matches!(
      self.unbounded_receiver.as_mut().unwrap().try_next(),
      Ok(Some(()))
    )
  }

  pub fn update(
    &mut self,
    input: &mut Input,
  ) {
    if self.pressed() {
      input.request_reset();
    }
  }
}
