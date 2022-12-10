// =============================================================================
//! - Methods for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-09
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
    BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
    EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
    MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
  },
  enums::Species,
  structures::{Bug, Evolve, View},
};

impl<const G: usize> Bug<G> {
  pub fn give_birth(&mut self) -> Self {
    self.energy -= BIRTH_ENERGY_COST;
    let mut baby_bug = Bug::new(self.position);
    for index in 0..GENES_MAX {
      baby_bug.genes_x[index] = self.genes_x[index];
      baby_bug.genes_y[index] = self.genes_y[index];
    }
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let mutant_gene_index: usize = thread_rng.gen_range(0..G);
    if rand::random() {
      baby_bug.genes_x[mutant_gene_index] = self.genes_x[mutant_gene_index];
    } else {
      baby_bug.genes_y[mutant_gene_index] = self.genes_y[mutant_gene_index];
    }
    baby_bug.update_species();
    baby_bug
  }

  pub fn update_species(&mut self) {
    // TODO: change color to classfication or species
    let mut x_count = 0;
    let mut y_count = 0;
    for i in 0..G {
      if self.genes_x[i] {
        x_count += 1;
      }
      if self.genes_y[i] {
        y_count += 1;
      }
    }
    let mut species = Species::Normal;
    if x_count == G / 2 && y_count == G / 2 {
      species = Species::Twirler;
    } else if x_count == 0 || x_count == G || y_count == 0 || y_count == G {
      species = Species::Cruiser;
    }
    self.species = species;
  }
}

impl<const G: usize> Evolve<G> {
  pub fn create_new_bug(
    &mut self,
    position: usize,
  ) {
    let bug = Bug::new(position);
    self.bugs.push(bug);
    // let bug_str = format!("{:?}", bug);
    // console::log_1(&JsValue::from_str(&bug_str));
  }

  // TODO: Is this method still needed?
  pub fn create_new_bug_if_dead(
    &mut self,
    position: usize,
  ) {
    if self.bugs.len() >= BUGS_MAX {
      return;
    }
    self.create_new_bug(position);
  }

  pub fn genes_average_string(&self) -> String {
    let mut gene_x_string = String::from("X:  ");
    let mut gene_y_string = String::from("Y:  ");
    let mut bugs_alive: usize = 0;
    for bug in self.bugs.iter() {
      if bug.energy > 0 {
        bugs_alive += 1;
      }
    }
    for i in 0..G {
      let mut x_sum: usize = 0;
      let mut y_sum: usize = 0;
      for bug in self.bugs.iter() {
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

  pub fn init(&mut self) {
    // TODO
  }

  pub fn move_bugs(&mut self) {
    self.time = self.time.saturating_add(1);
    if self.time >= GENES_MAX {
      self.time = 0;
    }
    let mut new_bugs = Vec::<Bug<G>>::new();
    let bugs_length = self.bugs.len();
    for bug in self.bugs.iter_mut() {
      if bug.energy == 0 {
        continue;
      }
      let mut x = Evolve::<G>::to_x_from_index(bug.position);
      let mut y = Evolve::<G>::to_y_from_index(bug.position);
      if self.flora_present[bug.position] {
        bug.energy = bug.energy.saturating_add(FLORA_ENERGY);
        if bug.energy > MAX_ENERGY {
          bug.energy = MAX_ENERGY;
        }
      }
      if bug.energy >= BIRTH_ENERGY && bugs_length + new_bugs.len() < BUGS_MAX {
        let new_bug = bug.give_birth();
        new_bugs.push(new_bug);
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
      bug.energy = bug.energy.saturating_sub(MOVE_COST);
    }
    let mut dead_bug_indices = Vec::<usize>::new();
    for (index, bug) in self.bugs.iter_mut().enumerate() {
      if bug.energy == 0 {
        dead_bug_indices.push(index);
      }
    }
    for dead_bug_index in dead_bug_indices {
      self.bugs.remove(dead_bug_index);
    }
    self.bugs.append(&mut new_bugs);
  }

  pub fn reset(&mut self) {
    let position: usize =
      Evolve::<G>::to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    for index in 0..BUGS_MAX {
      self.create_new_bug(position);
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
