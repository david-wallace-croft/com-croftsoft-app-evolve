// =============================================================================
//! - WorldUpdater for CroftSoft Evolve
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

use super::bugs::BugsUpdater;
use super::flora::FloraUpdater;
use crate::constants::{
  EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::World;
use rand::{rngs::ThreadRng, Rng};

pub struct WorldUpdater<const G: usize> {
  pub bugs_updater: BugsUpdater<G>,
  pub flora_updater: FloraUpdater<G>,
}

impl<const G: usize> WorldUpdater<G> {
  pub fn update(
    &self,
    world: &mut World<G>,
  ) {
    self.flora_updater.update(world);
    self.bugs_updater.update(world);
  }
}

impl<const G: usize> Default for WorldUpdater<G> {
  fn default() -> Self {
    let bugs_updater = BugsUpdater::default();
    let flora_updater = FloraUpdater::default();
    Self {
      bugs_updater,
      flora_updater,
    }
  }
}
