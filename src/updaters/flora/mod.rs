// =============================================================================
//! - Flora Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-25
//! - Updated: 2023-02-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{
  EDEN_X0, EDEN_X1, EDEN_Y0, EDEN_Y1, FLORA_GROWTH_RATE_MAX,
};
use crate::engine::functions::location::to_index_from_xy;
use crate::models::flora::Flora;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::rc::Rc;
// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

pub trait FloraUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait FloraUpdaterInputs {
  fn get_blight_requested(&self) -> bool;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
}

pub trait FloraUpdaterOptions {
  fn get_pause(&self) -> bool;
}

pub struct FloraUpdater {
  events: Rc<RefCell<dyn FloraUpdaterEvents>>,
  flora: Rc<RefCell<Flora>>,
  inputs: Rc<RefCell<dyn FloraUpdaterInputs>>,
  options: Rc<RefCell<dyn FloraUpdaterOptions>>,
}

impl FloraUpdater {
  pub fn new(
    events: Rc<RefCell<dyn FloraUpdaterEvents>>,
    flora: Rc<RefCell<Flora>>,
    inputs: Rc<RefCell<dyn FloraUpdaterInputs>>,
    options: Rc<RefCell<dyn FloraUpdaterOptions>>,
  ) -> Self {
    Self {
      events,
      flora,
      inputs,
      options,
    }
  }

  fn set_flora_present_for_all_locations(
    &mut self,
    present: bool,
  ) {
    self
      .flora
      .borrow_mut()
      .flora_present
      .iter_mut()
      .for_each(|location: &mut bool| *location = present);
  }

  fn set_garden_values(
    &mut self,
    value: bool,
  ) {
    let mut flora: RefMut<Flora> = self.flora.borrow_mut();
    for x in EDEN_X0..=EDEN_X1 {
      for y in EDEN_Y0..=EDEN_Y1 {
        let index: usize = to_index_from_xy(x, y);
        flora.flora_present[index] = value;
      }
    }
  }

  // TODO: move this
  fn update_garden(&mut self) {
    let garden_change_requested: Option<bool> =
      self.inputs.borrow().get_garden_change_requested();
    if let Some(enabled) = garden_change_requested {
      self.flora.borrow_mut().enabled_garden = enabled;
      if !self.flora.borrow().enabled_garden {
        self.set_garden_values(false);
        self.events.borrow_mut().set_updated();
      }
    }
    let time_to_update: bool = self.inputs.borrow().get_time_to_update();
    if time_to_update
      && !self.options.borrow().get_pause()
      && self.flora.borrow().enabled_garden
    {
      self.set_garden_values(true);
      self.events.borrow_mut().set_updated();
    }
  }
}

impl Updater for FloraUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_reset_requested() {
      self.set_flora_present_for_all_locations(true);
      self.events.borrow_mut().set_updated();
      return;
    }
    if let Some(flora_growth_rate) = self
      .inputs
      .borrow()
      .get_flora_growth_rate_change_requested()
    {
      let mut flora: RefMut<Flora> = self.flora.borrow_mut();
      if flora_growth_rate < FLORA_GROWTH_RATE_MAX {
        flora.flora_growth_rate = flora_growth_rate;
      } else {
        flora.flora_growth_rate = FLORA_GROWTH_RATE_MAX;
      }
    }
    if self.inputs.borrow().get_blight_requested() {
      self.set_flora_present_for_all_locations(false);
      self.events.borrow_mut().set_updated();
    } else {
      let time_to_update: bool = self.inputs.borrow().get_time_to_update();
      if time_to_update && !self.options.borrow().get_pause() {
        let mut thread_rng: ThreadRng = rand::thread_rng();
        let mut flora: RefMut<Flora> = self.flora.borrow_mut();
        for _i in 0..flora.flora_growth_rate {
          // Randomly position food flora
          let index: usize = thread_rng.gen_range(0..flora.flora_present.len());
          flora.flora_present[index] = true;
        }
        self.events.borrow_mut().set_updated();
      }
    }
    self.update_garden();
  }
}
