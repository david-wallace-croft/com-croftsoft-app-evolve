// =============================================================================
//! - Component for the HTML Canvas
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::world::World;
use crate::painters::world::WorldPainter;

pub struct CanvasComponent {
  pub id: String,
  pub world_painter_option: Option<WorldPainter>,
}

impl CanvasComponent {
  pub fn init(&mut self) {
    self.world_painter_option = Some(WorldPainter::new("canvas"));
  }

  pub fn make_html(&self) -> String {
    format!(
      "<canvas id=\"{}\" height=\"600\" width=\"600\"></canvas>",
      self.id
    )
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      world_painter_option: None,
    }
  }

  pub fn paint(
    &self,
    world: &World,
  ) {
    if let Some(world_painter) = &self.world_painter_option {
      world_painter.paint(world);
    }
  }
}
