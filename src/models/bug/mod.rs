// =============================================================================
//! - Bug model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-30
//! - Since: 2022-12-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::GENES_MAX;

pub struct Bug {
  pub energy: usize,
  pub genes_x: [bool; GENES_MAX],
  pub genes_y: [bool; GENES_MAX],
  pub position: usize,
  pub species: Species,
}

#[derive(Clone, Copy)]
pub enum Species {
  Cruiser,
  Normal,
  Twirlie,
}
