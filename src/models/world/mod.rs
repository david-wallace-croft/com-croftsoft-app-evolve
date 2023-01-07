// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-07
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
use crate::engine::input::Input;
use crate::engine::traits::Model;

#[derive(Default)]
pub struct World {
  pub fauna: Fauna,
  pub flora: Flora,
  pub time: usize,
}

impl Model for World {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested || self.time >= GENES_MAX - 1 {
      self.time = 0;
    } else {
      self.time += 1;
    }
    self.flora.update(input);
    self.fauna.update(input, &mut self.flora, self.time);
  }
}
