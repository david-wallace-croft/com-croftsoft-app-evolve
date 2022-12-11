// =============================================================================
//! - BackgroundPainter for CroftSoft Evolve
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

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BackgroundPainter {
  pub canvas_height: f64,
  pub canvas_width: f64,
  pub fill_style: JsValue,
}

impl BackgroundPainter {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("black");
    Self {
      canvas_height,
      canvas_width,
      fill_style,
    }
  }

  pub fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    context.set_fill_style(&self.fill_style);
    context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
  }
}
