// =============================================================================
//! - CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-10
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

#![allow(dead_code)]
#![allow(unused_imports)]

use js_sys::Object;
use models::world::structures::World;
use painters::world::WorldPainter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlDivElement;
use wee_alloc::WeeAlloc;

mod models;
mod painters;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  let document: Document = window().unwrap().document().unwrap();
  // hello_canvas(&document);
  hello_console();
  // hello_div(&document);
  start(&document);
  Ok(())
}

// fn hello_canvas(document: &Document) {
//   let element: Element = document.get_element_by_id("canvas").unwrap();
//   let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
//   let object: Object = html_canvas_element.get_context("2d").unwrap().unwrap();
//   let canvas_context: CanvasRenderingContext2d = object.dyn_into().unwrap();
//   canvas_context.set_font("normal 14px serif");
//   canvas_context.stroke_text("Hello, Canvas!", 0.0, 14.0).unwrap();
// }

fn hello_console() {
  console::log_1(&JsValue::from_str("Hello, Console!"));
}

// fn hello_div(document: &Document) {
//   let element: Element = document.get_element_by_id("div").unwrap();
//   let html_div_element: HtmlDivElement = element.dyn_into().unwrap();
//   html_div_element.insert_adjacent_text("afterbegin", "Hello, Div!").unwrap();
// }

fn start(document: &Document) {
  let element: Element = document.get_element_by_id("canvas").unwrap();
  let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
  let object: Object = html_canvas_element.get_context("2d").unwrap().unwrap();
  let context: CanvasRenderingContext2d = object.dyn_into().unwrap();
  context.set_font("normal 14px serif");
  context.stroke_text("Hello, Canvas!", 0.0, 14.0).unwrap();
  let canvas_height: f64 = html_canvas_element.height() as f64;
  let canvas_width: f64 = html_canvas_element.width() as f64;
  let mut world = World::<8>::default();
  world.reset();
  let world_painter =
    WorldPainter::new(canvas_height, canvas_width, &context, &world);
  world_painter.paint();
}
