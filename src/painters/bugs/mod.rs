// =============================================================================
//! - BugsPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-05
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

use crate::functions::location::{to_x_from_index, to_y_from_index};
use crate::models::bug::Species;
use crate::models::world::World;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BugsPainter {
  pub bug_color_cruiser: JsValue,
  pub bug_color_normal: JsValue,
  pub bug_color_twirler: JsValue,
  pub bug_height: f64,
  pub bug_width: f64,
  pub fill_style: JsValue,
  pub scale_x: f64,
  pub scale_y: f64,
}

impl BugsPainter {
  pub fn new(
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let bug_color_cruiser = JsValue::from_str("red");
    let bug_color_normal = JsValue::from_str("magenta");
    let bug_color_twirler = JsValue::from_str("blue");
    let bug_height = scale_y / 2.0;
    let bug_width = scale_x / 2.0;
    let fill_style: JsValue = JsValue::from_str("black");
    Self {
      bug_color_cruiser,
      bug_color_normal,
      bug_color_twirler,
      bug_height,
      bug_width,
      fill_style,
      scale_x,
      scale_y,
    }
  }

  pub fn paint(
    &self,
    context: &CanvasRenderingContext2d,
    world: &World,
  ) {
    for bug in world.fauna.bugs.iter() {
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
