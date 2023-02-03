// =============================================================================
//! - Frame Rate Painter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-02
//! - Since: 2023-02-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rate::FrameRate;
use crate::engine::traits::CanvasPainter;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct FrameRatePainter {
  fill_style: JsValue,
  frame_rate: Rc<RefCell<FrameRate>>,
}

impl FrameRatePainter {
  fn make_frame_rate_string(&self) -> String {
    format!("Frame: {}", self.frame_rate.borrow().frame_count)
  }

  pub fn new(frame_rate: Rc<RefCell<FrameRate>>) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
      frame_rate,
    }
  }
}

impl CanvasPainter for FrameRatePainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    let frame_rate_string = self.make_frame_rate_string();
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    context.fill_text(&frame_rate_string, 4.0, 34.0).unwrap();
  }
}
