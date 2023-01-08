// =============================================================================
//! - Clock model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-08
//! - Since: 2023-01-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::GENES_MAX;
use crate::engine::input::Input;
use crate::engine::traits::Model;

#[derive(Default)]
pub struct Clock {
  pub time: usize,
}

impl Model for Clock {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested || self.time >= GENES_MAX - 1 {
      self.time = 0;
    } else {
      self.time += 1;
    }
  }
}
