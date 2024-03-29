// =============================================================================
//! - Fauna Painter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-10
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{PAINT_OFFSET, PAINT_SCALE};
use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::models::bug::Species;
use crate::models::fauna::Fauna;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct FaunaPainter {
  bug_color_cruiser: JsValue,
  bug_color_normal: JsValue,
  bug_color_twirler: JsValue,
  bug_height: f64,
  bug_width: f64,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fauna: Rc<RefCell<Fauna>>,
  scale_x: f64,
  scale_y: f64,
}

impl FaunaPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    fauna: Rc<RefCell<Fauna>>,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let bug_color_cruiser = JsValue::from_str("red");
    let bug_color_normal = JsValue::from_str("magenta");
    let bug_color_twirler = JsValue::from_str("blue");
    let bug_height = (PAINT_SCALE * scale_y).trunc();
    let bug_width = (PAINT_SCALE * scale_x).trunc();
    Self {
      bug_color_cruiser,
      bug_color_normal,
      bug_color_twirler,
      bug_height,
      bug_width,
      context,
      fauna,
      scale_x,
      scale_y,
    }
  }
}

impl Painter for FaunaPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    for bug in self.fauna.borrow().bugs.iter() {
      let bug_color = match bug.species {
        Species::Cruiser => &self.bug_color_cruiser,
        Species::Normal => &self.bug_color_normal,
        Species::Twirlie => &self.bug_color_twirler,
      };
      context.set_fill_style(bug_color);
      let index = bug.position;
      let x: f64 = to_x_from_index(index) as f64;
      let y: f64 = to_y_from_index(index) as f64;
      let corner_x = (self.scale_x * (x + PAINT_OFFSET)).trunc();
      let corner_y = (self.scale_y * (y + PAINT_OFFSET)).trunc();
      context.fill_rect(corner_x, corner_y, self.bug_width, self.bug_height);
    }
  }
}
