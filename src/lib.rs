// =============================================================================
//! - CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
//! - Rust since: 2022-09-12
//! - Java version: 2008-04-19
//! - Java since: 1996-09-01
//!
//! # History
//! - Adapted from the Java package com.croftsoft.apps.evolve
//!   - In the Java-based [`CroftSoft Apps Library`]
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use constants::INFO;
use functions::web_sys::{log, spawn_local_loop};
use loopers::world::WorldLooper;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

mod components;
mod constants;
mod functions;
mod loopers;
mod models;
mod painters;
mod updaters;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  let mut world_looper = WorldLooper::default();
  world_looper.init();
  spawn_local_loop(world_looper);
  Ok(())
}
