// =============================================================================
//! - OverlayPainter for CroftSoft Evolve
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

use crate::models::world::World;

pub struct OverlayPainter<const G: usize> {
  pub fill_style: JsValue,
}

impl<const G: usize> OverlayPainter<G> {
  fn make_genes_average_string(
    &self,
    world: &World<G>,
  ) -> String {
    let mut gene_x_string = String::from("X:  ");
    let mut gene_y_string = String::from("Y:  ");
    let mut bugs_alive: usize = 0;
    for bug in world.bugs.iter() {
      if bug.energy > 0 {
        bugs_alive += 1;
      }
    }
    for i in 0..G {
      let mut x_sum: usize = 0;
      let mut y_sum: usize = 0;
      for bug in world.bugs.iter() {
        if bug.energy > 0 {
          if bug.genes_x[i] {
            x_sum += 1;
          }
          if bug.genes_y[i] {
            y_sum += 1;
          }
        }
      }
      if x_sum as f64 / bugs_alive as f64 >= 0.5 {
        gene_x_string.push('1');
      } else {
        gene_x_string.push('0');
      }
      if y_sum as f64 / bugs_alive as f64 >= 0.5 {
        gene_y_string.push('1');
      } else {
        gene_y_string.push('0');
      }
    }
    let mut result = String::from(&gene_x_string);
    result.push_str("    ");
    result.push_str(&gene_y_string);
    result
  }

  fn make_status_string(
    &self,
    world: &World<G>,
  ) -> String {
    let genes_average_string = self.make_genes_average_string(world);
    let bugs_alive = 0; // TODO
    let time = world.time;
    format!(
      "Alive: {}  Time: {}  Average Movement Genes {}",
      bugs_alive, time, genes_average_string
    )
  }

  pub fn paint(
    &self,
    context: &CanvasRenderingContext2d,
    world: &World<G>,
  ) {
    let status_string = self.make_status_string(world);
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 18px serif");
    context.fill_text(&status_string, 4.0, 18.0).unwrap();
  }
}

impl<const G: usize> Default for OverlayPainter<G> {
  fn default() -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
    }
  }
}
