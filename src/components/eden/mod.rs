// =============================================================================
//! - Component for the Garden of Eden button
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-24
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
use crate::models::world::World;
use futures::channel::mpsc::UnboundedReceiver;

pub struct EdenComponent {
  pub id: String,
  pub unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl EdenComponent {
  pub fn init(&mut self) {
    self.unbounded_receiver = add_change_handler_by_id(&self.id);
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      unbounded_receiver: None,
    }
  }

  pub fn make_html(&self) -> String {
    format!(
      "Garden of Eden <input id=\"{}\" type=\"checkbox\" checked>",
      self.id
    )
  }

  pub fn changed(&mut self) -> bool {
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
    world: &mut World,
  ) {
    if self.changed() {
      world.requested_eden = true;
    }
  }
}
