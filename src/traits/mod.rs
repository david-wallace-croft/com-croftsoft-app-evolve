// =============================================================================
//! - Traits for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-3
//! - Since: 2023-01-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait InputReader {
  fn get_blight(&self) -> bool;

  fn get_bug(&self) -> bool;

  fn get_bug_position_index(&self) -> usize;

  fn get_flora(&self) -> bool;

  fn get_flora_growth_rate(&self) -> usize;

  fn get_garden_off(&self) -> bool;

  fn get_garden_on(&self) -> bool;

  fn get_reset(&self) -> bool;

  fn get_speed(&self) -> bool;
}

pub trait InputWriter {
  fn request_blight(&mut self);

  fn request_bug(
    &mut self,
    position_index: usize,
  );

  fn request_flora(
    &mut self,
    flora_growth_rate: usize,
  );

  fn request_garden_off(&mut self);

  fn request_garden_on(&mut self);

  fn request_reset(&mut self);

  fn request_speed(&mut self);
}
