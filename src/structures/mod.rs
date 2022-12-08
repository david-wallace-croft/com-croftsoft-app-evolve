// =============================================================================
//! - Structures for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-07
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

use web_sys::CanvasRenderingContext2d;

use crate::{
  constants::{BUGS_MAX, SPACE_HEIGHT, SPACE_WIDTH},
  enums::Species,
};

#[derive(Debug)]
pub struct Bug<const G: usize> {
  pub energy: usize,
  pub genes_x: [bool; G],
  pub genes_y: [bool; G],
  pub position: usize,
  pub species: Species,
}

pub struct Evolve<const G: usize> {
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

pub struct View<'a, const G: usize> {
  pub context: CanvasRenderingContext2d,
  pub evolve: &'a Evolve<G>,
  pub height: f64,
  pub width: f64,
}
