// =============================================================================
//! - Functions for CroftSoft Evolve
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
#![allow(unused_variables)]

use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d};

use crate::models::bug::enums::Species;
use crate::models::bug::structures::Bug;
use crate::models::world::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
  EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
  MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::World;
use crate::views::world::structures::WorldPainter;

impl<'a, const G: usize> WorldPainter<'a, G> {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
    context: CanvasRenderingContext2d,
    evolve: &'a World<G>,
  ) -> Self {
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let bug_height = scale_y / 2.0;
    let bug_width = scale_x / 2.0;
    Self {
      bug_height,
      bug_width,
      canvas_height,
      canvas_width,
      context,
      evolve,
      scale_x,
      scale_y,
    }
  }
}
