// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-18
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

use web_sys::{Document, HtmlCollection};

use super::blight::BlightComponent;
use super::canvas::CanvasComponent;
use super::eden::EdenComponent;
use super::reset::ResetComponent;
use crate::loopers::world::window;
use crate::models::world::World;

pub struct EvolveComponent<const G: usize> {
  blight_component: BlightComponent<G>,
  canvas_component: CanvasComponent<G>,
  eden_component: EdenComponent<G>,
  id: String,
  reset_component: ResetComponent<G>,
}

impl<const G: usize> EvolveComponent<G> {
  pub fn init(&mut self) {
    let document: Document = window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-evolve");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let evolve_html: String = self.make_html();
    let _result = element.insert_adjacent_html("afterbegin", &evolve_html);
    self.blight_component.init();
    self.eden_component.init();
    self.reset_component.init();
  }

  pub fn make_html(&self) -> String {
    let blight_html: String = self.blight_component.make_html();
    let canvas_html: String = self.canvas_component.make_html();
    let eden_html: String = self.eden_component.make_html();
    let reset_html: String = self.reset_component.make_html();
    [
      format!("<div id={}>", self.id),
      canvas_html,
      String::from("<br>"),
      blight_html,
      eden_html,
      reset_html,
      String::from("</div>"),
    ]
    .join("\n")
  }

  pub fn new(id: &str) -> Self {
    Self {
      blight_component: BlightComponent::<G>::new("blight"),
      canvas_component: CanvasComponent::<G>::new("canvas"),
      eden_component: EdenComponent::<G>::new("eden"),
      id: String::from(id),
      reset_component: ResetComponent::<G>::new("reset"),
    }
  }

  pub fn update(
    &mut self,
    world: &mut World<G>,
  ) {
    self.blight_component.update(world);
    self.eden_component.update(world);
    self.reset_component.update(world);
  }
}
