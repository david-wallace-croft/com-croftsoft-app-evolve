// =============================================================================
//! - Frame Rate Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-06
//! - Since: 2023-02-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rater::FrameRater;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait FrameRateUpdaterEvents {
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
}

pub trait FrameRateUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
}

pub struct FrameRateUpdater {
  events: Rc<RefCell<dyn FrameRateUpdaterEvents>>,
  frame_rater: Rc<RefCell<FrameRater>>,
  inputs: Rc<RefCell<dyn FrameRateUpdaterInputs>>,
}

impl FrameRateUpdater {
  pub fn new(
    events: Rc<RefCell<dyn FrameRateUpdaterEvents>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn FrameRateUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      frame_rater,
      inputs,
    }
  }
}

impl Updater for FrameRateUpdater {
  fn update(&mut self) {
    let events: Ref<dyn FrameRateUpdaterEvents> = self.events.borrow();
    if let Some(update_period_millis) =
      events.get_update_period_millis_changed()
    {
      self
        .frame_rater
        .borrow_mut()
        .update_frame_sample_size(update_period_millis);
    }
    if self.inputs.borrow().get_reset_requested() {
      self.frame_rater.borrow_mut().clear();
      return;
    }
    self.frame_rater.borrow_mut().sample(events.get_update_time_millis());
  }
}
