// =============================================================================
//! - Associated functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-07
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
#![allow(unused_imports)]
#![allow(unused_variables)]

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

use crate::{
  constants::{
    BABY_ENERGY, BIRTH_ENERGY, FLORA_ENERGY, GENES_MAX, MAX_ENERGY, SPACE_WIDTH,
  },
  enums::Species,
  structures::{Bug, Evolve},
};

impl<const G: usize> Bug<G> {
  pub fn new(
    x: usize,
    y: usize,
  ) -> Self {
    let color = Species::Normal;
    let energy: usize = BABY_ENERGY;
    let mut genes_x: [bool; G] = [false; G];
    let mut genes_y: [bool; G] = [false; G];
    for index in 0..G {
      genes_x[index] = rand::random();
      genes_y[index] = rand::random();
    }
    let position: usize = Evolve::<G>::to_index_from_xy(x, y);
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
}

impl<const G: usize> Evolve<G> {
  pub fn create_status_string(
    bugs_alive: usize,
    time: usize,
    genes_average_string: &str,
  ) {
    // TODO
  }

  pub fn to_index_from_xy(
    x: usize,
    y: usize,
  ) -> usize {
    SPACE_WIDTH * y + x
  }

  pub fn to_x_from_index(index: usize) -> usize {
    index % SPACE_WIDTH
  }

  pub fn to_y_from_index(index: usize) -> usize {
    index / SPACE_WIDTH
  }
}
