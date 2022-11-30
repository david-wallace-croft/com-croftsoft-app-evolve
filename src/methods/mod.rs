// =============================================================================
//! - Methods for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-30
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

use crate::{
  constants::{BIRTH_ENERGY, FLORA_ENERGY, GENES_MAX, MAX_ENERGY},
  structures::{Bug, Evolve},
};

impl<const G: usize, const L: usize> Evolve<G, L> {
  pub fn give_birth(
    &self,
    parent_bug: &mut Bug<G>,
  ) {
  }

  pub fn grow_flora1(&mut self) {}

  pub fn move_bugs1(&mut self) {
    self.time += 1;
    if self.time >= GENES_MAX {
      self.time = 0;
    }
    let mut bugs = self.bugs.borrow_mut();
    for bug in bugs.iter_mut() {
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
        // TODO
      }
    }
  }

  pub fn repaint1(&mut self) {}

  pub fn update1(&mut self) {
    self.move_bugs1();
    self.grow_flora1();
    self.repaint1();
  }
}
