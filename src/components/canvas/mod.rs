// =============================================================================
//! - Component for the HTML Canvas
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-26
//! - Rust since: 2022-12-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::functions::location::to_index_from_xy;
use crate::functions::web_sys::{
  add_mouse_down_handler_by_id, get_html_canvas_element_by_id, log,
};
use crate::models::world::World;
use crate::painters::world::WorldPainter;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use web_sys::{DomRect, HtmlCanvasElement, MouseEvent};

pub struct CanvasComponent {
  pub id: String,
  pub unbounded_receiver_option: Option<UnboundedReceiver<MouseEvent>>,
  pub world_painter_option: Option<WorldPainter>,
}

impl CanvasComponent {
  pub fn get_scale_xy(canvas_element_id: &str) -> (f64, f64) {
    let html_canvas_element: HtmlCanvasElement =
      get_html_canvas_element_by_id(canvas_element_id);
    let canvas_height = html_canvas_element.height();
    let canvas_width = html_canvas_element.width();
    let scale_x = canvas_width as f64 / SPACE_WIDTH as f64;
    let scale_y = canvas_height as f64 / SPACE_HEIGHT as f64;
    (scale_x, scale_y)
  }

  pub fn init(&mut self) {
    self.unbounded_receiver_option = add_mouse_down_handler_by_id(&self.id);
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
      unbounded_receiver_option: None,
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

  pub fn update(
    &mut self,
    world: &mut World,
  ) {
    let mouse_event_option = self.poll_mouse_event();
    if let Some(mouse_event) = mouse_event_option {
      let client_x: f64 = mouse_event.client_x() as f64;
      let client_y: f64 = mouse_event.client_y() as f64;
      let html_canvas_element: HtmlCanvasElement =
        get_html_canvas_element_by_id(&self.id);
      let dom_rect: DomRect = html_canvas_element.get_bounding_client_rect();
      let scale_x = html_canvas_element.width() as f64 / dom_rect.width();
      let scale_y = html_canvas_element.height() as f64 / dom_rect.height();
      let x: i32 = ((client_x - dom_rect.left()) * scale_x) as i32;
      let y: i32 = ((client_y - dom_rect.top()) * scale_y) as i32;
      let index = self.to_world_index_from_canvas_xy(x, y);
      log(&format!("x {} y {} index {}", x, y, index));
      world.requested_bug = Some(index);
    }
  }

  // private methods

  fn poll_mouse_event(&mut self) -> Option<MouseEvent> {
    let unbounded_receiver: &mut UnboundedReceiver<MouseEvent> =
      self.unbounded_receiver_option.as_mut()?;
    let result: Result<Option<MouseEvent>, TryRecvError> =
      unbounded_receiver.try_next();
    if let Ok(mouse_event_option) = result {
      return mouse_event_option;
    }
    None
  }

  fn to_world_index_from_canvas_xy(
    &self,
    canvas_x: i32,
    canvas_y: i32,
  ) -> usize {
    let (scale_x, scale_y) = CanvasComponent::get_scale_xy(&self.id);
    let mut scaled_canvas_x: f64 = canvas_x as f64 / scale_x;
    let mut scaled_canvas_y: f64 = canvas_y as f64 / scale_y;
    if scaled_canvas_x < 0. {
      scaled_canvas_x = 0.;
    }
    if scaled_canvas_y < 0. {
      scaled_canvas_y = 0.;
    }
    let mut world_x = scaled_canvas_x as usize;
    let mut world_y = scaled_canvas_y as usize;
    if world_x >= SPACE_WIDTH {
      world_x = SPACE_WIDTH - 1;
    }
    if world_y >= SPACE_HEIGHT {
      world_y = SPACE_HEIGHT - 1;
    }
    to_index_from_xy(world_x, world_y)
  }
}
