// =============================================================================
//! - Frame Rate Animator for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-01
//! - Since: 2023-02-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::{Painter, Updater};

use crate::engine::traits::Animator;

#[derive(Default)]
pub struct FrameRateAnimator {
  frame_count: usize,
}

impl Animator for FrameRateAnimator {}

impl Painter for FrameRateAnimator {
  fn paint(&self) {
    // TODO
  }
}

impl Updater for FrameRateAnimator {
  fn update(&mut self) {
    self.frame_count += 1;
  }
}
