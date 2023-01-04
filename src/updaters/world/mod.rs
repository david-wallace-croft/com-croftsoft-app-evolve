// =============================================================================
//! - WorldUpdater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-03
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
use crate::models::world::World;
use crate::traits::InputReader;

pub fn reset(world: &mut World) {
  let position: usize = to_index_from_xy(SPACE_WIDTH / 2, SPACE_HEIGHT / 2);
  world.bugs.clear();
  for _i in 0..BUGS_MAX {
    let bug = Bug::new(position);
    // let bug_str = format!("{:?}", bug);
    // console::log_1(&JsValue::from_str(&bug_str));
    world.bugs.push(bug);
  }
  for index in 0..SPACE_HEIGHT * SPACE_WIDTH {
    world.flora_present[index] = true;
  }
}

pub fn update<I: InputReader>(
  input: &I,
  world: &mut World,
) {
  if input.get_reset_requested() {
    reset(world);
  } else {
    super::flora::update(input, world);
    super::bugs::update(input, world);
  }
}
