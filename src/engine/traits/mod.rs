// =============================================================================
//! - Traits for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-07
//! - Since: 2023-01-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use web_sys::CanvasRenderingContext2d;

use crate::engine::input::Input;
use crate::models::world::World;

// TODO: Maybe merge with WorldPainter
pub trait CanvasPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
    world: &World,
  );
}

pub trait Component {
  fn init(&mut self);

  fn make_html(&self) -> String;

  fn new(id: &str) -> Self;

  fn update(
    &mut self,
    input: &mut Input,
  );
}

pub trait Model {
  fn update(
    &mut self,
    input: &Input,
  );
}

pub trait WorldPainter {
  fn paint(
    &self,
    world: &World,
  );
}
