// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-02-05
//! - Rust since: 2022-12-17
//! - Java version: 2008-04-19
//! - Java since: 1996-09-01
//!
//! # History
//! - Adapted from the Java package com.croftsoft.apps.evolve
//!   - In the Java-based [`CroftSoft Apps Library`]
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::blight::BlightComponent;
use super::canvas::CanvasComponent;
use super::flora::FloraComponent;
use super::garden::GardenComponent;
use super::reset::ResetComponent;
use super::speed::SpeedComponent;
use crate::engine::frame_rater::FrameRater;
use crate::engine::functions::web_sys::get_window;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use crate::models::world::World;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct EvolveComponent {
  blight_component: Rc<RefCell<BlightComponent>>,
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 6],
  flora_component: Rc<RefCell<FloraComponent>>,
  garden_component: Rc<RefCell<GardenComponent>>,
  input: Rc<RefCell<Input>>,
  reset_component: Rc<RefCell<ResetComponent>>,
  speed_component: Rc<RefCell<SpeedComponent>>,
}

impl EvolveComponent {
  // TODO: do something with the ID
  pub fn new(
    _id: &str,
    frame_rater: Rc<RefCell<FrameRater>>,
    input: Rc<RefCell<Input>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let blight_component =
      Rc::new(RefCell::new(BlightComponent::new("blight", input.clone())));
    let canvas_component = Rc::new(RefCell::new(CanvasComponent::new(
      "canvas",
      frame_rater,
      input.clone(),
      world,
    )));
    let flora_component =
      Rc::new(RefCell::new(FloraComponent::new("flora", input.clone())));
    let garden_component =
      Rc::new(RefCell::new(GardenComponent::new("garden", input.clone())));
    let reset_component =
      Rc::new(RefCell::new(ResetComponent::new("reset", input.clone())));
    let speed_component =
      Rc::new(RefCell::new(SpeedComponent::new("speed", input.clone())));
    let components: [Rc<RefCell<dyn Component>>; 6] = [
      blight_component.clone(),
      canvas_component.clone(),
      flora_component.clone(),
      garden_component.clone(),
      reset_component.clone(),
      speed_component.clone(),
    ];
    Self {
      blight_component,
      canvas_component,
      components,
      flora_component,
      garden_component,
      input,
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
    let garden_html: String = self.garden_component.borrow().make_html();
    let reset_html: String = self.reset_component.borrow().make_html();
    let speed_html: String = self.speed_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"evolve\">"),
      canvas_html,
      String::from("<br>"),
      blight_html,
      flora_html,
      garden_html,
      reset_html,
      speed_html,
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
    if !self.input.borrow().updated_world {
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
