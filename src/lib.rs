// =============================================================================
//! - Main function for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-09-12
//! - Updated: 2023-03-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#![allow(clippy::uninlined_format_args)]

use com_croftsoft_lib_animation::web_sys::log;
use constants::INFO;
use engine::looper::Looper;
use wasm_bindgen::prelude::*;

mod components;
mod constants;
mod engine;
mod messages;
mod models;
mod painters;
mod updaters;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Looper::launch();
  Ok(())
}
