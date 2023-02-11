// =============================================================================
//! - OverlayPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-10
//! - Updated: 2023-02-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::CanvasPainter;
use crate::models::frame_rate::FrameRate;
use crate::models::overlay::Overlay;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  frame_rate: Rc<RefCell<FrameRate>>,
  overlay: Rc<RefCell<Overlay>>,
  fill_style: JsValue,
}

impl OverlayPainter {
  pub fn new(
    frame_rate: Rc<RefCell<FrameRate>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
      frame_rate,
      overlay,
    }
  }
}

impl CanvasPainter for OverlayPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    let overlay: Ref<Overlay> = self.overlay.borrow();
    let frame_rate_string = &overlay.frame_rate_string;
    let status_string = &overlay.status_string;
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    context.fill_text(status_string, 4.0, 17.0).unwrap();
    if self.frame_rate.borrow().display {
      context.fill_text(frame_rate_string, 4.0, 34.0).unwrap();
    }
  }
}
