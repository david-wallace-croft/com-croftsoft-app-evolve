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
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct World {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  flora: Rc<RefCell<Flora>>,
  models: Vec<Rc<RefCell<dyn Model>>>,
}

// TODO: extract the trait?
impl World {
  pub fn clock_as_ref(&self) -> Ref<Clock> {
    self.clock.borrow()
  }

  pub fn fauna_as_ref(&self) -> Ref<Fauna> {
    self.fauna.borrow()
  }

  pub fn flora_as_ref(&self) -> Ref<Flora> {
    self.flora.borrow()
  }
}

impl Default for World {
  fn default() -> Self {
    let clock = Rc::new(RefCell::new(Clock::default()));
    let flora = Rc::new(RefCell::new(Flora::default()));
    let fauna = Rc::new(RefCell::new(Fauna::new(clock.clone(), flora.clone())));
    let models: Vec<Rc<RefCell<dyn Model>>> = vec![
      clock.clone(),
      flora.clone(),
      fauna.clone(),
    ];
    Self {
      clock,
      fauna,
      flora,
      models,
    }
  }
}

impl Model for World {
  fn update(
    &mut self,
    input: &Input,
  ) {
    self.models.iter().for_each(|model| model.borrow_mut().update(input));
  }
}
