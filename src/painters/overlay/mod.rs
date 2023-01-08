// =============================================================================
//! - OverlayPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
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

use core::cell::RefCell;

use crate::constants::GENES_MAX;
use crate::engine::traits::CanvasPainter;
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  fill_style: JsValue,
}

impl OverlayPainter {
  fn make_genes_average_string(&self) -> String {
    let mut gene_x_string = String::from("X:");
    let mut gene_y_string = String::from("Y:");
    let mut bugs_alive: usize = 0;
    for bug in self.fauna.borrow().bugs.iter() {
      if bug.energy > 0 {
        bugs_alive += 1;
      }
    }
    for i in 0..GENES_MAX {
      let mut x_sum: usize = 0;
      let mut y_sum: usize = 0;
      for bug in self.fauna.borrow().bugs.iter() {
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
    result.push(' ');
    result.push_str(&gene_y_string);
    result
  }

  fn make_status_string(&self) -> String {
    let genes_average_string = self.make_genes_average_string();
    let bugs_alive = self.fauna.borrow().bugs.iter().fold(0, |count, bug| {
      if bug.energy > 0 {
        count + 1
      } else {
        count
      }
    });
    let time = self.clock.borrow().time;
    format!(
      "Average Movement Genes {} Time:{} Alive:{}",
      genes_average_string, time, bugs_alive,
    )
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    fauna: Rc<RefCell<Fauna>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      clock,
      fauna,
      fill_style,
    }
  }
}

impl CanvasPainter for OverlayPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    let status_string = self.make_status_string();
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    context.fill_text(&status_string, 4.0, 17.0).unwrap();
  }
}
