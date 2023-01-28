// =============================================================================
//! - Fauna Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-28
//! - Since: 2023-01-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{
  BIRTH_ENERGY, BUGS_MAX, FLORA_ENERGY, MAX_ENERGY, MOVE_COST, SPACE_HEIGHT,
  SPACE_WIDTH,
};
use crate::engine::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
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

  fn update_bugs(
    &mut self,
    bugs_length: usize,
    new_bugs: &mut Vec<Bug>,
  ) {
    for bug in self.fauna.borrow_mut().bugs.iter_mut() {
      let bug_position: usize = bug.position;
      if self.flora.borrow().flora_present[bug_position] {
        self.flora.borrow_mut().flora_present[bug_position] = false;
        bug.energy = bug.energy.saturating_add(FLORA_ENERGY);
        if bug.energy > MAX_ENERGY {
          bug.energy = MAX_ENERGY;
        }
      }
      if bug.energy >= BIRTH_ENERGY && bugs_length + new_bugs.len() < BUGS_MAX {
        let new_bug = bug.give_birth();
        new_bugs.push(new_bug);
      }
      let mut x = to_x_from_index(bug_position);
      let mut y = to_y_from_index(bug_position);
      if rand::random() {
        if bug.genes_x[self.clock.borrow().time] {
          if x < SPACE_WIDTH - 1 {
            x += 1;
          } else {
            x = 0;
          }
        } else if x > 0 {
          x -= 1;
        } else {
          x = SPACE_WIDTH - 1;
        }
      }
      if rand::random() {
        if bug.genes_y[self.clock.borrow().time] {
          if y < SPACE_HEIGHT - 1 {
            y += 1;
          } else {
            y = 0;
          }
        } else if y > 0 {
          y -= 1;
        } else {
          y = SPACE_HEIGHT - 1;
        }
      }
      bug.position = to_index_from_xy(x, y);
      bug.energy = bug.energy.saturating_sub(MOVE_COST);
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
    self.update_bugs(bugs_length, &mut new_bugs);
    self.fauna.borrow_mut().bugs.retain(|bug| bug.energy > 0);
    self.fauna.borrow_mut().bugs.append(&mut new_bugs);
  }
}
