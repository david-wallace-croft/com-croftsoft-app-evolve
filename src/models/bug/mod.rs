// =============================================================================
//! - Bug model
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

use crate::constants::{
  BABY_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, GENES_MAX, SPACE_HEIGHT,
  SPACE_WIDTH,
};

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Bug<const G: usize> {
  pub energy: usize,
  pub genes_x: [bool; G],
  pub genes_y: [bool; G],
  pub position: usize,
  pub species: Species,
}

#[derive(Debug)]
pub enum Species {
  Cruiser, // red
  Normal,  // magenta
  Twirler, // blue
}

impl<const G: usize> Bug<G> {
  pub fn give_birth(&mut self) -> Self {
    self.energy = self.energy.saturating_sub(BIRTH_ENERGY_COST);
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

  pub fn new(position: usize) -> Self {
    let color = Species::Normal;
    let energy: usize = BABY_ENERGY;
    let mut genes_x: [bool; G] = [false; G];
    let mut genes_y: [bool; G] = [false; G];
    for index in 0..G {
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
    bug.update_species();
    bug
  }

  pub fn update_species(&mut self) {
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
