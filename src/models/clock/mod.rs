// =============================================================================
//! - Clock model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-20
//! - Since: 2023-01-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::GENES_MAX;
use crate::engine::input::Input;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Clock {
  input: Rc<RefCell<Input>>,
  pub time: usize,
}

impl Clock {
  pub fn new(input: Rc<RefCell<Input>>) -> Self {
    Self {
      input,
      time: 0,
    }
  }
}

impl Updater for Clock {
  fn update(&mut self) {
    if self.input.borrow().reset_requested || self.time >= GENES_MAX - 1 {
      self.time = 0;
    } else {
      self.time += 1;
    }
  }
}
