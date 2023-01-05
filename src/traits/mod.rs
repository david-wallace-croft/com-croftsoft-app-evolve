// =============================================================================
//! - Traits for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-04
//! - Since: 2023-01-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait InputReader {
  fn get_blight_requested(&self) -> bool;

  fn get_bug_requested(&self) -> Option<usize>;

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;

  fn get_garden_change_requested(&self) -> Option<bool>;

  fn get_reset_requested(&self) -> bool;

  fn get_speed_toggle_requested(&self) -> bool;
}

pub trait InputWriter {
  fn request_blight(&mut self);

  fn request_bug(
    &mut self,
    position_index: usize,
  );

  fn request_flora_growth_rate_change(
    &mut self,
    flora_growth_rate: usize,
  );

  fn request_garden_change(
    &mut self,
    enabled: bool,
  );

  fn request_reset(&mut self);

  fn request_speed_toggle(&mut self);
}

pub trait Model<I: InputReader> {
  fn update(
    &mut self,
    input: &I,
  );
}
