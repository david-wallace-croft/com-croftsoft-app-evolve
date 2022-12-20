// =============================================================================
//! - wasm-bindgen functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-19
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::cell::RefCell;
use futures::Future;
use std::rc::Rc;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::Closure;

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
  Closure::wrap(data)
}

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
  closure_wrap(Box::new(f))
}

pub fn spawn_local<F>(future: F)
where
  F: Future<Output = ()> + 'static,
{
  wasm_bindgen_futures::spawn_local(future);
}
