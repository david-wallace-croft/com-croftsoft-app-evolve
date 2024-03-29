// =============================================================================
//! - Root Painter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-27
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::overlay::OverlayPainter;
use crate::constants::{FILL_STYLE_BACKGROUND, SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::options::Options;
use crate::models::root::Root;
use crate::painters::fauna::FaunaPainter;
use crate::painters::flora::FloraPainter;
use com_croftsoft_lib_animation::painter::background::BackgroundPainter;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use js_sys::Object;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement,
};

pub struct RootPainter {
  painters: Vec<Box<dyn Painter>>,
}

impl RootPainter {
  pub fn new(
    canvas_element_id: &str,
    options: Rc<RefCell<Options>>,
    root_model: &Root,
  ) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let element: Element =
      document.get_element_by_id(canvas_element_id).unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let canvas_context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let context: Rc<RefCell<CanvasRenderingContext2d>> =
      Rc::new(RefCell::new(canvas_context));
    let canvas_height: f64 = html_canvas_element.height() as f64;
    let canvas_width: f64 = html_canvas_element.width() as f64;
    let background_painter = BackgroundPainter::new(
      canvas_height,
      canvas_width,
      context.clone(),
      FILL_STYLE_BACKGROUND,
    );
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let fauna_painter = FaunaPainter::new(
      context.clone(),
      root_model.fauna.clone(),
      scale_x,
      scale_y,
    );
    let flora_painter = FloraPainter::new(
      context.clone(),
      root_model.flora.clone(),
      scale_x,
      scale_y,
    );
    let overlay_painter =
      OverlayPainter::new(context, options, root_model.overlay.clone());
    let painters: Vec<Box<dyn Painter>> = vec![
      Box::new(background_painter),
      Box::new(flora_painter),
      Box::new(fauna_painter),
      Box::new(overlay_painter),
    ];
    Self {
      painters,
    }
  }
}

impl Painter for RootPainter {
  fn paint(&self) {
    self.painters.iter().for_each(|painter| painter.paint());
  }
}
