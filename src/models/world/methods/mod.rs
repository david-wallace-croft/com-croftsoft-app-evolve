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

use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::models::bug::Bug;
use crate::models::bug::Species;
use crate::models::world::{
  constants::{
    BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, EDEN_X0, EDEN_X1,
    EDEN_Y0, EDEN_Y1, FLORA_ENERGY, GENES_MAX, INIT_GROWTH_RATE, MAX_ENERGY,
    MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
  },
  structures::World,
};

impl<const G: usize> World<G> {
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
    for _i in 0..self.flora_growth_rate {
      // Randomly position food flora
      let x: usize = thread_rng.gen_range(0..SPACE_WIDTH);
      let y: usize = thread_rng.gen_range(0..SPACE_HEIGHT);
      let index: usize = World::<G>::to_index_from_xy(x, y);
      self.flora_present[index] = true;
    }
    // Replenishing the Garden of Eden
    if self.eden_check_box {
      for x in EDEN_X0..=EDEN_X1 {
        for y in EDEN_Y0..=EDEN_Y1 {
          let index: usize = World::<G>::to_index_from_xy(x, y);
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
      let mut x = World::<G>::to_x_from_index(bug.position);
      let mut y = World::<G>::to_y_from_index(bug.position);
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
      bug.position = World::<G>::to_index_from_xy(x, y);
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
      World::<G>::to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    for _i in 0..BUGS_MAX {
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
