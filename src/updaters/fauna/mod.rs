// =============================================================================
//! - Fauna Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2023-01-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{BUGS_MAX, SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::to_index_from_xy;
use crate::models::bug::Bug;
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::rc::Rc;

pub trait FaunaUpdaterInput {
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_reset_requested(&self) -> bool;
}

pub struct FaunaUpdater {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  flora: Rc<RefCell<Flora>>,
  input: Rc<RefCell<dyn FaunaUpdaterInput>>,
}

impl FaunaUpdater {
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    fauna: Rc<RefCell<Fauna>>,
    flora: Rc<RefCell<Flora>>,
    input: Rc<RefCell<dyn FaunaUpdaterInput>>,
  ) -> Self {
    Self {
      clock,
      fauna,
      flora,
      input,
    }
  }

  fn reset(&mut self) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    let mut fauna: RefMut<Fauna> = self.fauna.borrow_mut();
    fauna.bugs.clear();
    for _i in 0..BUGS_MAX {
      let bug = Bug::new(position);
      fauna.bugs.push(bug);
    }
  }
}

impl Updater for FaunaUpdater {
  fn update(&mut self) {
    if self.input.borrow().get_reset_requested() {
      self.reset();
      return;
    }
    let mut new_bugs = Vec::<Bug>::new();
    let bugs_length = self.fauna.borrow().bugs.len();
    if bugs_length < BUGS_MAX {
      if let Some(position_index) = self.input.borrow().get_bug_requested() {
        let new_bug = Bug::new(position_index);
        new_bugs.push(new_bug);
      }
    }
    let clock = &self.clock.borrow();
    let flora = &mut self.flora.borrow_mut();
    let mut fauna: RefMut<Fauna> = self.fauna.borrow_mut();
    for bug in fauna.bugs.iter_mut() {
      // TODO: Can I implement WorldUpdater trait for Bug?
      bug.update(bugs_length, clock, flora, &mut new_bugs);
    }
    fauna.bugs.retain(|bug| bug.energy > 0);
    fauna.bugs.append(&mut new_bugs);
  }
}
