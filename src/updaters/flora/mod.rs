// =============================================================================
//! - FloraUpdater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-24
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
  EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::functions::location::to_index_from_xy;
use crate::models::world::World;
use rand::{rngs::ThreadRng, Rng};

#[derive(Default)]
pub struct FloraUpdater {}

impl FloraUpdater {
  pub fn update(
    &self,
    world: &mut World,
  ) {
    if world.requested_blight {
      world.requested_blight = false;
      for i in 0..SPACE_HEIGHT * SPACE_WIDTH {
        world.flora_present[i] = false;
      }
    } else {
      let mut thread_rng: ThreadRng = rand::thread_rng();
      for _i in 0..world.flora_growth_rate {
        // Randomly position food flora
        let index: usize = thread_rng.gen_range(0..SPACE_HEIGHT * SPACE_WIDTH);
        world.flora_present[index] = true;
      }
    }
    self.update_garden(world);
  }

  // private methods

  fn set_garden_values(
    &self,
    world: &mut World,
    value: bool,
  ) {
    for x in EDEN_X0..=EDEN_X1 {
      for y in EDEN_Y0..=EDEN_Y1 {
        let index: usize = to_index_from_xy(x, y);
        world.flora_present[index] = value;
      }
    }
  }

  fn update_garden(
    &self,
    world: &mut World,
  ) {
    if world.requested_eden {
      world.requested_eden = false;
      world.enabled_eden = !world.enabled_eden;
      if !world.enabled_eden {
        self.set_garden_values(world, false);
      }
    }
    if world.enabled_eden {
      self.set_garden_values(world, true);
    }
  }
}
