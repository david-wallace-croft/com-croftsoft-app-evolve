// =============================================================================
//! - BugsUpdater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-31
//! - Rust since: 2022-12-10
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

use crate::constants::{
  BIRTH_ENERGY, BUGS_MAX, FLORA_ENERGY, GENES_MAX, MAX_ENERGY, MOVE_COST,
  SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
use crate::models::bug::Bug;
use crate::models::input::Input;
use crate::models::world::World;

#[derive(Default)]
pub struct BugsUpdater {}

impl BugsUpdater {
  pub fn update(
    &self,
    input: &Input,
    world: &mut World,
  ) {
    world.time = world.time.saturating_add(1);
    if world.time >= GENES_MAX {
      world.time = 0;
    }
    let mut new_bugs = Vec::<Bug>::new();
    let bugs_length = world.bugs.len();
    if bugs_length < BUGS_MAX && input.get_bug() {
      let new_bug = Bug::new(input.get_bug_position_index());
      new_bugs.push(new_bug);
    }
    for bug in world.bugs.iter_mut() {
      if bug.energy == 0 {
        continue;
      }
      let bug_position: usize = bug.position;
      let mut x = to_x_from_index(bug_position);
      let mut y = to_y_from_index(bug_position);
      if world.flora_present[bug_position] {
        world.flora_present[bug_position] = false;
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
        if bug.genes_x[world.time] {
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
        if bug.genes_y[world.time] {
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
    world.bugs.retain(|bug| bug.energy > 0);
    world.bugs.append(&mut new_bugs);
  }
}
