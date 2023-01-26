// =============================================================================
//! - Flora Model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2023-01-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{FLORA_GROWTH_RATE_INIT, LOCATION_COUNT};

pub struct Flora {
  pub enabled_garden: bool,
  pub flora_growth_rate: usize,
  pub flora_present: [bool; LOCATION_COUNT],
}

impl Default for Flora {
  fn default() -> Self {
    Self {
      enabled_garden: true,
      flora_growth_rate: FLORA_GROWTH_RATE_INIT,
      flora_present: [false; LOCATION_COUNT],
    }
  }
}
