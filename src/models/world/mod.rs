// =============================================================================
//! - World Model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-25
//! - Since: 2022-12-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::clock::Clock;
use super::fauna::Fauna;
use super::flora::Flora;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
  pub clock: Rc<RefCell<Clock>>,
  pub fauna: Rc<RefCell<Fauna>>,
  pub flora: Rc<RefCell<Flora>>,
}
