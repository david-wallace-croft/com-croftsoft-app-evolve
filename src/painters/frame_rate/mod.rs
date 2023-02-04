// =============================================================================
//! - Frame Rate Painter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-02-03
//! - Since: 2023-02-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rater::FrameRater;
use crate::engine::traits::CanvasPainter;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct FrameRatePainter {
  fill_style: JsValue,
  frame_rater: Rc<RefCell<FrameRater>>,
}

impl FrameRatePainter {
  fn make_frame_rate_string(&self) -> String {
    format!(
      "Frames per second: {}",
      self.frame_rater.borrow().get_frames_per_second_sampled()
    )
  }

  pub fn new(frame_rater: Rc<RefCell<FrameRater>>) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
      frame_rater,
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
