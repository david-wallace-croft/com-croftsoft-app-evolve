// =============================================================================
//! - FloraUpdater for CroftSoft Evolve
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
  EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::World;
use rand::{rngs::ThreadRng, Rng};

pub struct FloraUpdater<const G: usize> {}

impl<const G: usize> FloraUpdater<G> {
  pub fn update(
    &self,
    world: &mut World<G>,
  ) {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    for _i in 0..world.flora_growth_rate {
      // Randomly position food flora
      let index: usize = thread_rng.gen_range(0..SPACE_HEIGHT * SPACE_WIDTH);
      world.flora_present[index] = true;
    }
    // Replenishing the Garden of Eden
    if world.eden_check_box {
      for x in EDEN_X0..=EDEN_X1 {
        for y in EDEN_Y0..=EDEN_Y1 {
          let index: usize = World::<G>::to_index_from_xy(x, y);
          world.flora_present[index] = true;
        }
      }
    }
  }
}

impl<const G: usize> Default for FloraUpdater<G> {
  fn default() -> Self {
    Self {}
  }
}
