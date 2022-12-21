// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
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

use crate::constants::{INIT_GROWTH_RATE, SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::bug::Bug;

pub struct World {
  pub bugs: Vec<Bug>,
  pub bugs_alive: usize,
  pub enabled_eden: bool,
  pub flora_growth_rate: usize,
  pub flora_present: [bool; SPACE_HEIGHT * SPACE_WIDTH],
  pub growth_rate_spinner_number_model: usize,
  pub requested_blight: bool,
  pub requested_eden: bool,
  pub requested_reset: bool,
  pub time: usize,
}

impl Default for World {
  fn default() -> Self {
    World {
      bugs: Vec::<Bug>::new(),
      bugs_alive: 0,
      enabled_eden: false,
      flora_growth_rate: INIT_GROWTH_RATE,
      flora_present: [false; SPACE_HEIGHT * SPACE_WIDTH],
      growth_rate_spinner_number_model: 0,
      requested_blight: false,
      requested_eden: false,
      requested_reset: true,
      time: 0,
    }
  }
}
