// =============================================================================
//! - WorldPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-12
//! - Rust since: 2022-11-27
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

use crate::components::blight::BlightComponent;
use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::world::World;
use crate::painters::background::BackgroundPainter;
use crate::painters::bugs::BugsPainter;
use crate::painters::flora::FloraPainter;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement,
  HtmlCollection,
};

use super::overlay::OverlayPainter;

pub struct WorldPainter<const G: usize> {
  pub background_painter: BackgroundPainter,
  pub bugs_painter: BugsPainter<G>,
  pub context: CanvasRenderingContext2d,
  pub flora_painter: FloraPainter<G>,
  pub overlay_painter: OverlayPainter<G>,
}

impl<const G: usize> WorldPainter<G> {
  pub fn new(canvas_element_id: &str) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-evolve");
    let element_option = html_collection.item(0);
    let blight_id: &str = &[
      String::from(canvas_element_id),
      String::from("blight"),
    ]
    .join("-");
    if let Some(element) = element_option {
      let canvas_html: String = format!(
        "<canvas id=\"{}\" height=\"600\" width=\"600\"></canvas>",
        canvas_element_id
      );
      let button_html: String = BlightComponent::make_html(blight_id);
      let html: String = [
        canvas_html,
        String::from("<br>"),
        button_html,
      ]
      .join("\n");
      let _result = element.insert_adjacent_html("afterbegin", &html);
    }
    // TODO: move the HTML stuff to an AppComponent
    let blight_component: BlightComponent =
      BlightComponent::initialize(blight_id).unwrap();
    let element: Element =
      document.get_element_by_id(canvas_element_id).unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let canvas_height: f64 = html_canvas_element.height() as f64;
    let canvas_width: f64 = html_canvas_element.width() as f64;
    let background_painter =
      BackgroundPainter::new(canvas_height, canvas_width);
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let bugs_painter = BugsPainter::new(scale_x, scale_y);
    let flora_painter = FloraPainter::new(scale_x, scale_y);
    let overlay_painter = OverlayPainter::default();
    Self {
      background_painter,
      bugs_painter,
      context,
      flora_painter,
      overlay_painter,
    }
  }

  pub fn paint(
    &self,
    world: &World<G>,
  ) {
    self.background_painter.paint(&self.context);
    self.flora_painter.paint(&self.context, world);
    self.bugs_painter.paint(&self.context, world);
    self.overlay_painter.paint(&self.context, world);
  }
}
