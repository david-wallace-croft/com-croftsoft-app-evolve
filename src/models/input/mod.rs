// =============================================================================
//! - Input model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-12-31
//! - Since: 2022-12-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Debug, Default)]
pub struct Input {
  pub blight: bool,
  pub bug: bool,
  pub bug_position_index: usize,
  pub flora: bool,
  pub flora_growth_rate: usize,
  pub garden_off: bool,
  pub garden_on: bool,
  pub reset: bool,
  pub speed: bool,
}

impl Input {
  pub fn reset(&mut self) {
    self.blight = false;
    self.bug = false;
    self.bug_position_index = 0;
    self.flora = false;
    self.flora_growth_rate = 0;
    self.garden_off = false;
    self.garden_on = false;
    self.reset = false;
    self.speed = false;
  }
}
