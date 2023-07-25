// =============================================================================
//! - Root Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-25
//! - Updated: 2023-06-26
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::clock::{
  ClockUpdater, ClockUpdaterEvents, ClockUpdaterInputs, ClockUpdaterOptions,
};
use super::fauna::{
  FaunaUpdater, FaunaUpdaterEvents, FaunaUpdaterInputs, FaunaUpdaterOptions,
};
use super::flora::{
  FloraUpdater, FloraUpdaterEvents, FloraUpdaterInputs, FloraUpdaterOptions,
};
use super::options::{OptionsUpdater, OptionsUpdaterInputs};
use super::overlay::{
  OverlayUpdater, OverlayUpdaterEvents, OverlayUpdaterInputs,
  OverlayUpdaterOptions,
};
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use crate::models::options::Options;
use crate::models::overlay::Overlay;
use crate::models::root::Root;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdaterInputs;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::updater::{
  MetronomeUpdater, MetronomeUpdaterEvents, MetronomeUpdaterInputs,
};
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct RootUpdaterConfiguration {
  pub update_period_millis_initial: f64,
}

pub trait RootUpdaterEvents {
  fn get_updated(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_time_to_update(&mut self);
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated(&mut self);
}

struct RootUpdaterEventsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
}

impl RootUpdaterEventsAdapter {
  fn new(events: Rc<RefCell<dyn RootUpdaterEvents>>) -> Self {
    Self {
      events,
    }
  }
}

impl ClockUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl FaunaUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl FloraUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl MetronomeUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self
      .events
      .borrow_mut()
      .set_update_period_millis_changed(update_period_millis);
  }

  fn set_tick(&mut self) {
    self.events.borrow_mut().set_time_to_update();
  }
}

impl OverlayUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

pub trait RootUpdaterInputs {
  fn get_blight_requested(&self) -> bool;
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_current_time_millis(&self) -> f64;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_display_change_requested(&self) -> Option<bool>;
}

struct RootUpdaterInputsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
  inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
}

impl RootUpdaterInputsAdapter {
  fn new(
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      inputs,
    }
  }
}

impl ClockUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl FaunaUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_bug_requested(&self) -> Option<usize> {
    self.inputs.borrow().get_bug_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl FloraUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_blight_requested(&self) -> bool {
    self.inputs.borrow().get_blight_requested()
  }

  fn get_flora_growth_rate_change_requested(&self) -> Option<usize> {
    self
      .inputs
      .borrow()
      .get_flora_growth_rate_change_requested()
  }

  fn get_garden_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_garden_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl FrameRaterUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl MetronomeUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.inputs.borrow().get_period_millis_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl OptionsUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_time_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_time_display_change_requested()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }
}

impl OverlayUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_bug_requested(&self) -> Option<usize> {
    self.inputs.borrow().get_bug_requested()
  }

  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_time_display_change_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }
}

pub trait RootUpdaterOptions {
  fn get_pause(&self) -> bool;
  fn get_time_display(&self) -> bool;
  fn get_update_rate_display(&self) -> bool;
}

struct RootUpdaterOptionsAdapter {
  options: Rc<RefCell<dyn RootUpdaterOptions>>,
}

impl RootUpdaterOptionsAdapter {
  fn new(options: Rc<RefCell<dyn RootUpdaterOptions>>) -> Self {
    Self {
      options,
    }
  }
}

impl ClockUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl FaunaUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl FloraUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl OverlayUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }

  fn get_time_display(&self) -> bool {
    self.options.borrow().get_time_display()
  }

  fn get_update_rate_display(&self) -> bool {
    self.options.borrow().get_update_rate_display()
  }
}

pub struct RootUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
}

impl RootUpdater {
  pub fn new(
    configuration: RootUpdaterConfiguration,
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
    root_model: Rc<RefCell<Root>>,
  ) -> Self {
    let root_updater_events_adapter =
      Rc::new(RefCell::new(RootUpdaterEventsAdapter::new(events.clone())));
    let root_updater_inputs_adapter = Rc::new(RefCell::new(
      RootUpdaterInputsAdapter::new(events.clone(), inputs.clone()),
    ));
    let root_updater_options_adapter = Rc::new(RefCell::new(
      RootUpdaterOptionsAdapter::new(options.clone()),
    ));
    let root_model: Ref<Root> = root_model.borrow();
    let clock: Rc<RefCell<Clock>> = root_model.clock.clone();
    let fauna: Rc<RefCell<Fauna>> = root_model.fauna.clone();
    let flora: Rc<RefCell<Flora>> = root_model.flora.clone();
    let overlay: Rc<RefCell<Overlay>> = root_model.overlay.clone();
    let clock_updater = ClockUpdater::new(
      clock.clone(),
      root_updater_events_adapter.clone(),
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter.clone(),
    );
    let fauna_updater = FaunaUpdater::new(
      clock.clone(),
      root_updater_events_adapter.clone(),
      fauna.clone(),
      flora.clone(),
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter.clone(),
    );
    let flora_updater = FloraUpdater::new(
      root_updater_events_adapter.clone(),
      flora,
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter.clone(),
    );
    let frame_rater_updater = FrameRaterUpdater::new(
      false,
      frame_rater.clone(),
      root_updater_inputs_adapter.clone(),
    );
    let options_updater =
      OptionsUpdater::new(root_updater_inputs_adapter.clone(), options);
    let overlay_updater = OverlayUpdater::new(
      clock,
      root_updater_events_adapter.clone(),
      fauna,
      frame_rater,
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter,
      overlay,
    );
    let metronome = Rc::new(RefCell::new(DeltaMetronome {
      period_millis: configuration.update_period_millis_initial,
      time_millis_next_tick: 0.,
    }));
    let metronome_updater = MetronomeUpdater::new(
      root_updater_events_adapter,
      root_updater_inputs_adapter,
      metronome,
    );
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(metronome_updater),
      Box::new(options_updater),
      Box::new(frame_rater_updater),
      Box::new(clock_updater),
      Box::new(flora_updater),
      Box::new(fauna_updater),
      Box::new(overlay_updater),
    ];
    Self {
      child_updaters,
    }
  }
}

impl Updater for RootUpdater {
  fn update(&mut self) {
    self
      .child_updaters
      .iter_mut()
      .for_each(|updater| updater.update());
  }
}
