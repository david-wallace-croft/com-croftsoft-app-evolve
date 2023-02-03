// =============================================================================
//! - Clock Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2023-01-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::GENES_MAX;
use crate::models::clock::Clock;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait ClockUpdaterInput {
  fn get_reset_requested(&self) -> bool;
}

pub struct ClockUpdater {
  clock: Rc<RefCell<Clock>>,
  input: Rc<RefCell<dyn ClockUpdaterInput>>,
}

impl ClockUpdater {
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    input: Rc<RefCell<dyn ClockUpdaterInput>>,
  ) -> Self {
    Self {
      clock,
      input,
    }
  }
}

impl Updater for ClockUpdater {
  fn update(&mut self) {
    let mut clock: RefMut<Clock> = self.clock.borrow_mut();
    let input: Ref<dyn ClockUpdaterInput> = self.input.borrow();
    if clock.time >= GENES_MAX - 1 || input.get_reset_requested() {
      clock.time = 0;
    } else {
      clock.time += 1;
    }
  }
}