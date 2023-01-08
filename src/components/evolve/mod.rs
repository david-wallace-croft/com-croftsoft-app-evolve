// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-08
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
use crate::engine::functions::web_sys::get_window;
use crate::engine::input::Input;
use crate::engine::traits::{Component, Painter};
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::flora::Flora;
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct EvolveComponent {
  blight_component: BlightComponent,
  canvas_component: CanvasComponent,
  flora_component: FloraComponent,
  garden_component: GardenComponent,
  reset_component: ResetComponent,
  speed_component: SpeedComponent,
}

impl EvolveComponent {
  // TODO: do something with the ID
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    fauna: Rc<RefCell<Fauna>>,
    flora: Rc<RefCell<Flora>>,
    _id: &str,
  ) -> Self {
    Self {
      blight_component: BlightComponent::new("blight"),
      canvas_component: CanvasComponent::new(clock, fauna, flora, "canvas"),
      flora_component: FloraComponent::new("flora"),
      garden_component: GardenComponent::new("garden"),
      reset_component: ResetComponent::new("reset"),
      speed_component: SpeedComponent::new("speed"),
    }
  }
}

impl Component for EvolveComponent {
  fn init(&mut self) {
    let document: Document = get_window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-evolve");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let evolve_html: String = self.make_html();
    // TODO: remove existing child nodes
    let _result = element.insert_adjacent_html("afterbegin", &evolve_html);
    self.blight_component.init();
    self.canvas_component.init();
    self.flora_component.init();
    self.garden_component.init();
    self.reset_component.init();
    self.speed_component.init();
  }

  fn make_html(&self) -> String {
    let blight_html: String = self.blight_component.make_html();
    let canvas_html: String = self.canvas_component.make_html();
    let flora_html: String = self.flora_component.make_html();
    let garden_html: String = self.garden_component.make_html();
    let reset_html: String = self.reset_component.make_html();
    let speed_html: String = self.speed_component.make_html();
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

  fn update(
    &mut self,
    input: &mut Input,
  ) {
    self.blight_component.update(input);
    self.canvas_component.update(input);
    self.flora_component.update(input);
    self.garden_component.update(input);
    self.reset_component.update(input);
    self.speed_component.update(input);
  }
}

impl Painter for EvolveComponent {
  fn paint(&self) {
    self.canvas_component.paint();
  }
}
