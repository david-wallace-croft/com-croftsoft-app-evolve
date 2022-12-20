// =============================================================================
//! - WorldLooper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-19
//! - Rust since: 2022-12-15
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

use crate::components::evolve::EvolveComponent;
use crate::constants::FRAME_PERIOD_MILLIS;

// TODO: rename this to EvolveLooper
pub struct WorldLooper<const G: usize> {
  // TODO: Make EvolveComponent implement a Looper trait then use directly
  evolve_component: EvolveComponent<G>,
  // TODO: move this into evolve component if the user can adjust the frame rate
  frame_period_millis: f64,
}

impl<const G: usize> WorldLooper<G> {
  // TODO: maybe rename this to get_millis_until_next_update()
  // TODO: maybe rename this to get_next_update_time()
  // TODO: or maybe this is not needed at all if it decides when to update
  pub fn get_frame_period_millis(&self) -> f64 {
    self.frame_period_millis
  }

  pub fn init(&mut self) {
    self.evolve_component.init();
  }

  // TODO: maybe rename this to update()
  // TODO: maybe return a boolean if it should be updated again
  pub fn loop_once(&mut self) {
    self.evolve_component.update();
  }

  // TODO: add a new() that takes an initial configuration for values
  // such as frame rate and other parameters
}

impl<const G: usize> Default for WorldLooper<G> {
  fn default() -> Self {
    Self {
      evolve_component: EvolveComponent::<G>::new("evolve"),
      frame_period_millis: FRAME_PERIOD_MILLIS,
    }
  }
}
