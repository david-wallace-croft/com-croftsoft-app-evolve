// =============================================================================
//! - WorldLooper for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-19
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
use crate::constants::FRAME_PERIOD_MILLIS;
use crate::functions::web_sys::{get_window, request_animation_frame};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;

type LoopClosure = Closure<dyn FnMut(f64)>;

pub struct WorldLooper<const G: usize> {
  pub evolve_component: EvolveComponent<G>,
  pub last_update_time: f64,
}

impl<const G: usize> WorldLooper<G> {
  pub async fn start() -> Result<()> {
    let mut evolve_component = EvolveComponent::<G>::new("evolve");
    evolve_component.init();
    let last_update_time = get_window()?
      .performance()
      .ok_or_else(|| anyhow!("Performance object not found"))?
      .now();
    let mut world_looper = WorldLooper {
      evolve_component,
      last_update_time,
    };
    let f: Rc<RefCell<Option<LoopClosure>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |performance: f64| {
      if performance - world_looper.last_update_time > FRAME_PERIOD_MILLIS {
        world_looper.last_update_time = performance;
        world_looper.evolve_component.update();
      }
      let _result: Result<i32, anyhow::Error> =
        request_animation_frame(f.borrow().as_ref().unwrap());
    })));
    request_animation_frame(
      g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
    )?;
    Ok(())
  }
}
