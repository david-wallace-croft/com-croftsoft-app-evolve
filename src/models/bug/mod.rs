// =============================================================================
//! - Bug model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-31
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

use crate::constants::{BABY_ENERGY, BIRTH_ENERGY_COST, GENES_MAX};

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

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
  pub fn give_birth(&mut self) -> Self {
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

  pub fn classify_species(&mut self) {
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
}
