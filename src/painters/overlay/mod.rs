// =============================================================================
//! - OverlayPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-10
//! - Updated: 2023-02-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::CanvasPainter;
use crate::models::overlay::Overlay;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  overlay: Rc<RefCell<Overlay>>,
  fill_style: JsValue,
}

impl OverlayPainter {
  pub fn new(overlay: Rc<RefCell<Overlay>>) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
      overlay,
    }
  }
}

impl CanvasPainter for OverlayPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    let status_string = &self.overlay.borrow().status_string;
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    context.fill_text(status_string, 4.0, 17.0).unwrap();
  }
}
