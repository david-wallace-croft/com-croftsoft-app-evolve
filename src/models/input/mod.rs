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

  pub fn get_blight(&self) -> bool {
    self.blight
  }

  pub fn get_bug(&self) -> bool {
    self.bug
  }

  pub fn get_bug_position_index(&self) -> usize {
    self.bug_position_index
  }

  pub fn get_flora(&self) -> bool {
    self.flora
  }

  pub fn get_flora_growth_rate(&self) -> usize {
    self.flora_growth_rate
  }

  pub fn get_garden_off(&self) -> bool {
    self.garden_off
  }

  pub fn get_garden_on(&self) -> bool {
    self.garden_on
  }

  pub fn get_reset(&self) -> bool {
    self.reset
  }

  pub fn get_speed(&self) -> bool {
    self.speed
  }

  pub fn request_blight(&mut self) {
    self.blight = true;
  }

  pub fn request_bug(
    &mut self,
    position_index: usize,
  ) {
    self.bug = true;
    self.bug_position_index = position_index;
  }

  pub fn request_flora(
    &mut self,
    flora_growth_rate: usize,
  ) {
    self.flora = true;
    self.flora_growth_rate = flora_growth_rate;
  }

  pub fn request_garden_off(&mut self) {
    self.garden_off = true;
  }

  pub fn request_garden_on(&mut self) {
    self.garden_on = true;
  }

  pub fn request_reset(&mut self) {
    self.reset = true;
  }

  pub fn request_speed(&mut self) {
    self.speed = true;
  }
}
