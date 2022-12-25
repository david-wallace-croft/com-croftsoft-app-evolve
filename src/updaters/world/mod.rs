// =============================================================================
//! - WorldUpdater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
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
  BUGS_MAX, FLORA_GROWTH_RATE_INIT, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::functions::location::to_index_from_xy;
use crate::models::bug::Bug;
use crate::models::world::World;

pub struct WorldUpdater {
  pub bugs_updater: BugsUpdater,
  pub flora_updater: FloraUpdater,
}

impl WorldUpdater {
  pub fn reset(
    &self,
    world: &mut World,
  ) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    world.bugs.clear();
    for _i in 0..BUGS_MAX {
      let bug = Bug::new(position);
      // let bug_str = format!("{:?}", bug);
      // console::log_1(&JsValue::from_str(&bug_str));
      world.bugs.push(bug);
    }
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      world.flora_present[index] = true;
    }
    world.enabled_eden = true;
    world.growth_rate_spinner_number_model = FLORA_GROWTH_RATE_INIT; // TODO: event?
  }

  pub fn update(
    &self,
    world: &mut World,
  ) {
    if world.requested_reset {
      world.requested_reset = false;
      self.reset(world);
    } else {
      self.flora_updater.update(world);
      self.bugs_updater.update(world);
    }
  }
}

impl Default for WorldUpdater {
  fn default() -> Self {
    let bugs_updater = BugsUpdater::default();
    let flora_updater = FloraUpdater::default();
    Self {
      bugs_updater,
      flora_updater,
    }
  }
}
