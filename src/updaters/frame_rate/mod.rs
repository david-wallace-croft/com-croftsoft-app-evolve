// =============================================================================
//! - Frame Rate Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-05
//! - Since: 2023-02-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rater::FrameRater;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait FrameRateUpdaterInput {
  fn get_reset_requested(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct FrameRateUpdater {
  frame_rater: Rc<RefCell<FrameRater>>,
  input: Rc<RefCell<dyn FrameRateUpdaterInput>>,
}

impl FrameRateUpdater {
  pub fn new(
    frame_rater: Rc<RefCell<FrameRater>>,
    input: Rc<RefCell<dyn FrameRateUpdaterInput>>,
  ) -> Self {
    Self {
      frame_rater,
      input,
    }
  }
}

impl Updater for FrameRateUpdater {
  fn update(&mut self) {
    let input: Ref<dyn FrameRateUpdaterInput> = self.input.borrow();
    if let Some(update_period_millis) = input.get_update_period_millis_changed()
    {
      self
        .frame_rater
        .borrow_mut()
        .update_frame_sample_size(update_period_millis);
    }
    if input.get_reset_requested() {
      self.frame_rater.borrow_mut().clear();
      return;
    }
    self.frame_rater.borrow_mut().sample(input.get_update_time_millis());
  }
}
