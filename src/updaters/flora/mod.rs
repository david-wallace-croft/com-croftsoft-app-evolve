// =============================================================================
//! - Flora Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-06
//! - Since: 2023-01-25
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

pub trait FloraUpdaterInputs {
  fn get_blight_requested(&self) -> bool;
  fn get_flora_growth_rate_change_requested(&self) -> Option<usize>;
  fn get_garden_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
}

pub struct FloraUpdater {
  flora: Rc<RefCell<Flora>>,
  inputs: Rc<RefCell<dyn FloraUpdaterInputs>>,
}

impl FloraUpdater {
  pub fn new(
    flora: Rc<RefCell<Flora>>,
    inputs: Rc<RefCell<dyn FloraUpdaterInputs>>,
  ) -> Self {
    Self {
      flora,
      inputs,
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
      }
    }
    if self.flora.borrow().enabled_garden {
      self.set_garden_values(true);
    }
  }
}

impl Updater for FloraUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_reset_requested() {
      self.set_flora_present_for_all_locations(true);
      return;
    }
    if let Some(flora_growth_rate) =
      self.inputs.borrow().get_flora_growth_rate_change_requested()
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
    } else {
      let mut thread_rng: ThreadRng = rand::thread_rng();
      let mut flora: RefMut<Flora> = self.flora.borrow_mut();
      for _i in 0..flora.flora_growth_rate {
        // Randomly position food flora
        let index: usize = thread_rng.gen_range(0..flora.flora_present.len());
        flora.flora_present[index] = true;
      }
    }
    self.update_garden();
  }
}
