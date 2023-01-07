// =============================================================================
//! - Fauna model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-07
//! - Rust since: 2023-01-05
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

use super::bug::Bug;
use super::flora::Flora;
use crate::constants::{BUGS_MAX, SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::to_index_from_xy;
use crate::engine::input::Input;

#[derive(Default)]
pub struct Fauna {
  pub bugs: Vec<Bug>,
}

impl Fauna {
  fn reset(&mut self) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    self.bugs.clear();
    for _i in 0..BUGS_MAX {
      let bug = Bug::new(position);
      self.bugs.push(bug);
    }
  }

  pub fn update(
    &mut self,
    input: &Input,
    flora: &mut Flora,
    time: usize,
  ) {
    if input.reset_requested {
      self.reset();
      return;
    }
    let mut new_bugs = Vec::<Bug>::new();
    let bugs_length = self.bugs.len();
    if bugs_length < BUGS_MAX {
      if let Some(position_index) = input.bug_requested {
        let new_bug = Bug::new(position_index);
        new_bugs.push(new_bug);
      }
    }
    for bug in self.bugs.iter_mut() {
      bug.update(bugs_length, flora, &mut new_bugs, time);
    }
    self.bugs.retain(|bug| bug.energy > 0);
    self.bugs.append(&mut new_bugs);
  }
}
