use crate::models::world::World;
use crate::painters::world::WorldPainter;
use crate::updaters::world::WorldUpdater;
use anyhow::{anyhow, Result};
use futures::Future;
use std::{cell::RefCell, rc::Rc, sync::Mutex};
use wasm_bindgen::{
  closure::{WasmClosure, WasmClosureFnOnce},
  prelude::Closure,
  JsCast, JsValue,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, Window};

pub struct WorldLooper<const G: usize> {
  pub world: World<G>,
  pub world_painter: WorldPainter<G>,
  pub world_updater: WorldUpdater<G>,
}

impl<const G: usize> WorldLooper<G> {
  pub fn loop_once(&mut self) {
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
    world: World<G>,
    world_painter: WorldPainter<G>,
    world_updater: WorldUpdater<G>,
  ) -> Result<()> {
    let mut world_looper = WorldLooper {
      world,
      world_painter,
      world_updater,
    };
    let f: SharedLoopClosure = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(create_raf_closure(move |perf: f64| {
      world_looper.loop_once();
      let _result: Result<i32, anyhow::Error> =
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(
      g.borrow().as_ref().ok_or_else(|| anyhow!("loop failed"))?,
    )?;
    Ok(())
  }
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
  Closure::wrap(data)
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
