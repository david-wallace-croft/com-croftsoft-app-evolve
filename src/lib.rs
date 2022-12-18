// =============================================================================
//! - CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-18
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

use components::evolve::EvolveComponent;
use constants::INFO;
use functions::web_sys::log;
use loopers::world::WorldLooper;
use models::world::World;
use painters::world::WorldPainter;
use updaters::world::WorldUpdater;
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
  WorldLooper::<8>::spawn_local(async move {
    let mut evolve_component = EvolveComponent::<8>::new("evolve");
    evolve_component.init();
    let world_painter = WorldPainter::new("canvas");
    let mut world = World::<8>::default();
    let world_updater = WorldUpdater::default();
    world_updater.reset(&mut world);
    world_painter.paint(&world);
    WorldLooper::<8>::start(
      evolve_component,
      world,
      world_painter,
      world_updater,
    )
    .await
    .expect("loop start failed");
  });
  Ok(())
}
