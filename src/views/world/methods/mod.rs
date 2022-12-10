// =============================================================================
//! - Methods for CroftSoft Evolve
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

#![allow(dead_code)]
#![allow(unused_variables)]

use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::models::bug::enums::Species;
use crate::models::bug::structures::Bug;
use crate::models::world::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
  EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
  MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::Evolve;
use crate::views::world::structures::WorldPainter;

impl<'a, const G: usize> WorldPainter<'a, G> {
  pub fn paint(&self) {
    self.paint_background();
    self.paint_flora();
    self.paint_bugs();
  }

  fn paint_background(&self) {
    self.context.set_fill_style(&JsValue::from_str("black"));
    self.context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
  }

  fn paint_bugs(&self) {
    for bug in self.evolve.bugs.iter() {
      let bug_color = match bug.species {
        Species::Cruiser => "red",
        Species::Normal => "magenta",
        Species::Twirler => "blue",
      };
      self.context.set_fill_style(&JsValue::from_str(bug_color));
      let index = bug.position;
      let x: f64 = Evolve::<8>::to_x_from_index(index) as f64;
      let y: f64 = Evolve::<8>::to_y_from_index(index) as f64;
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

  fn paint_flora(&self) {
    let width = self.scale_x / 2.0;
    let height = self.scale_y / 2.0;
    self.context.set_fill_style(&JsValue::from_str("green"));
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      if self.evolve.flora_present[index] {
        // TODO: replace with PlotLib.xy()
        let x: f64 = Evolve::<8>::to_x_from_index(index) as f64;
        let y: f64 = Evolve::<8>::to_y_from_index(index) as f64;
        let corner_x = self.scale_x * (x + 0.5);
        let corner_y = self.scale_y * (y + 0.5);
        self.context.fill_rect(corner_x, corner_y, width, height);
      }
    }
  }
}
