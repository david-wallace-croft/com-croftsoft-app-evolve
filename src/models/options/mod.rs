// =============================================================================
//! - Options Model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-27
//! - Updated: 2023-03-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterOptions;

#[derive(Default)]
pub struct Options {
  pub pause: bool,
  pub time_display: bool,
  pub update_rate_display: bool,
}

impl WorldUpdaterOptions for Options {
  fn get_pause(&self) -> bool {
    self.pause
  }

  fn get_time_display(&self) -> bool {
    self.time_display
  }

  fn get_update_rate_display(&self) -> bool {
    self.update_rate_display
  }
}
