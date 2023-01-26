// =============================================================================
//! - World Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2023-01-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::clock::{ClockUpdater, ClockUpdaterInput};
use super::fauna::{FaunaUpdater, FaunaUpdaterInput};
use super::flora::{FloraUpdater, FloraUpdaterInput};
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait WorldUpdaterInput {
  fn get_blight_requested(&self) -> bool;
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
}

struct WorldUpdaterInputAdapter {
  input: Rc<RefCell<dyn WorldUpdaterInput>>,
}

impl WorldUpdaterInputAdapter {
  fn new(input: Rc<RefCell<dyn WorldUpdaterInput>>) -> Self {
    Self {
      input,
    }
  }
}

impl ClockUpdaterInput for WorldUpdaterInputAdapter {
  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

impl FaunaUpdaterInput for WorldUpdaterInputAdapter {
  fn get_bug_requested(&self) -> Option<usize> {
    self.input.borrow().get_bug_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

impl FloraUpdaterInput for WorldUpdaterInputAdapter {
  fn get_blight_requested(&self) -> bool {
    self.input.borrow().get_blight_requested()
  }

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize> {
    self.input.borrow().get_flora_growth_rate_change_requested()
  }

  fn get_garden_change_requested(&self) -> Option<bool> {
    self.input.borrow().get_garden_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

pub struct WorldUpdater {
  updaters: [Box<dyn Updater>; 3],
}

impl WorldUpdater {
  pub fn new(
    input: Rc<RefCell<dyn WorldUpdaterInput>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let world: Ref<World> = world.borrow();
    let clock: Rc<RefCell<Clock>> = world.clock.clone();
    let fauna: Rc<RefCell<Fauna>> = world.fauna.clone();
    let flora: Rc<RefCell<Flora>> = world.flora.clone();
    let world_updater_input_adapter =
      Rc::new(RefCell::new(WorldUpdaterInputAdapter::new(input)));
    let clock_updater =
      ClockUpdater::new(clock.clone(), world_updater_input_adapter.clone());
    let fauna_updater = FaunaUpdater::new(
      clock,
      fauna,
      flora.clone(),
      world_updater_input_adapter.clone(),
    );
    let flora_updater = FloraUpdater::new(flora, world_updater_input_adapter);
    let updaters: [Box<dyn Updater>; 3] = [
      Box::new(clock_updater),
      Box::new(flora_updater),
      Box::new(fauna_updater),
    ];
    Self {
      updaters,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    self.updaters.iter_mut().for_each(|updater| updater.update());
  }
}
