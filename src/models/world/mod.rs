// =============================================================================
//! - Structures for the World model
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-20
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
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub struct World {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  flora: Rc<RefCell<Flora>>,
  updaters: [Rc<RefCell<dyn Updater>>; 3],
}

// TODO: extract the trait?
impl World {
  pub fn clock_clone(&self) -> Rc<RefCell<Clock>> {
    self.clock.clone()
  }

  pub fn fauna_clone(&self) -> Rc<RefCell<Fauna>> {
    self.fauna.clone()
  }

  pub fn flora_clone(&self) -> Rc<RefCell<Flora>> {
    self.flora.clone()
  }

  pub fn new(input: Rc<RefCell<Input>>) -> Self {
    let clock = Rc::new(RefCell::new(Clock::new(input.clone())));
    let flora = Rc::new(RefCell::new(Flora::new(input.clone())));
    let fauna = Rc::new(RefCell::new(Fauna::new(
      clock.clone(),
      flora.clone(),
      input.clone(),
    )));
    let updaters: [Rc<RefCell<dyn Updater>>; 3] = [
      clock.clone(),
      flora.clone(),
      fauna.clone(),
    ];
    Self {
      clock,
      fauna,
      flora,
      updaters,
    }
  }
}

impl Updater for World {
  fn update(&mut self) {
    self.updaters.iter().for_each(|model| model.borrow_mut().update());
  }
}
