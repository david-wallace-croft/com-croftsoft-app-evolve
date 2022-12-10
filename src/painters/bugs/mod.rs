// =============================================================================
//! - BugsPainter for CroftSoft Evolve
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

use crate::models::bug::enums::Species;
use crate::models::world::structures::World;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BugsPainter<'a, 'b, const G: usize> {
  pub bug_color_cruiser: JsValue,
  pub bug_color_normal: JsValue,
  pub bug_color_twirler: JsValue,
  pub bug_height: f64,
  pub bug_width: f64,
  pub context: &'a CanvasRenderingContext2d,
  pub fill_style: JsValue,
  pub scale_x: f64,
  pub scale_y: f64,
  pub world: &'b World<G>,
}

impl<'a, 'b, const G: usize> BugsPainter<'a, 'b, G> {
  pub fn new(
    context: &'a CanvasRenderingContext2d,
    scale_x: f64,
    scale_y: f64,
    world: &'b World<G>,
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
      context,
      fill_style,
      scale_x,
      scale_y,
      world,
    }
  }

  pub fn paint(&self) {
    for bug in self.world.bugs.iter() {
      let bug_color = match bug.species {
        Species::Cruiser => &self.bug_color_cruiser,
        Species::Normal => &self.bug_color_normal,
        Species::Twirler => &self.bug_color_twirler,
      };
      self.context.set_fill_style(bug_color);
      let index = bug.position;
      let x: f64 = World::<G>::to_x_from_index(index) as f64;
      let y: f64 = World::<G>::to_y_from_index(index) as f64;
      let corner_x = self.scale_x * (x + 0.5);
      let corner_y = self.scale_y * (y + 0.5);
      self.context.fill_rect(
        corner_x,
        corner_y,
        self.bug_width,
        self.bug_height,
      );
    }
  }
}