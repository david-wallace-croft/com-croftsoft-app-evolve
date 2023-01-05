// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-04
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

use crate::constants::{BUGS_MAX, SPACE_HEIGHT, SPACE_WIDTH};
use crate::functions::location::to_index_from_xy;
use crate::models::bug::Bug;
use crate::traits::{InputReader, Model};

use super::flora::Flora;

#[derive(Default)]
pub struct World {
  pub bugs: Vec<Bug>,
  pub flora: Flora,
  pub time: usize,
}

impl World {
  fn reset(&mut self) {
    let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
    self.bugs.clear();
    for _i in 0..BUGS_MAX {
      let bug = Bug::new(position);
      // let bug_str = format!("{:?}", bug);
      // console::log_1(&JsValue::from_str(&bug_str));
      self.bugs.push(bug);
    }
  }
}

impl<I: InputReader> Model<I> for World {
  fn update(
    &mut self,
    input: &I,
  ) {
    self.flora.update(input);
    if input.get_reset_requested() {
      self.reset();
    } else {
      crate::updaters::bugs::update(input, self);
    }
  }
}
