// =============================================================================
//! - Flora Painter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-27
//! - Since: 2022-12-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{PAINT_OFFSET, PAINT_SCALE};
use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::engine::traits::CanvasPainter;
use crate::models::flora::Flora;
use core::cell::RefCell;
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
    let flora_height = PAINT_SCALE * scale_y;
    let flora_width = PAINT_SCALE * scale_x;
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
    self.flora.borrow().flora_present.iter().enumerate().for_each(
      |(index, location)| {
        if *location {
          let x: f64 = to_x_from_index(index) as f64;
          let y: f64 = to_y_from_index(index) as f64;
          let corner_x = self.scale_x * (x + PAINT_OFFSET);
          let corner_y = self.scale_y * (y + PAINT_OFFSET);
          context.fill_rect(
            corner_x,
            corner_y,
            self.flora_width,
            self.flora_height,
          );
        }
      },
    );
  }
}
