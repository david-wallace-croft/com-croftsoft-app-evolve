// =============================================================================
//! - Associated functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-10
//! - Rust since: 2022-11-27
//! - Java version: 2008-04-19
//! - Java since: 1996-09-01
//!
//! # History
//! - Adapted from the Java package com.croftsoft.apps.evolve
//!   - In the Java-based [`CroftSoft Apps Library`]
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

use crate::models::world::constants::{
  BABY_ENERGY, BIRTH_ENERGY, FLORA_ENERGY, GENES_MAX, MAX_ENERGY, SPACE_WIDTH,
};
use crate::models::world::structures::World;

impl<const G: usize> World<G> {
  pub fn create_status_string(
    bugs_alive: usize,
    time: usize,
    genes_average_string: &str,
  ) -> String {
    format!(
      "Alive: {}  Time: {}  Average Movement Genes {}",
      bugs_alive, time, genes_average_string
    )
  }

  pub fn to_index_from_xy(
    x: usize,
    y: usize,
  ) -> usize {
    SPACE_WIDTH * y + x
  }

  pub fn to_x_from_index(index: usize) -> usize {
    index % SPACE_WIDTH
  }

  pub fn to_y_from_index(index: usize) -> usize {
    index / SPACE_WIDTH
  }
}
