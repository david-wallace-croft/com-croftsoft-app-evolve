// =============================================================================
//! - Associated functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-02
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

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{
  constants::{BIRTH_ENERGY, FLORA_ENERGY, GENES_MAX, MAX_ENERGY, SPACE_WIDTH},
  structures::{Bug, Evolve},
};

impl<const G: usize> Evolve<G> {
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
