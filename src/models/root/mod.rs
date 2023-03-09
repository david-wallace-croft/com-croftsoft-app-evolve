// =============================================================================
//! - Root Model for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-10
//! - Updated: 2023-03-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::clock::Clock;
use super::fauna::Fauna;
use super::flora::Flora;
use super::overlay::Overlay;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Root {
  pub clock: Rc<RefCell<Clock>>,
  pub fauna: Rc<RefCell<Fauna>>,
  pub flora: Rc<RefCell<Flora>>,
  pub overlay: Rc<RefCell<Overlay>>,
}
