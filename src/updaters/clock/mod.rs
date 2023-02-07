// =============================================================================
//! - Clock Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-06
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

pub trait ClockUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
}

pub struct ClockUpdater {
  clock: Rc<RefCell<Clock>>,
  inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
}

impl ClockUpdater {
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
  ) -> Self {
    Self {
      clock,
      inputs,
    }
  }
}

impl Updater for ClockUpdater {
  fn update(&mut self) {
    let mut clock: RefMut<Clock> = self.clock.borrow_mut();
    let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    if clock.time >= GENES_MAX - 1 || inputs.get_reset_requested() {
      clock.time = 0;
    } else {
      clock.time += 1;
    }
  }
}
