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
use crate::models::world::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
  EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
  MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::models::world::structures::Evolve;
use crate::views::structures::View;

impl<'a, const G: usize> View<'a, G> {
  pub fn paint(&self) {
    self.context.set_fill_style(&JsValue::from_str("black"));
    self.context.fill_rect(0.0, 0.0, self.width, self.height);
    self.plot_flora();
    self.plot_bugs();
  }

  pub fn plot_bugs(&self) {
    let scale_x = self.width / SPACE_WIDTH as f64;
    let scale_y = self.height / SPACE_HEIGHT as f64;
    let width = scale_x / 2.0;
    let height = scale_y / 2.0;
    for bug in self.evolve.bugs.iter() {
      // let bug_str = format!("{:?}", bug);
      // console::log_1(&JsValue::from_str(&bug_str));
      if bug.energy == 0 {
        continue;
      }
      let bug_color = match bug.species {
        Species::Cruiser => "red",
        Species::Normal => "magenta",
        Species::Twirler => "blue",
      };
      self.context.set_fill_style(&JsValue::from_str(bug_color));
      let index = bug.position;
      let x: f64 = Evolve::<8>::to_x_from_index(index) as f64;
      let y: f64 = Evolve::<8>::to_y_from_index(index) as f64;
      let corner_x = scale_x * (x + 0.5);
      let corner_y = scale_y * (y + 0.5);
      self.context.fill_rect(corner_x, corner_y, width, height);
    }
  }

  pub fn plot_flora(&self) {
    let scale_x = self.width / SPACE_WIDTH as f64;
    let scale_y = self.height / SPACE_HEIGHT as f64;
    let width = scale_x / 2.0;
    let height = scale_y / 2.0;
    self.context.set_fill_style(&JsValue::from_str("green"));
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      if self.evolve.flora_present[index] {
        // TODO: replace with PlotLib.xy()
        let x: f64 = Evolve::<8>::to_x_from_index(index) as f64;
        let y: f64 = Evolve::<8>::to_y_from_index(index) as f64;
        let corner_x = scale_x * (x + 0.5);
        let corner_y = scale_y * (y + 0.5);
        self.context.fill_rect(corner_x, corner_y, width, height);
      }
    }
  }
}
