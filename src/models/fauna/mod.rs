// =============================================================================
//! - Fauna model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-05
//! - Rust since: 2023-01-05
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

use super::bug::Bug;
use super::flora::Flora;
use crate::constants::{
  BIRTH_ENERGY, BUGS_MAX, FLORA_ENERGY, MAX_ENERGY, MOVE_COST, SPACE_HEIGHT,
  SPACE_WIDTH,
};
use crate::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
use crate::traits::InputReader;

#[derive(Debug, Default)]
pub struct Fauna {
  pub bugs: Vec<Bug>,
}

impl Fauna {
  fn reset(&mut self) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    self.bugs.clear();
    for _i in 0..BUGS_MAX {
      let bug = Bug::new(position);
      // let bug_str = format!("{:?}", bug);
      // console::log_1(&JsValue::from_str(&bug_str));
      self.bugs.push(bug);
    }
  }

  pub fn update<I: InputReader>(
    &mut self,
    input: &I,
    flora: &mut Flora,
    time: usize,
  ) {
    if input.get_reset_requested() {
      self.reset();
      return;
    }
    let mut new_bugs = Vec::<Bug>::new();
    let bugs_length = self.bugs.len();
    if bugs_length < BUGS_MAX {
      if let Some(position_index) = input.get_bug_requested() {
        let new_bug = Bug::new(position_index);
        new_bugs.push(new_bug);
      }
    }
    for bug in self.bugs.iter_mut() {
      if bug.energy == 0 {
        continue;
      }
      let bug_position: usize = bug.position;
      let mut x = to_x_from_index(bug_position);
      let mut y = to_y_from_index(bug_position);
      if flora.flora_present[bug_position] {
        flora.flora_present[bug_position] = false;
        bug.energy = bug.energy.saturating_add(FLORA_ENERGY);
        if bug.energy > MAX_ENERGY {
          bug.energy = MAX_ENERGY;
        }
      }
      if bug.energy >= BIRTH_ENERGY && bugs_length + new_bugs.len() < BUGS_MAX {
        let new_bug = bug.give_birth();
        new_bugs.push(new_bug);
      }
      if rand::random() {
        if bug.genes_x[time] {
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
        if bug.genes_y[time] {
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
    self.bugs.retain(|bug| bug.energy > 0);
    self.bugs.append(&mut new_bugs);
  }
}
