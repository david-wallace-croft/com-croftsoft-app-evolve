// =============================================================================
//! - Looper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-06
//! - Since: 2023-01-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::configuration::Configuration;
use super::frame_rater::FrameRater;
use crate::components::evolve::EvolveComponent;
use crate::constants::{CONFIGURATION, FRAME_PERIOD_MILLIS_MINIMUM};
use crate::engine::functions::web_sys::{spawn_local_loop, LoopUpdater};
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::models::world::World;
use crate::updaters::world::{WorldUpdater, WorldUpdaterConfiguration};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  events: Rc<RefCell<Events>>,
  evolve_component: EvolveComponent,
  inputs: Rc<RefCell<Inputs>>,
  world_updater: WorldUpdater,
}

impl Looper {
  pub fn launch() {
    let mut looper = Looper::default();
    looper.initialize();
    spawn_local_loop(looper);
  }

  pub fn new(configuration: Configuration) -> Self {
    let world_updater_configuration = WorldUpdaterConfiguration {
      update_period_millis_maximum: configuration.frame_period_millis,
      update_period_millis_minimum: FRAME_PERIOD_MILLIS_MINIMUM,
    };
    let frame_rater = Rc::new(RefCell::new(FrameRater::new(
      configuration.frame_period_millis,
    )));
    let events = Rc::new(RefCell::new(Events::default()));
    let inputs = Rc::new(RefCell::new(Inputs::default()));
    let world = Rc::new(RefCell::new(World::default()));
    let evolve_component = EvolveComponent::new(
      events.clone(),
      "evolve",
      frame_rater.clone(),
      inputs.clone(),
      world.clone(),
    );
    let world_updater = WorldUpdater::new(
      world_updater_configuration,
      events.clone(),
      frame_rater,
      inputs.clone(),
      world,
    );
    Self {
      events,
      evolve_component,
      inputs,
      world_updater,
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
    self.inputs.borrow_mut().reset_requested = true;
  }
}

impl LoopUpdater for Looper {
  // TODO: rename this function
  fn update_loop(
    &mut self,
    update_time_millis: f64,
  ) {
    self.events.borrow_mut().update_time_millis = update_time_millis;
    self.evolve_component.update();
    self.world_updater.update();
    self.evolve_component.paint();
    self.events.borrow_mut().clear();
    self.inputs.borrow_mut().clear();
  }
}
