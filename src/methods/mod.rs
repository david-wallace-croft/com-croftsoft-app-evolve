// =============================================================================
//! - Methods for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-06
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

use crate::{
  constants::{
    BABY_ENERGY, BIRTH_ENERGY, BUGS_MAX, EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1,
    FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY, MOVE_COST,
    SPACE_HEIGHT, SPACE_WIDTH,
  },
  enums::Color,
  structures::{Bug, Evolve, View},
};

impl<const G: usize> Evolve<G> {
  pub fn create_new_bug(
    &mut self,
    x: usize,
    y: usize,
    index: usize,
  ) {
    let mut bug = self.bugs.borrow_mut()[index];
    bug.alive = true;
    bug.energy = BABY_ENERGY;
    for gene_index in 0..G {
      bug.genes_x[gene_index] = rand::random();
      bug.genes_y[gene_index] = rand::random();
    }
    bug.position = Evolve::<G>::to_index_from_xy(x, y);
    self.set_bug_color(&mut bug);
    // let bug_str = format!("{:?}", bug);
    // console::log_1(&JsValue::from_str(&bug_str));
  }

  pub fn create_new_bug_if_dead(
    &mut self,
    x: usize,
    y: usize,
  ) {
    let index_option: Option<usize> = self.index_of_first_dead_bug();
    if let Some(index) = index_option {
      self.create_new_bug(x, y, index);
    }
  }

  pub fn genes_average_string(&self) -> String {
    let mut gene_x_string = String::from("X:  ");
    let mut gene_y_string = String::from("Y:  ");
    let mut bugs_alive: usize = 0;
    for bug in self.bugs.borrow().iter() {
      if bug.energy > 0 {
        bugs_alive += 1;
      }
    }
    for i in 0..G {
      let mut x_sum: usize = 0;
      let mut y_sum: usize = 0;
      for bug in self.bugs.borrow().iter() {
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

  pub fn give_birth(
    &self,
    parent_bug: &mut Bug<G>,
  ) {
    // TODO
    todo!();
  }

  pub fn grow_flora(&mut self) {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    for i in 0..self.flora_growth_rate {
      // Randomly position food flora
      let x: usize = thread_rng.gen_range(0..SPACE_WIDTH);
      let y: usize = thread_rng.gen_range(0..SPACE_HEIGHT);
      let index: usize = Evolve::<G>::to_index_from_xy(x, y);
      self.flora_present[index] = true;
    }
    // Replenishing the Garden of Eden
    if self.eden_check_box {
      for x in EDEN_X0..=EDEN_X1 {
        for y in EDEN_Y0..=EDEN_Y1 {
          let index: usize = Evolve::<G>::to_index_from_xy(x, y);
          self.flora_present[index] = true;
        }
      }
    }
  }

  pub fn index_of_first_dead_bug(&self) -> Option<usize> {
    for (index, bug) in self.bugs.borrow().iter().enumerate() {
      if bug.energy == 0 {
        return Some(index);
      }
    }
    None
  }

  pub fn init(&mut self) {
    // TODO
  }

  pub fn move_bugs(&mut self) {
    self.time += 1;
    if self.time >= GENES_MAX {
      self.time = 0;
    }

    for (index, bug) in self.bugs.borrow_mut().iter_mut().enumerate() {
      let mut x = Evolve::<G>::to_x_from_index(bug.position);
      let mut y = Evolve::<G>::to_y_from_index(bug.position);
      if bug.energy > 0 {
        if self.flora_present[bug.position] {
          bug.energy += FLORA_ENERGY;
          if bug.energy > MAX_ENERGY {
            bug.energy = MAX_ENERGY;
          }
        }
        if bug.energy >= BIRTH_ENERGY {
          self.give_birth(bug);
        }
        if rand::random() {
          if bug.genes_x[self.time] {
            if x < SPACE_WIDTH - 1 {
              x += 1;
            } else {
              x = 0;
            }
          } else if x > 0 {
            x -= 1;
          } else {
            x = SPACE_WIDTH - 1;
          }
        }
        if rand::random() {
          if bug.genes_y[self.time] {
            if y < SPACE_HEIGHT - 1 {
              y += 1;
            } else {
              y = 0;
            }
          } else if y > 0 {
            y -= 1;
          } else {
            y = SPACE_HEIGHT - 1;
          }
        }
        bug.position = Evolve::<G>::to_index_from_xy(x, y);
        bug.energy -= MOVE_COST;
      }
    }
  }

  pub fn reset(&mut self) {
    for index in 0..BUGS_MAX {
      self.create_new_bug(SPACE_WIDTH / 2, SPACE_HEIGHT / 2, index);
    }
    // for bug in self.bugs.borrow().iter() {
    //   let bug_str = format!("{:?}", bug);
    //   console::log_1(&JsValue::from_str(&bug_str));
    // }
    self.set_all_flora_present(true);
    self.eden_check_box = true; // TODO: event?
    self.growth_rate_spinner_number_model = INIT_GROWTH_RATE; // TODO: event?
  }

  pub fn set_all_flora_present(
    &mut self,
    flora_present: bool,
  ) {
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      self.flora_present[index] = flora_present;
    }
  }

  pub fn set_bug_color(
    &mut self,
    bug: &mut Bug<G>,
  ) {
    let mut x_count = 0;
    let mut y_count = 0;
    for i in 0..G {
      if bug.genes_x[i] {
        x_count += 1;
      }
      if bug.genes_y[i] {
        y_count += 1;
      }
    }
    let mut color = Color::Normal;
    if x_count == G / 2 && y_count == G / 2 {
      color = Color::Twirler;
    } else if x_count == 0 || x_count == G || y_count == 0 || y_count == G {
      color = Color::Cruiser;
    }
    bug.color = color;
  }

  pub fn update(&mut self) {
    self.move_bugs();
    self.grow_flora();
  }
}

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
    for bug in self.evolve.bugs.borrow().iter() {
      // let bug_str = format!("{:?}", bug);
      // console::log_1(&JsValue::from_str(&bug_str));
      if bug.energy == 0 {
        continue;
      }
      let bug_color = match bug.color {
        Color::Cruiser => "red",
        Color::Normal => "magenta",
        Color::Twirler => "blue",
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
