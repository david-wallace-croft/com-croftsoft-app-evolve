// =============================================================================
//! - Input model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-12-31
//! - Since: 2022-12-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Debug)]
pub struct Input {
  pub blight: bool,
  pub bug: Option<usize>,
  // TODO: why is this an Option instead of a simple bool?
  pub garden: Option<bool>,
  pub flora: Option<usize>,
  pub reset: bool,
  pub speed: bool,
}

impl Default for Input {
  fn default() -> Self {
    Self {
      blight: false,
      bug: None,
      garden: None,
      flora: None,
      reset: true,
      speed: false,
    }
  }
}
