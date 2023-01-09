// =============================================================================
//! - FloraPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-08
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

use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::engine::traits::CanvasPainter;
use crate::models::flora::Flora;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct FloraPainter {
  fill_style: JsValue,
  flora: Rc<RefCell<Flora>>,
  flora_height: f64,
  flora_width: f64,
  scale_x: f64,
  scale_y: f64,
}

impl FloraPainter {
  pub fn new(
    flora: Rc<RefCell<Flora>>,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let fill_style = JsValue::from_str("green");
    let flora_height = scale_y / 2.0;
    let flora_width = scale_x / 2.0;
    Self {
      fill_style,
      flora_height,
      flora_width,
      flora,
      scale_x,
      scale_y,
    }
  }
}

impl CanvasPainter for FloraPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    context.set_fill_style(&self.fill_style);
    let flora: Ref<Flora> = self.flora.borrow();
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      if flora.flora_present[index] {
        // TODO: replace with PlotLib.xy()
        let x: f64 = to_x_from_index(index) as f64;
        let y: f64 = to_y_from_index(index) as f64;
        let corner_x = self.scale_x * (x + 0.5);
        let corner_y = self.scale_y * (y + 0.5);
        context.fill_rect(
          corner_x,
          corner_y,
          self.flora_width,
          self.flora_height,
        );
      }
    }
  }
}
