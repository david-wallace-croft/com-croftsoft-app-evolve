// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-26
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

use crate::constants::{FLORA_GROWTH_RATE_INIT, SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::bug::Bug;

pub struct World {
  pub bugs: Vec<Bug>,
  pub enabled_eden: bool,
  pub flora_growth_rate: usize,
  pub flora_present: [bool; SPACE_HEIGHT * SPACE_WIDTH],
  // TODO: move requested flags out of World model
  pub requested_blight: bool,
  pub requested_bug: Option<usize>,
  pub requested_eden: Option<bool>,
  pub requested_flora: Option<usize>,
  pub requested_reset: bool,
  pub requested_speed: bool,
  pub time: usize,
}

impl Default for World {
  fn default() -> Self {
    World {
      bugs: Vec::<Bug>::new(),
      enabled_eden: true,
      flora_growth_rate: FLORA_GROWTH_RATE_INIT,
      flora_present: [false; SPACE_HEIGHT * SPACE_WIDTH],
      requested_blight: false,
      requested_bug: None,
      requested_eden: None,
      requested_flora: None,
      requested_reset: true,
      requested_speed: false,
      time: 0,
    }
  }
}
