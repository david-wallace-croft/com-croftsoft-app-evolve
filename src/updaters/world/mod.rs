// =============================================================================
//! - World Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-25
//! - Updated: 2023-02-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::clock::{ClockUpdater, ClockUpdaterInputs};
use super::fauna::{FaunaUpdater, FaunaUpdaterInputs};
use super::flora::{FloraUpdater, FloraUpdaterInputs};
use super::frame_rate::{
  FrameRateUpdater, FrameRateUpdaterEvents, FrameRateUpdaterInputs,
};
use super::overlay::{OverlayUpdater, OverlayUpdaterInputs};
use crate::engine::frame_rater::FrameRater;
use crate::engine::update_timer::UpdateTimer;
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use crate::models::frame_rate::FrameRate;
use crate::models::overlay::Overlay;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct WorldUpdaterConfiguration {
  pub update_period_millis_maximum: f64,
  pub update_period_millis_minimum: f64,
}

pub trait WorldUpdaterEvents {
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated_world(&mut self);
}

pub trait WorldUpdaterInputs {
  fn get_blight_requested(&self) -> bool;
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_speed_toggle_requested(&self) -> bool;
  fn get_update_time_millis(&self) -> f64;
}

struct WorldUpdaterEventsAdapter {
  events: Rc<RefCell<dyn WorldUpdaterEvents>>,
}

impl WorldUpdaterEventsAdapter {
  fn new(events: Rc<RefCell<dyn WorldUpdaterEvents>>) -> Self {
    Self {
      events,
    }
  }
}

struct WorldUpdaterInputAdapter {
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
}

impl WorldUpdaterInputAdapter {
  fn new(inputs: Rc<RefCell<dyn WorldUpdaterInputs>>) -> Self {
    Self {
      inputs,
    }
  }
}

impl ClockUpdaterInputs for WorldUpdaterInputAdapter {
  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl FaunaUpdaterInputs for WorldUpdaterInputAdapter {
  fn get_bug_requested(&self) -> Option<usize> {
    self.inputs.borrow().get_bug_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl FloraUpdaterInputs for WorldUpdaterInputAdapter {
  fn get_blight_requested(&self) -> bool {
    self.inputs.borrow().get_blight_requested()
  }

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize> {
    self.inputs.borrow().get_flora_growth_rate_change_requested()
  }

  fn get_garden_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_garden_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl FrameRateUpdaterEvents for WorldUpdaterEventsAdapter {
  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }
}

impl FrameRateUpdaterInputs for WorldUpdaterInputAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_frame_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_update_time_millis()
  }
}

impl OverlayUpdaterInputs for WorldUpdaterInputAdapter {
  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_update_time_millis()
  }
}

pub struct WorldUpdater {
  configuration: WorldUpdaterConfiguration,
  events: Rc<RefCell<dyn WorldUpdaterEvents>>,
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
  update_timer_world: UpdateTimer,
  updaters: [Box<dyn Updater>; 5],
}

impl WorldUpdater {
  pub fn new(
    configuration: WorldUpdaterConfiguration,
    events: Rc<RefCell<dyn WorldUpdaterEvents>>,
    frame_rate: Rc<RefCell<FrameRate>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let world_updater_events_adapter =
      Rc::new(RefCell::new(WorldUpdaterEventsAdapter::new(events.clone())));
    let update_timer_world = UpdateTimer {
      update_period_millis: configuration.update_period_millis_maximum,
      update_time_millis_next: 0.,
    };
    let world: Ref<World> = world.borrow();
    let clock: Rc<RefCell<Clock>> = world.clock.clone();
    let fauna: Rc<RefCell<Fauna>> = world.fauna.clone();
    let flora: Rc<RefCell<Flora>> = world.flora.clone();
    let overlay: Rc<RefCell<Overlay>> = world.overlay.clone();
    let world_updater_input_adapter =
      Rc::new(RefCell::new(WorldUpdaterInputAdapter::new(inputs.clone())));
    let clock_updater =
      ClockUpdater::new(clock.clone(), world_updater_input_adapter.clone());
    let fauna_updater = FaunaUpdater::new(
      clock.clone(),
      fauna.clone(),
      flora.clone(),
      world_updater_input_adapter.clone(),
    );
    let flora_updater =
      FloraUpdater::new(flora, world_updater_input_adapter.clone());
    let frame_rate_updater = FrameRateUpdater::new(
      world_updater_events_adapter,
      frame_rate,
      frame_rater,
      world_updater_input_adapter.clone(),
    );
    let overlay_updater =
      OverlayUpdater::new(clock, fauna, world_updater_input_adapter, overlay);
    let updaters: [Box<dyn Updater>; 5] = [
      Box::new(clock_updater),
      Box::new(flora_updater),
      Box::new(fauna_updater),
      Box::new(overlay_updater),
      Box::new(frame_rate_updater),
    ];
    Self {
      configuration,
      events,
      inputs,
      update_timer_world,
      updaters,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_speed_toggle_requested() {
      self.update_timer_world.update_period_millis =
        if self.update_timer_world.update_period_millis
          == self.configuration.update_period_millis_minimum
        {
          self.configuration.update_period_millis_maximum
        } else {
          self.configuration.update_period_millis_minimum
        };
      self.events.borrow_mut().set_update_period_millis_changed(
        self.update_timer_world.update_period_millis,
      );
      self.update_timer_world.update_time_millis_next = 0.;
    }
    let update_time_millis = self.inputs.borrow().get_update_time_millis();
    {
      let inputs: Ref<dyn WorldUpdaterInputs> = self.inputs.borrow();
      if self.update_timer_world.before_next_update_time(update_time_millis)
        && !inputs.get_blight_requested()
        && inputs.get_bug_requested().is_none()
        && inputs.get_flora_growth_rate_change_requested().is_none()
        && inputs.get_frame_rate_display_change_requested().is_none()
        && inputs.get_garden_change_requested().is_none()
        && !inputs.get_reset_requested()
      {
        return;
      }
    }
    self.updaters.iter_mut().for_each(|updater| updater.update());
    self.events.borrow_mut().set_updated_world();
  }
}
