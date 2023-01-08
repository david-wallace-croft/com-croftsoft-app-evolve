// =============================================================================
//! - Looper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-08
//! - Since: 2023-01-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::configuration::Configuration;
use super::input::Input;
use super::traits::{Component, WorldPainter};
use crate::components::evolve::EvolveComponent;
use crate::constants::{CONFIGURATION, FRAME_PERIOD_MILLIS_MINIMUM};
use crate::engine::functions::web_sys::{spawn_local_loop, LoopUpdater};
use crate::models::fauna::Fauna;
use crate::models::world::World;
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  configuration: Configuration,
  evolve_component: EvolveComponent,
  frame_period_millis: f64,
  input: Input,
  next_update_time: f64,
  world_ref: Rc<RefCell<World>>,
}

impl Looper {
  pub fn init(&mut self) {
    self.evolve_component.init();
    self.input.reset_requested = true;
  }

  pub fn launch() {
    let mut looper = Looper::default();
    looper.init();
    spawn_local_loop(looper);
  }

  pub fn new(configuration: Configuration) -> Self {
    let Configuration {
      frame_period_millis,
    } = configuration;
    let world_ref = Rc::new(RefCell::new(World::default()));
    let clone = Rc::clone(&world_ref);
    let looper = Self {
      evolve_component: EvolveComponent::new("evolve"),
      configuration,
      input: Input::default(),
      frame_period_millis,
      next_update_time: 0.0,
      world_ref,
    };
    looper.world_ref.borrow_mut().fauna_option = Some(Fauna::new(clone));
    looper
  }

  fn update_frame_rate(&mut self) {
    if !self.input.speed_toggle_requested {
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

impl LoopUpdater for Looper {
  // TODO: rename this function
  fn update_loop(
    &mut self,
    update_time: f64,
  ) {
    if update_time < self.next_update_time {
      return;
    }
    self.evolve_component.update(&mut self.input);
    World::update_world(&self.input, &self.world_ref);
    self.evolve_component.paint(&self.world_ref.borrow());
    self.update_frame_rate();
    self.next_update_time = update_time + self.frame_period_millis;
    self.input.clear();
  }
}
