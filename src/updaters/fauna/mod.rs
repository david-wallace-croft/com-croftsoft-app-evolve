// =============================================================================
//! - Fauna Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-06
//! - Since: 2023-01-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{
  BABY_ENERGY, BIRTH_ENERGY, BIRTH_ENERGY_COST, BUGS_MAX, FLORA_ENERGY,
  GENES_MAX, LOCATION_COUNT, MAX_ENERGY, MOVE_COST, SPACE_HEIGHT, SPACE_WIDTH,
};

use crate::engine::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
use crate::models::bug::{Bug, Species};
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::rc::Rc;

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

pub trait FaunaUpdaterInputs {
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_reset_requested(&self) -> bool;
}

pub struct FaunaUpdater {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  flora: Rc<RefCell<Flora>>,
  inputs: Rc<RefCell<dyn FaunaUpdaterInputs>>,
}

impl FaunaUpdater {
  fn classify(bug: &Bug) -> Species {
    let mut x_sum: isize = 0;
    let mut y_sum: isize = 0;
    for i in 0..GENES_MAX {
      if bug.genes_x[i] {
        x_sum += 1;
      } else {
        x_sum -= 1;
      }
      if bug.genes_y[i] {
        y_sum += 1;
      } else {
        y_sum -= 1;
      }
    }
    let unscaled_speed: f64 =
      ((x_sum as f64).powi(2) + (y_sum as f64).powi(2)).sqrt();
    let scaling_factor: f64 = (2.0 * ((GENES_MAX as f64).powi(2))).sqrt();
    let speed: f64 = unscaled_speed / scaling_factor;
    if speed <= 0.30 {
      return Species::Twirlie;
    }
    if speed >= 0.70 {
      return Species::Cruiser;
    }
    Species::Normal
  }

  fn make_bug(position: usize) -> Bug {
    let species = Species::Normal;
    let energy: usize = BABY_ENERGY;
    let mut genes_x: [bool; GENES_MAX] = [false; GENES_MAX];
    let mut genes_y: [bool; GENES_MAX] = [false; GENES_MAX];
    for index in 0..GENES_MAX {
      genes_x[index] = rand::random();
      genes_y[index] = rand::random();
    }
    let mut bug = Bug {
      energy,
      genes_x,
      genes_y,
      position,
      species,
    };
    bug.species = Self::classify(&bug);
    bug
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    fauna: Rc<RefCell<Fauna>>,
    flora: Rc<RefCell<Flora>>,
    inputs: Rc<RefCell<dyn FaunaUpdaterInputs>>,
  ) -> Self {
    Self {
      clock,
      fauna,
      flora,
      inputs,
    }
  }

  fn reset(&mut self) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    let mut fauna: RefMut<Fauna> = self.fauna.borrow_mut();
    fauna.bugs.clear();
    for _i in 0..BUGS_MAX {
      fauna.bugs.push(Self::make_bug(position));
    }
  }

  fn update_bug(
    bug: &mut Bug,
    bugs_length: usize,
    flora_present: &mut [bool; LOCATION_COUNT],
    new_bugs: &mut Vec<Bug>,
    time: usize,
  ) {
    Self::update_bug_graze(bug, flora_present);
    Self::update_bug_spawn(bug, bugs_length, new_bugs);
    Self::update_bug_move(bug, time);
  }

  fn update_bug_graze(
    bug: &mut Bug,
    flora_present: &mut [bool; LOCATION_COUNT],
  ) {
    let bug_position: usize = bug.position;
    if flora_present[bug_position] {
      flora_present[bug_position] = false;
      bug.energy = bug.energy.saturating_add(FLORA_ENERGY);
      if bug.energy > MAX_ENERGY {
        bug.energy = MAX_ENERGY;
      }
    }
  }

  fn update_bug_move(
    bug: &mut Bug,
    time: usize,
  ) {
    let bug_position: usize = bug.position;
    let mut x = to_x_from_index(bug_position);
    let mut y = to_y_from_index(bug_position);
    if rand::random() {
      if bug.genes_x[time] {
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
      if bug.genes_y[time] {
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
    bug.position = to_index_from_xy(x, y);
    bug.energy = bug.energy.saturating_sub(MOVE_COST);
  }

  fn update_bug_spawn(
    bug: &mut Bug,
    bugs_length: usize,
    new_bugs: &mut Vec<Bug>,
  ) {
    if bug.energy < BIRTH_ENERGY || bugs_length + new_bugs.len() >= BUGS_MAX {
      return;
    }
    bug.energy = bug.energy.saturating_sub(BIRTH_ENERGY_COST);
    let mut baby_bug = Bug {
      energy: BABY_ENERGY,
      genes_x: bug.genes_x,
      genes_y: bug.genes_y,
      position: bug.position,
      species: bug.species,
    };
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let roll: usize = thread_rng.gen_range(0..10);
    if roll == 0 {
      let mutant_gene_index: usize = thread_rng.gen_range(0..GENES_MAX);
      if rand::random() {
        baby_bug.genes_x[mutant_gene_index] = !bug.genes_x[mutant_gene_index];
      } else {
        baby_bug.genes_y[mutant_gene_index] = !bug.genes_y[mutant_gene_index];
      }
    }
    baby_bug.species = Self::classify(&baby_bug);
    new_bugs.push(baby_bug);
  }
}

impl Updater for FaunaUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_reset_requested() {
      self.reset();
      return;
    }
    let mut new_bugs = Vec::<Bug>::new();
    let bugs_length = self.fauna.borrow().bugs.len();
    if bugs_length < BUGS_MAX {
      if let Some(position_index) = self.inputs.borrow().get_bug_requested() {
        new_bugs.push(Self::make_bug(position_index));
      }
    }
    for bug in self.fauna.borrow_mut().bugs.iter_mut() {
      Self::update_bug(
        bug,
        bugs_length,
        &mut self.flora.borrow_mut().flora_present,
        &mut new_bugs,
        self.clock.borrow().time,
      );
    }
    let mut fauna: RefMut<Fauna> = self.fauna.borrow_mut();
    fauna.bugs.retain(|bug| bug.energy > 0);
    fauna.bugs.append(&mut new_bugs);
  }
}
