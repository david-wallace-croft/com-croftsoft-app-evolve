// =============================================================================
//! - Frame Rate for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-02
//! - Since: 2023-02-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub struct FrameRate {
  pub frame_count: usize,
  pub frame_period_millis: f64,
  pub update_time_millis_next: f64,
}

impl FrameRate {
  pub fn before_next_update_time(
    &mut self,
    update_time_millis: f64,
  ) -> bool {
    if update_time_millis < self.update_time_millis_next {
      return true;
    }
    self.frame_count = self.frame_count.saturating_add(1);
    self.update_time_millis_next =
      update_time_millis + self.frame_period_millis;
    false
  }

  pub fn new(frame_period_millis: f64) -> Self {
    Self {
      frame_count: 0,
      frame_period_millis,
      update_time_millis_next: 0.,
    }
  }
}
