// =============================================================================
//! - Flora model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-07
//! - Rust since: 2023-01-04
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
  EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, FLORA_GROWTH_RATE_INIT,
  FLORA_GROWTH_RATE_MAX, SPACE_HEIGHT, SPACE_WIDTH,
};
use crate::engine::functions::location::to_index_from_xy;
use crate::engine::input::Input;
use crate::engine::traits::Model;

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Flora {
  pub enabled_garden: bool,
  pub flora_growth_rate: usize,
  pub flora_present: [bool; SPACE_HEIGHT * SPACE_WIDTH],
}

impl Flora {
  fn reset(&mut self) {
    for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
      self.flora_present[index] = true;
    }
  }

  fn set_garden_values(
    &mut self,
    value: bool,
  ) {
    for x in EDEN_X0..=EDEN_X1 {
      for y in EDEN_Y0..=EDEN_Y1 {
        let index: usize = to_index_from_xy(x, y);
        self.flora_present[index] = value;
      }
    }
  }

  // TODO: move this
  fn update_garden(
    &mut self,
    input: &Input,
  ) {
    if let Some(enabled) = input.garden_change_requested {
      self.enabled_garden = enabled;
      if !self.enabled_garden {
        self.set_garden_values(false);
      }
    }
    if self.enabled_garden {
      self.set_garden_values(true);
    }
  }
}

impl Default for Flora {
  fn default() -> Self {
    Self {
      enabled_garden: true,
      flora_growth_rate: FLORA_GROWTH_RATE_INIT,
      flora_present: [false; SPACE_HEIGHT * SPACE_WIDTH],
    }
  }
}

impl Model for Flora {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested {
      self.reset();
      return;
    }
    if let Some(flora_growth_rate) = input.flora_growth_rate_change_requested {
      if flora_growth_rate < FLORA_GROWTH_RATE_MAX {
        self.flora_growth_rate = flora_growth_rate;
      } else {
        self.flora_growth_rate = FLORA_GROWTH_RATE_MAX;
      }
    }
    if input.blight_requested {
      for i in 0..SPACE_HEIGHT * SPACE_WIDTH {
        self.flora_present[i] = false;
      }
    } else {
      let mut thread_rng: ThreadRng = rand::thread_rng();
      for _i in 0..self.flora_growth_rate {
        // Randomly position food flora
        let index: usize = thread_rng.gen_range(0..SPACE_HEIGHT * SPACE_WIDTH);
        self.flora_present[index] = true;
      }
    }
    self.update_garden(input);
  }
}
