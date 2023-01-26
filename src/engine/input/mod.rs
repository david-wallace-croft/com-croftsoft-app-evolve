// =============================================================================
//! - Input for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2022-12-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterInput;

#[derive(Default)]
pub struct Input {
  pub blight_requested: bool,
  pub bug_requested: Option<usize>,
  pub flora_growth_rate_change_requested: Option<usize>,
  pub garden_change_requested: Option<bool>,
  pub reset_requested: bool,
  pub speed_toggle_requested: bool,
}

impl Input {
  pub fn clear(&mut self) {
    self.blight_requested = false;
    self.bug_requested = None;
    self.flora_growth_rate_change_requested = None;
    self.garden_change_requested = None;
    self.reset_requested = false;
    self.speed_toggle_requested = false;
  }
}

impl WorldUpdaterInput for Input {
  fn get_blight_requested(&self) -> bool {
    self.blight_requested
  }

  fn get_bug_requested(&self) -> Option<usize> {
    self.bug_requested
  }

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize> {
    self.flora_growth_rate_change_requested
  }

  fn get_garden_change_requested(&self) -> Option<bool> {
    self.garden_change_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }
}
