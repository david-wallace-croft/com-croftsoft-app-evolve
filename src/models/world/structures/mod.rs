// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-10
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

use crate::models::bug::Bug;
use crate::models::world::constants::{SPACE_HEIGHT, SPACE_WIDTH};

pub struct World<const G: usize> {
  // TODO: animatedComponent
  pub bugs: Vec<Bug<G>>,
  pub bugs_alive: usize,
  // TODO: droughtButton
  pub eden_check_box: bool,
  // TODO: random
  pub flora_growth_rate: usize,
  pub flora_present: [bool; SPACE_HEIGHT * SPACE_WIDTH],
  pub growth_rate_spinner_number_model: usize,
  // TODO: resetButton
  pub time: usize,
}
