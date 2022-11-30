// =============================================================================
//! - Structures for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-30
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

use core::cell::RefCell;

pub struct Bug<const G: usize> {
  // TODO: color
  pub energy: usize,
  pub genes_x: [bool; G],
  pub genes_y: [bool; G],
  pub position: usize,
}

pub struct Evolve<const G: usize, const L: usize> {
  // TODO: animatedComponent
  // TODO: bounds Rectangle
  pub bugs: RefCell<Vec<Bug<G>>>,
  pub bugs_alive: usize,
  // TODO: droughtButton
  // TODO: edenCheckBox
  // TODO: random
  pub flora_growth_rate: usize,
  pub flora_present: [bool; L],
  // TODO: growthRateSpinnerNumberModel
  // TODO: resetButton
  pub time: usize,
}
