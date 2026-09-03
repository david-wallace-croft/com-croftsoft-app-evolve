// =============================================================================
//! - OverlayPainter for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-10
//! - Updated: 2026-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::options::Options;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

const FILL_STYLE_WHITE: &str = "white";

pub struct OverlayPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: &'static str,
  options: Rc<RefCell<Options>>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    options: Rc<RefCell<Options>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    Self {
      context,
      fill_style: FILL_STYLE_WHITE,
      options,
      overlay,
    }
  }
}

impl Painter for OverlayPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style_str(self.fill_style);
    context.set_font("bold 17px monospace");
    let overlay: Ref<Overlay> = self.overlay.borrow();
    context.fill_text(&overlay.status_string, 4., 17.).unwrap();
    let options = self.options.borrow();
    if options.update_rate_display && !options.pause {
      context
        .fill_text(&overlay.update_rate_string, 4., 34.)
        .unwrap();
    }
    if options.time_display {
      context.fill_text(&overlay.time_string, 4., 51.).unwrap();
    }
  }
}
