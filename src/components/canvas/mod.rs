// =============================================================================
//! - Component for the HTML Canvas
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-18
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub struct CanvasComponent<const G: usize> {
  pub id: String,
}

impl<const G: usize> CanvasComponent<G> {
  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
    }
  }

  pub fn make_html(&self) -> String {
    format!(
      "<canvas id=\"{}\" height=\"600\" width=\"600\"></canvas>",
      self.id
    )
  }
}
