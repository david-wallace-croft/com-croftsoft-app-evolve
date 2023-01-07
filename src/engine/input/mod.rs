// =============================================================================
//! - Input model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-07
//! - Since: 2022-12-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

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
