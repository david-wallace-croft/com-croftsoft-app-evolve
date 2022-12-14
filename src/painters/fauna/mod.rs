// =============================================================================
//! - FaunaPainter for CroftSoft Evolve
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

use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::engine::traits::CanvasPainter;
use crate::models::bug::Species;
use crate::models::fauna::Fauna;
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
  fauna: Rc<RefCell<Fauna>>,
  scale_x: f64,
  scale_y: f64,
}

impl FaunaPainter {
  pub fn new(
    fauna: Rc<RefCell<Fauna>>,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let bug_color_cruiser = JsValue::from_str("red");
    let bug_color_normal = JsValue::from_str("magenta");
    let bug_color_twirler = JsValue::from_str("blue");
    let bug_height = scale_y / 2.0;
    let bug_width = scale_x / 2.0;
    Self {
      bug_color_cruiser,
      bug_color_normal,
      bug_color_twirler,
      bug_height,
      bug_width,
      fauna,
      scale_x,
      scale_y,
    }
  }
}

impl CanvasPainter for FaunaPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
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
      let corner_x = self.scale_x * (x + 0.5);
      let corner_y = self.scale_y * (y + 0.5);
      context.fill_rect(corner_x, corner_y, self.bug_width, self.bug_height);
    }
  }
}
