// =============================================================================
//! - Component for the speed button
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-03
//! - Since: 2022-12-20
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::functions::web_sys::add_click_handler_by_id;
use crate::traits::InputWriter;
use futures::channel::mpsc::UnboundedReceiver;

pub struct SpeedComponent {
  pub id: String,
  pub unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl SpeedComponent {
  pub fn init(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }

  pub fn new(id: &str) -> Self {
    Self {
      id: String::from(id),
      unbounded_receiver: None,
    }
  }

  pub fn make_html(&self) -> String {
    format!("<button id=\"{}\">Speed</button>", self.id)
  }

  pub fn pressed(&mut self) -> bool {
    if self.unbounded_receiver.is_none() {
      return false;
    }
    matches!(
      self.unbounded_receiver.as_mut().unwrap().try_next(),
      Ok(Some(()))
    )
  }

  pub fn update<I: InputWriter>(
    &mut self,
    input: &mut I,
  ) {
    if self.pressed() {
      input.request_speed();
    }
  }
}
