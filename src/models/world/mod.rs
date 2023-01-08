// =============================================================================
//! - Structures for the World model
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

use super::fauna::Fauna;
use super::flora::Flora;
use crate::constants::GENES_MAX;
use crate::engine::input::Input;
use crate::engine::traits::{Model, WorldUpdater};
use core::mem::take;

pub struct World {
  fauna_option: Option<Fauna>,
  pub flora: Flora,
  pub time: usize,
}

impl World {
  // make a Trait out of these methods
  pub fn get_fauna_as_ref(&self) -> &Fauna {
    self.fauna_option.as_ref().unwrap()
  }

  // pub fn get_fauna_as_mut(&mut self) -> &mut Fauna {
  //   self.fauna_option.as_mut().unwrap()
  // }
}

impl Default for World {
  fn default() -> Self {
    Self {
      fauna_option: Some(Fauna::default()),
      flora: Flora::default(),
      time: 0,
    }
  }
}

impl Model for World {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested || self.time >= GENES_MAX - 1 {
      self.time = 0;
    } else {
      self.time += 1;
    }
    self.flora.update(input);
    // TODO: convert updater tree to vector so grandchildren can mutate parent
    let mut fauna_option = take(&mut self.fauna_option);
    let fauna: &mut Fauna = fauna_option.as_mut().unwrap();
    fauna.update_world(input, self);
    self.fauna_option = fauna_option;
  }
}
