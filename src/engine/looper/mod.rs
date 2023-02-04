// =============================================================================
//! - Looper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-04
//! - Since: 2023-01-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::configuration::Configuration;
use super::frame_rater::FrameRater;
use super::input::Input;
use crate::components::evolve::EvolveComponent;
use crate::constants::{CONFIGURATION, FRAME_PERIOD_MILLIS_MINIMUM};
use crate::engine::functions::web_sys::{spawn_local_loop, LoopUpdater};
use crate::models::world::World;
use crate::updaters::world::WorldUpdater;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::{RefCell, RefMut};
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  configuration: Configuration,
  evolve_component: EvolveComponent,
  frame_rater: Rc<RefCell<FrameRater>>,
  input: Rc<RefCell<Input>>,
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
    let frame_rater =
      Rc::new(RefCell::new(FrameRater::new(frame_period_millis)));
    let input = Rc::new(RefCell::new(Input::default()));
    let world = Rc::new(RefCell::new(World::default()));
    let evolve_component = EvolveComponent::new(
      "evolve",
      frame_rater.clone(),
      input.clone(),
      world.clone(),
    );
    let world_updater = WorldUpdater::new(input.clone(), world);
    Self {
      configuration,
      evolve_component,
      frame_rater,
      input,
      world_updater,
    }
  }

  fn update_frame_rate(&mut self) {
    if !self.input.borrow().speed_toggle_requested {
      return;
    }
    self.input.borrow_mut().speed_toggle_requested = false;
    let mut frame_rater: RefMut<FrameRater> = self.frame_rater.borrow_mut();
    let frame_period_millis = frame_rater.get_frame_period_millis_target();
    if frame_period_millis == FRAME_PERIOD_MILLIS_MINIMUM {
      frame_rater
        .set_frame_period_millis_target(self.configuration.frame_period_millis);
    } else {
      frame_rater.set_frame_period_millis_target(FRAME_PERIOD_MILLIS_MINIMUM);
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
    self.evolve_component.update();
    self.update_frame_rate();
    if self.frame_rater.borrow_mut().before_next_update_time(update_time) {
      return;
    }
    self.world_updater.update();
    self.evolve_component.paint();
    self.input.borrow_mut().clear();
  }
}
