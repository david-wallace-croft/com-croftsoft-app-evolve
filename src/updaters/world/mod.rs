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
use super::frame_rate::{FrameRateUpdater, FrameRateUpdaterInput};
use crate::engine::frame_rater::FrameRater;
use crate::engine::update_timer::UpdateTimer;
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct WorldUpdaterConfiguration {
  pub update_period_millis_maximum: f64,
  pub update_period_millis_minimum: f64,
}

pub trait WorldUpdaterInput {
  fn get_blight_requested(&self) -> bool;
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_speed_toggle_requested(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated_world(&mut self);
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

impl FrameRateUpdaterInput for WorldUpdaterInputAdapter {
  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.input.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.input.borrow().get_update_time_millis()
  }
}

pub struct WorldUpdater {
  configuration: WorldUpdaterConfiguration,
  input: Rc<RefCell<dyn WorldUpdaterInput>>,
  update_timer_world: UpdateTimer,
  updaters: [Box<dyn Updater>; 4],
}

impl WorldUpdater {
  pub fn new(
    configuration: WorldUpdaterConfiguration,
    frame_rater: Rc<RefCell<FrameRater>>,
    input: Rc<RefCell<dyn WorldUpdaterInput>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let update_timer_world = UpdateTimer {
      update_period_millis: configuration.update_period_millis_maximum,
      update_time_millis_next: 0.,
    };
    let world: Ref<World> = world.borrow();
    let clock: Rc<RefCell<Clock>> = world.clock.clone();
    let fauna: Rc<RefCell<Fauna>> = world.fauna.clone();
    let flora: Rc<RefCell<Flora>> = world.flora.clone();
    let world_updater_input_adapter =
      Rc::new(RefCell::new(WorldUpdaterInputAdapter::new(input.clone())));
    let clock_updater =
      ClockUpdater::new(clock.clone(), world_updater_input_adapter.clone());
    let fauna_updater = FaunaUpdater::new(
      clock,
      fauna,
      flora.clone(),
      world_updater_input_adapter.clone(),
    );
    let flora_updater =
      FloraUpdater::new(flora, world_updater_input_adapter.clone());
    let frame_rate_updater =
      FrameRateUpdater::new(frame_rater, world_updater_input_adapter);
    let updaters: [Box<dyn Updater>; 4] = [
      Box::new(clock_updater),
      Box::new(flora_updater),
      Box::new(fauna_updater),
      Box::new(frame_rate_updater),
    ];
    Self {
      configuration,
      input,
      update_timer_world,
      updaters,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    if self.input.borrow().get_speed_toggle_requested() {
      self.update_timer_world.update_period_millis =
        if self.update_timer_world.update_period_millis
          == self.configuration.update_period_millis_minimum
        {
          self.configuration.update_period_millis_maximum
        } else {
          self.configuration.update_period_millis_minimum
        };
      self.input.borrow_mut().set_update_period_millis_changed(
        self.update_timer_world.update_period_millis,
      );
      self.update_timer_world.update_time_millis_next = 0.;
    }
    let update_time_millis = self.input.borrow().get_update_time_millis();
    {
      let input: Ref<dyn WorldUpdaterInput> = self.input.borrow();
      if self.update_timer_world.before_next_update_time(update_time_millis)
        && !input.get_blight_requested()
        && input.get_bug_requested().is_none()
        && input.get_flora_growth_rate_change_requested().is_none()
        && input.get_garden_change_requested().is_none()
        && !input.get_reset_requested()
      {
        return;
      }
    }
    self.updaters.iter_mut().for_each(|updater| updater.update());
    self.input.borrow_mut().set_updated_world();
  }
}
