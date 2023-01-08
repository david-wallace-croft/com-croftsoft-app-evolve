// =============================================================================
//! - Bug model for CroftSoft Evolve
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

use crate::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, FLORA_ENERGY,
  GENES_MAX, MAX_ENERGY, MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::engine::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

use super::flora::Flora;
use super::world::World;

#[derive(Debug)]
pub struct Bug {
  pub energy: usize,
  pub genes_x: [bool; GENES_MAX],
  pub genes_y: [bool; GENES_MAX],
  pub position: usize,
  pub species: Species,
}

#[derive(Debug)]
pub enum Species {
  Cruiser,
  Normal,
  Twirlie,
}

impl Bug {
  fn classify_species(&mut self) {
    let mut x_sum: isize = 0;
    let mut y_sum: isize = 0;
    for i in 0..GENES_MAX {
      if self.genes_x[i] {
        x_sum += 1;
      } else {
        x_sum -= 1;
      }
      if self.genes_y[i] {
        y_sum += 1;
      } else {
        y_sum -= 1;
      }
    }
    let unscaled_speed: f64 =
      ((x_sum as f64).powi(2) + (y_sum as f64).powi(2)).sqrt();
    let scaling_factor: f64 = (2.0 * ((GENES_MAX as f64).powi(2))).sqrt();
    let speed: f64 = unscaled_speed / scaling_factor;
    if speed >= 0.70 {
      self.species = Species::Cruiser;
    } else if speed <= 0.30 {
      self.species = Species::Twirlie;
    }
  }

  fn give_birth(&mut self) -> Self {
    self.energy = self.energy.saturating_sub(BIRTH_ENERGY_COST);
    let mut baby_bug = Bug::new(self.position);
    for index in 0..GENES_MAX {
      baby_bug.genes_x[index] = self.genes_x[index];
      baby_bug.genes_y[index] = self.genes_y[index];
    }
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let roll: usize = thread_rng.gen_range(0..10);
    if roll == 0 {
      let mutant_gene_index: usize = thread_rng.gen_range(0..GENES_MAX);
      if rand::random() {
        baby_bug.genes_x[mutant_gene_index] = !self.genes_x[mutant_gene_index];
      } else {
        baby_bug.genes_y[mutant_gene_index] = !self.genes_y[mutant_gene_index];
      }
    }
    baby_bug.classify_species();
    baby_bug
  }

  pub fn new(position: usize) -> Self {
    let color = Species::Normal;
    let energy: usize = BABY_ENERGY;
    let mut genes_x: [bool; GENES_MAX] = [false; GENES_MAX];
    let mut genes_y: [bool; GENES_MAX] = [false; GENES_MAX];
    for index in 0..GENES_MAX {
      genes_x[index] = rand::random();
      genes_y[index] = rand::random();
    }
    let mut bug = Bug {
      species: color,
      energy,
      genes_x,
      genes_y,
      position,
    };
    bug.classify_species();
    bug
  }

  pub fn update(
    &mut self,
    bugs_length: usize,
    new_bugs: &mut Vec<Bug>,
    world: &mut World,
  ) {
    let bug_position: usize = self.position;
    let flora: &mut Flora = &mut world.flora;
    if flora.flora_present[bug_position] {
      flora.flora_present[bug_position] = false;
      self.energy = self.energy.saturating_add(FLORA_ENERGY);
      if self.energy > MAX_ENERGY {
        self.energy = MAX_ENERGY;
      }
    }
    if self.energy >= BIRTH_ENERGY && bugs_length + new_bugs.len() < BUGS_MAX {
      let new_bug = self.give_birth();
      new_bugs.push(new_bug);
    }
    let mut x = to_x_from_index(bug_position);
    let mut y = to_y_from_index(bug_position);
    if rand::random() {
      if self.genes_x[world.time] {
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
      if self.genes_y[world.time] {
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
    self.position = to_index_from_xy(x, y);
    self.energy = self.energy.saturating_sub(MOVE_COST);
  }
}
