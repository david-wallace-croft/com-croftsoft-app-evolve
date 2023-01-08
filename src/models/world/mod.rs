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

use super::clock::Clock;
use super::fauna::Fauna;
use super::flora::Flora;
use crate::engine::input::Input;
use crate::engine::traits::Model;
use core::cell::RefCell;
use core::mem::take;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
  pub clock: Clock,
  pub fauna_option: Option<Fauna>,
  pub flora: Flora,
}

impl World {
  // make a Trait out of these methods
  pub fn get_fauna_as_ref(&self) -> &Fauna {
    self.fauna_option.as_ref().unwrap()
  }

  pub fn update_world(
    input: &Input,
    world_ref: &Rc<RefCell<World>>,
  ) {
    world_ref.borrow_mut().clock.update(input);
    world_ref.borrow_mut().flora.update(input);
    let mut fauna_option = take(&mut world_ref.borrow_mut().fauna_option);
    let fauna: &mut Fauna = fauna_option.as_mut().unwrap();
    fauna.update(input);
    world_ref.borrow_mut().fauna_option = fauna_option;
  }
}
