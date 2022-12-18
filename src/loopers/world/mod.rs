// =============================================================================
//! - WorldLooper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-18
//! - Rust since: 2022-12-15
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

use crate::components::evolve::EvolveComponent;
use crate::functions::wasm_bindgen::closure_wrap;
use crate::models::world::World;
use crate::painters::world::WorldPainter;
use crate::updaters::world::WorldUpdater;
use anyhow::{anyhow, Result};
use futures::Future;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Window;

pub struct WorldLooper<const G: usize> {
  pub evolve_component: EvolveComponent<G>,
  pub last_update_time: f64,
  pub world: World<G>,
  pub world_painter: WorldPainter<G>,
  pub world_updater: WorldUpdater<G>,
}

impl<const G: usize> WorldLooper<G> {
  pub fn loop_once(&mut self) {
    self.evolve_component.update(&mut self.world);
    self.world_updater.update(&mut self.world);
    self.world_painter.paint(&self.world);
  }

  pub fn spawn_local<F>(future: F)
  where
    F: Future<Output = ()> + 'static,
  {
    wasm_bindgen_futures::spawn_local(future);
  }

  pub async fn start(
    evolve_component: EvolveComponent<G>,
    world: World<G>,
    world_painter: WorldPainter<G>,
    world_updater: WorldUpdater<G>,
  ) -> Result<()> {
    let last_update_time = window()?
      .performance()
      .ok_or_else(|| anyhow!("Performance object not found"))?
      .now();
    let mut world_looper = WorldLooper {
      evolve_component,
      last_update_time,
      world,
      world_painter,
      world_updater,
    };
    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(create_raf_closure(move |performance: f64| {
      if performance - world_looper.last_update_time > 1_000.0 {
        world_looper.last_update_time = performance;
        world_looper.loop_once();
      }
      let _result: Result<i32, anyhow::Error> =
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(
      g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
    )?;
    Ok(())
  }
}

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
  closure_wrap(Box::new(f))
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
  window()?
    .request_animation_frame(callback.as_ref().unchecked_ref())
    .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

pub fn window() -> Result<Window> {
  web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;
