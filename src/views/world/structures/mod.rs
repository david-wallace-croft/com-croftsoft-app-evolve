// =============================================================================
//! - Structures for CroftSoft Evolve
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

#![allow(dead_code)]

use crate::models::world::structures::Evolve;
use web_sys::CanvasRenderingContext2d;

pub struct WorldPainter<'a, const G: usize> {
  pub bug_height: f64,
  pub bug_width: f64,
  pub canvas_height: f64,
  pub canvas_width: f64,
  pub context: CanvasRenderingContext2d,
  pub evolve: &'a Evolve<G>,
  pub scale_x: f64,
  pub scale_y: f64,
}