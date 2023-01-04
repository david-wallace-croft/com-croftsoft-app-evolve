// =============================================================================
//! - Input model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-03
//! - Since: 2022-12-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::traits::{InputReader, InputWriter};

// TODO: split into read-only and write-only parts
#[derive(Debug, Default)]
pub struct Input {
  blight_requested: bool,
  bug_requested: Option<usize>,
  flora_growth_rate_change_requested: Option<usize>,
  garden_change_requested: Option<bool>,
  reset_requested: bool,
  speed_toggle_requested: bool,
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

impl InputReader for Input {
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

  fn get_speed_toggle_requested(&self) -> bool {
    self.speed_toggle_requested
  }
}

impl InputWriter for Input {
  fn request_blight(&mut self) {
    self.blight_requested = true;
  }

  fn request_bug(
    &mut self,
    position_index: usize,
  ) {
    self.bug_requested = Some(position_index);
  }

  fn request_flora_growth_rate_change(
    &mut self,
    flora_growth_rate: usize,
  ) {
    self.flora_growth_rate_change_requested = Some(flora_growth_rate);
  }

  fn request_garden_change(
    &mut self,
    enabled: bool,
  ) {
    self.garden_change_requested = Some(enabled);
  }

  fn request_reset(&mut self) {
    self.reset_requested = true;
  }

  fn request_speed_toggle(&mut self) {
    self.speed_toggle_requested = true;
  }
}
