// =============================================================================
//! - BugsUpdater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-10
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
  BIRTH_ENERGY, BUGS_MAX, EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, FLORA_ENERGY,
  GENES_MAX, MAX_ENERGY, MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::functions::{to_index_from_xy, to_x_from_index, to_y_from_index};
use crate::models::bug::Bug;
use crate::models::world::World;
use rand::{rngs::ThreadRng, Rng};

pub struct BugsUpdater<const G: usize> {}

impl<const G: usize> BugsUpdater<G> {
  pub fn update(
    &self,
    world: &mut World<G>,
  ) {
    world.time = world.time.saturating_add(1);
    if world.time >= GENES_MAX {
      world.time = 0;
    }
    let mut new_bugs = Vec::<Bug<G>>::new();
    let bugs_length = world.bugs.len();
    for bug in world.bugs.iter_mut() {
      if bug.energy == 0 {
        continue;
      }
      let mut x = to_x_from_index(bug.position);
      let mut y = to_y_from_index(bug.position);
      if world.flora_present[bug.position] {
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
    let mut dead_bug_indices = Vec::<usize>::new();
    for (index, bug) in world.bugs.iter_mut().enumerate() {
      if bug.energy == 0 {
        dead_bug_indices.push(index);
      }
    }
    for dead_bug_index in dead_bug_indices {
      world.bugs.remove(dead_bug_index);
    }
    world.bugs.append(&mut new_bugs);
  }
}

impl<const G: usize> Default for BugsUpdater<G> {
  fn default() -> Self {
    Self {}
  }
}
