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
  blight: bool,
  bug: bool,
  bug_position_index: usize,
  flora: bool,
  flora_growth_rate: usize,
  garden_off: bool,
  garden_on: bool,
  reset: bool,
  speed: bool,
}

impl Input {
  pub fn clear(&mut self) {
    self.blight = false;
    self.bug = false;
    self.bug_position_index = 0;
    self.flora = false;
    self.flora_growth_rate = 0;
    // TODO: change to garden_toggled
    self.garden_off = false;
    self.garden_on = false;
    self.reset = false;
    self.speed = false;
  }
}

impl InputReader for Input {
  fn get_blight(&self) -> bool {
    self.blight
  }

  fn get_bug(&self) -> bool {
    self.bug
  }

  fn get_bug_position_index(&self) -> usize {
    self.bug_position_index
  }

  fn get_flora(&self) -> bool {
    self.flora
  }

  fn get_flora_growth_rate(&self) -> usize {
    self.flora_growth_rate
  }

  fn get_garden_off(&self) -> bool {
    self.garden_off
  }

  fn get_garden_on(&self) -> bool {
    self.garden_on
  }

  fn get_reset(&self) -> bool {
    self.reset
  }

  fn get_speed(&self) -> bool {
    self.speed
  }
}

impl InputWriter for Input {
  fn request_blight(&mut self) {
    self.blight = true;
  }

  fn request_bug(
    &mut self,
    position_index: usize,
  ) {
    self.bug = true;
    self.bug_position_index = position_index;
  }

  fn request_flora(
    &mut self,
    flora_growth_rate: usize,
  ) {
    self.flora = true;
    self.flora_growth_rate = flora_growth_rate;
  }

  fn request_garden_off(&mut self) {
    self.garden_off = true;
  }

  fn request_garden_on(&mut self) {
    self.garden_on = true;
  }

  fn request_reset(&mut self) {
    self.reset = true;
  }

  fn request_speed(&mut self) {
    self.speed = true;
  }
}
