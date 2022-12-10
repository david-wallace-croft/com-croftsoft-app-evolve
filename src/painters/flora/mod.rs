// =============================================================================
//! - FloraPainter for CroftSoft Evolve
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

use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::bug::Species;
use crate::models::world::structures::World;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct FloraPainter<'a, const G: usize> {
  pub context: &'a CanvasRenderingContext2d,
  pub fill_style: JsValue,
  pub flora_height: f64,
  pub flora_width: f64,
  pub scale_x: f64,
  pub scale_y: f64,
}

impl<'a, const G: usize> FloraPainter<'a, G> {
  pub fn new(
    context: &'a CanvasRenderingContext2d,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let fill_style = JsValue::from_str("green");
    let flora_height = scale_y / 2.0;
    let flora_width = scale_x / 2.0;
    Self {
      context,
      fill_style,
      flora_height,
      flora_width,
      scale_x,
      scale_y,
    }
  }

  pub fn paint(
    &self,
    world: &World<G>,
  ) {
    self.context.set_fill_style(&self.fill_style);
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      if world.flora_present[index] {
        // TODO: replace with PlotLib.xy()
        let x: f64 = World::<G>::to_x_from_index(index) as f64;
        let y: f64 = World::<G>::to_y_from_index(index) as f64;
        let corner_x = self.scale_x * (x + 0.5);
        let corner_y = self.scale_y * (y + 0.5);
        self.context.fill_rect(
          corner_x,
          corner_y,
          self.flora_width,
          self.flora_height,
        );
      }
    }
  }
}
