// =============================================================================
//! - WorldPainter for CroftSoft Evolve
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

use crate::models::bug::enums::Species;
use crate::models::bug::structures::Bug;
use crate::models::world::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
  EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
  MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::World;
use crate::painters::background::BackgroundPainter;
use crate::painters::bugs::BugsPainter;
use crate::painters::flora::FloraPainter;
use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d};

pub struct WorldPainter<'a, 'b, const G: usize> {
  pub background_painter: BackgroundPainter<'a>,
  pub bugs_painter: BugsPainter<'a, 'b, G>,
  pub flora_painter: FloraPainter<'a, 'b, G>,
}

impl<'a, 'b, const G: usize> WorldPainter<'a, 'b, G> {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
    context: &'a CanvasRenderingContext2d,
    world: &'b World<G>,
  ) -> Self {
    let background_painter =
      BackgroundPainter::new(canvas_height, canvas_width, context);
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let bugs_painter = BugsPainter::new(context, scale_x, scale_y, world);
    let flora_painter = FloraPainter::new(context, scale_x, scale_y, world);
    Self {
      background_painter,
      bugs_painter,
      flora_painter,
    }
  }

  pub fn paint(&self) {
    self.background_painter.paint();
    self.flora_painter.paint();
    self.bugs_painter.paint();
  }
}
