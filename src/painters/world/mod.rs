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

use crate::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
  EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
  MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::bug::Bug;
use crate::models::bug::Species;
use crate::models::world::World;
use crate::painters::background::BackgroundPainter;
use crate::painters::bugs::BugsPainter;
use crate::painters::flora::FloraPainter;
use js_sys::Object;
use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
  console, window, CanvasRenderingContext2d, Document, Element,
  HtmlCanvasElement,
};

use super::overlay::OverlayPainter;

pub struct WorldPainter<const G: usize> {
  pub background_painter: BackgroundPainter,
  pub bugs_painter: BugsPainter<G>,
  pub context: CanvasRenderingContext2d,
  pub flora_painter: FloraPainter<G>,
  pub overlay_painter: OverlayPainter<G>,
}

impl<const G: usize> WorldPainter<G> {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
  ) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let element: Element = document.get_element_by_id("canvas").unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let background_painter =
      BackgroundPainter::new(canvas_height, canvas_width);
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let bugs_painter = BugsPainter::new(scale_x, scale_y);
    let flora_painter = FloraPainter::new(scale_x, scale_y);
    let overlay_painter = OverlayPainter::default();
    Self {
      background_painter,
      bugs_painter,
      context,
      flora_painter,
      overlay_painter,
    }
  }

  pub fn paint(
    &self,
    world: &World<G>,
  ) {
    self.background_painter.paint(&self.context);
    self.flora_painter.paint(&self.context, world);
    self.bugs_painter.paint(&self.context, world);
    self.overlay_painter.paint(&self.context, world);
  }
}
