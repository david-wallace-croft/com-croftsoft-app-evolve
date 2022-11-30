// =============================================================================
//! - Associated functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-29
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

impl<const G: usize, const L: usize> Evolve<G, L> {
  pub fn grow_flora(evolve: Evolve<G, L>) -> Evolve<G, L> {
    todo!();
  }

  pub fn move_bugs(mut evolve: Evolve<G, L>) -> Evolve<G, L> {
    evolve.time += 1;
    if evolve.time >= GENES_MAX {
      evolve.time = 0;
    }
    let Evolve {
      mut bugs,
      bugs_alive,
      flora_growth_rate,
      flora_present,
      time,
    } = evolve;
    for bug in bugs.iter_mut() {
      if bug.energy > 0 {
        if evolve.flora_present[bug.position] {
          bug.energy += FLORA_ENERGY;
          if bug.energy > MAX_ENERGY {
            bug.energy = MAX_ENERGY;
          }
        }
        if bug.energy >= BIRTH_ENERGY {
          Evolve::<G, L>::give_birth(bug);
        }
        // TODO
      }
    }
    Evolve {
      bugs,
      bugs_alive,
      flora_growth_rate,
      flora_present,
      time,
    }
  }

  pub fn repaint(evolve: Evolve<G, L>) -> Evolve<G, L> {
    todo!();
  }

  pub fn update(mut evolve: Evolve<G, L>) -> Evolve<G, L> {
    evolve = Evolve::move_bugs(evolve);
    evolve = Evolve::grow_flora(evolve);
    Evolve::repaint(evolve)
  }

  // private methods

  fn give_birth(
    // evolve: Evolve<G>,
    parent_bug: &Bug<G>,
  ) -> () {
    // TODO
    // evolve
  }
}
