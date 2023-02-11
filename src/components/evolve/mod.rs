// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-12-17
//! - Updated: 2023-02-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::blight::BlightComponent;
use super::canvas::CanvasComponent;
use super::flora::FloraComponent;
use super::frame_rate::FrameRateComponent;
use super::garden::GardenComponent;
use super::reset::ResetComponent;
use super::speed::SpeedComponent;
use crate::engine::functions::web_sys::get_window;
use crate::engine::traits::Component;
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::models::frame_rate::FrameRate;
use crate::models::world::World;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct EvolveComponent {
  blight_component: Rc<RefCell<BlightComponent>>,
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 7],
  events: Rc<RefCell<Events>>,
  flora_component: Rc<RefCell<FloraComponent>>,
  frame_rate_component: Rc<RefCell<FrameRateComponent>>,
  garden_component: Rc<RefCell<GardenComponent>>,
  reset_component: Rc<RefCell<ResetComponent>>,
  speed_component: Rc<RefCell<SpeedComponent>>,
}

impl EvolveComponent {
  // TODO: do something with the ID
  pub fn new(
    events: Rc<RefCell<Events>>,
    frame_rate: Rc<RefCell<FrameRate>>,
    _id: &str,
    inputs: Rc<RefCell<Inputs>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let blight_component =
      Rc::new(RefCell::new(BlightComponent::new("blight", inputs.clone())));
    let canvas_component = Rc::new(RefCell::new(CanvasComponent::new(
      frame_rate,
      "canvas",
      inputs.clone(),
      world,
    )));
    let flora_component =
      Rc::new(RefCell::new(FloraComponent::new("flora", inputs.clone())));
    let frame_rate_component = Rc::new(RefCell::new(FrameRateComponent::new(
      "frame-rate",
      inputs.clone(),
    )));
    let garden_component =
      Rc::new(RefCell::new(GardenComponent::new("garden", inputs.clone())));
    let reset_component =
      Rc::new(RefCell::new(ResetComponent::new("reset", inputs.clone())));
    let speed_component =
      Rc::new(RefCell::new(SpeedComponent::new("speed", inputs)));
    let components: [Rc<RefCell<dyn Component>>; 7] = [
      blight_component.clone(),
      canvas_component.clone(),
      flora_component.clone(),
      frame_rate_component.clone(),
      garden_component.clone(),
      reset_component.clone(),
      speed_component.clone(),
    ];
    Self {
      blight_component,
      canvas_component,
      components,
      events,
      flora_component,
      frame_rate_component,
      garden_component,
      reset_component,
      speed_component,
    }
  }
}

impl Component for EvolveComponent {
  fn make_html(&self) -> String {
    let blight_html: String = self.blight_component.borrow().make_html();
    let canvas_html: String = self.canvas_component.borrow().make_html();
    let flora_html: String = self.flora_component.borrow().make_html();
    let frame_rate_html: String =
      self.frame_rate_component.borrow().make_html();
    let garden_html: String = self.garden_component.borrow().make_html();
    let reset_html: String = self.reset_component.borrow().make_html();
    let speed_html: String = self.speed_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"evolve\">"),
      canvas_html,
      String::from("<br>"),
      reset_html,
      blight_html,
      flora_html,
      garden_html,
      String::from("<br>"),
      speed_html,
      frame_rate_html,
      String::from("</div>"),
    ]
    .join("\n")
  }
}

impl Initializer for EvolveComponent {
  fn initialize(&mut self) {
    let document: Document = get_window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-evolve");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let evolve_html: String = self.make_html();
    // TODO: Remove existing child nodes
    let _result = element.insert_adjacent_html("afterbegin", &evolve_html);
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().initialize());
  }
}

impl Painter for EvolveComponent {
  fn paint(&self) {
    if !self.events.borrow().updated_world {
      return;
    }
    self.canvas_component.borrow().paint();
  }
}

impl Updater for EvolveComponent {
  fn update(&mut self) {
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().update());
  }
}
