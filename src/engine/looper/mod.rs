// =============================================================================
//! - Looper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-01
//! - Since: 2023-01-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::configuration::Configuration;
use super::input::Input;
use crate::animators::frame_rate::FrameRateAnimator;
use crate::components::evolve::EvolveComponent;
use crate::constants::{CONFIGURATION, FRAME_PERIOD_MILLIS_MINIMUM};
use crate::engine::functions::web_sys::{spawn_local_loop, LoopUpdater};
use crate::models::world::World;
use crate::updaters::world::WorldUpdater;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  configuration: Configuration,
  evolve_component: EvolveComponent,
  frame_period_millis: f64,
  frame_rate_animator: FrameRateAnimator,
  input: Rc<RefCell<Input>>,
  next_update_time: f64,
  world_updater: WorldUpdater,
}

impl Looper {
  pub fn launch() {
    let mut looper = Looper::default();
    looper.initialize();
    spawn_local_loop(looper);
  }

  pub fn new(configuration: Configuration) -> Self {
    let Configuration {
      frame_period_millis,
    } = configuration;
    let input = Rc::new(RefCell::new(Input::default()));
    let world = Rc::new(RefCell::new(World::default()));
    let evolve_component =
      EvolveComponent::new("evolve", input.clone(), world.clone());
    let world_updater = WorldUpdater::new(input.clone(), world);
    let frame_rate_animator = FrameRateAnimator::default();
    Self {
      configuration,
      evolve_component,
      frame_period_millis,
      frame_rate_animator,
      input,
      next_update_time: 0.0,
      world_updater,
    }
  }

  fn update_frame_rate(&mut self) {
    if !self.input.borrow().speed_toggle_requested {
      return;
    }
    if self.frame_period_millis == FRAME_PERIOD_MILLIS_MINIMUM {
      self.frame_period_millis = self.configuration.frame_period_millis;
    } else {
      self.frame_period_millis = FRAME_PERIOD_MILLIS_MINIMUM;
    }
  }
}

impl Default for Looper {
  fn default() -> Self {
    Looper::new(CONFIGURATION)
  }
}

impl Initializer for Looper {
  fn initialize(&mut self) {
    self.evolve_component.initialize();
    self.input.borrow_mut().reset_requested = true;
  }
}

impl LoopUpdater for Looper {
  // TODO: rename this function
  fn update_loop(
    &mut self,
    update_time: f64,
  ) {
    if update_time < self.next_update_time {
      return;
    }
    self.evolve_component.update();
    self.world_updater.update();
    self.evolve_component.paint();
    self.frame_rate_animator.update();
    self.frame_rate_animator.paint();
    self.update_frame_rate();
    self.next_update_time = update_time + self.frame_period_millis;
    self.input.borrow_mut().clear();
  }
}
