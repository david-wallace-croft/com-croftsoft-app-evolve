// =============================================================================
//! - Trait implementations for CroftSoft Evolve
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

use crate::models::bug::Bug;
use crate::models::world::{
  constants::{BUGS_MAX, SPACE_HEIGHT, SPACE_WIDTH},
  structures::World,
};

impl<const G: usize> Default for World<G> {
  fn default() -> Self {
    World {
      bugs: Vec::<Bug<G>>::new(),
      bugs_alive: 0,
      eden_check_box: false,
      flora_growth_rate: 0,
      flora_present: [false; SPACE_HEIGHT * SPACE_WIDTH],
      growth_rate_spinner_number_model: 0,
      time: 0,
    }
  }
}
