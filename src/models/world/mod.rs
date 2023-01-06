// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-05
//! - Rust since: 2022-12-10
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

use super::fauna::Fauna;
use super::flora::Flora;
use crate::constants::GENES_MAX;
use crate::traits::InputReader;

#[derive(Default)]
pub struct World {
  pub fauna: Fauna,
  pub flora: Flora,
  pub time: usize,
}

impl World {
  pub fn update<I: InputReader>(
    &mut self,
    input: &I,
  ) {
    self.flora.update(input);
    self.fauna.update(input, &mut self.flora, self.time);
    self.time = self.time.saturating_add(1);
    if self.time >= GENES_MAX {
      self.time = 0;
    }
  }
}
