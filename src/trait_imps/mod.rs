// =============================================================================
//! - Trait implementations for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-29
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

use crate::structures::Evolve;

impl<const G: usize, const L: usize> Default for Evolve<G, L> {
  fn default() -> Self {
    Evolve {
      bugs: Vec::new(),
      bugs_alive: 0,
      flora_growth_rate: 0,
      flora_present: [false; L],
      time: 0,
    }
  }
}
