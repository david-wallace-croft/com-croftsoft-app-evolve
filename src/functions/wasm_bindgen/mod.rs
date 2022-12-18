// =============================================================================
//! - wasm-bindgen functions for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-18
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::Closure;

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
  Closure::wrap(data)
}
