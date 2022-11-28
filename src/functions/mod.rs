// =============================================================================
//! - Associated functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-27
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

use crate::{
  constants::{BIRTH_ENERGY, FLORA_ENERGY, GENES_MAX, MAX_ENERGY},
  structures::{Bug, Evolve},
};

impl<const G: usize> Evolve<G> {
  pub fn grow_flora(evolve: Evolve<G>) -> Evolve<G> {
    todo!();
  }

  pub fn move_bugs(mut evolve: Evolve<G>) -> Evolve<G> {
    evolve.time += 1;
    if evolve.time >= GENES_MAX {
      evolve.time = 0;
    }
    for bug in &mut evolve.bugs {
      let Bug {
        energy,
        genes_x,
        genes_y,
        x,
        y,
      } = bug;
      if bug.energy > 0 {
        if evolve.flora_present[*x][*y] {
          bug.energy += FLORA_ENERGY;
          if bug.energy > MAX_ENERGY {
            bug.energy = MAX_ENERGY;
          }
        }
        if bug.energy >= BIRTH_ENERGY {
          Evolve::give_birth(evolve, bug);
        }
        // TODO
      }
    }
    evolve
  }

  pub fn repaint(evolve: Evolve<G>) -> Evolve<G> {
    todo!();
  }

  pub fn update(evolve: Evolve<G>) -> Evolve<G> {
    let evolve: Evolve<G> = Evolve::move_bugs(evolve);
    let evolve: Evolve<G> = Evolve::grow_flora(evolve);
    let evolve: Evolve<G> = Evolve::repaint(evolve);
    evolve
  }

  // private methods

  fn give_birth(
    evolve: Evolve<G>,
    parent_bug: &Bug<G>,
  ) -> Evolve<G> {
    todo!();
  }
}
