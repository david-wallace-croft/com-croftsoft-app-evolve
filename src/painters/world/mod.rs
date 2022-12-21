// =============================================================================
//! - WorldPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
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

use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::world::World;
use crate::painters::background::BackgroundPainter;
use crate::painters::bugs::BugsPainter;
use crate::painters::flora::FloraPainter;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement,
};

use super::overlay::OverlayPainter;

pub struct WorldPainter {
  pub background_painter: BackgroundPainter,
  pub bugs_painter: BugsPainter,
  pub context: CanvasRenderingContext2d,
  pub flora_painter: FloraPainter,
  pub overlay_painter: OverlayPainter,
}

impl WorldPainter {
  pub fn new(canvas_element_id: &str) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let element: Element =
      document.get_element_by_id(canvas_element_id).unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let canvas_height: f64 = html_canvas_element.height() as f64;
    let canvas_width: f64 = html_canvas_element.width() as f64;
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
    world: &World,
  ) {
    self.background_painter.paint(&self.context);
    self.flora_painter.paint(&self.context, world);
    self.bugs_painter.paint(&self.context, world);
    self.overlay_painter.paint(&self.context, world);
  }
}
