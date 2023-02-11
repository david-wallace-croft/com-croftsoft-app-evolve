// =============================================================================
//! - Inputs for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-31
//! - Updated: 2023-02-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterInputs;

#[derive(Default)]
pub struct Inputs {
  pub blight_requested: bool,
  pub bug_requested: Option<usize>,
  pub flora_growth_rate_change_requested: Option<usize>,
  pub frame_rate_display_change_requested: Option<bool>,
  pub garden_change_requested: Option<bool>,
  pub reset_requested: bool,
  pub speed_change_requested: Option<usize>,
  pub update_time_millis: f64,
}

impl Inputs {
  pub fn clear(&mut self) {
    self.blight_requested = false;
    self.bug_requested = None;
    self.flora_growth_rate_change_requested = None;
    self.frame_rate_display_change_requested = None;
    self.garden_change_requested = None;
    self.reset_requested = false;
    self.speed_change_requested = None;
    self.update_time_millis = 0.;
  }
}

impl WorldUpdaterInputs for Inputs {
  fn get_blight_requested(&self) -> bool {
    self.blight_requested
  }

  fn get_bug_requested(&self) -> Option<usize> {
    self.bug_requested
  }

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize> {
    self.flora_growth_rate_change_requested
  }

  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.frame_rate_display_change_requested
  }

  fn get_garden_change_requested(&self) -> Option<bool> {
    self.garden_change_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }

  fn get_speed_change_requested(&self) -> Option<usize> {
    self.speed_change_requested
  }

  fn get_update_time_millis(&self) -> f64 {
    self.update_time_millis
  }
}
