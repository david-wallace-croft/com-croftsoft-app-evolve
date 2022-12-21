// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-20
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
use super::eden::EdenComponent;
use super::reset::ResetComponent;
use crate::functions::web_sys::{get_window, spawn_local_loop};
use crate::models::world::World;
use crate::updaters::world::WorldUpdater;
use web_sys::{Document, HtmlCollection};

pub struct EvolveComponentConfiguration {
  pub frame_period_millis: f64,
}

pub struct EvolveComponent {
  blight_component: BlightComponent,
  canvas_component: CanvasComponent,
  eden_component: EdenComponent,
  frame_period_millis: f64,
  next_update_time: f64,
  reset_component: ResetComponent,
  world: World,
  world_updater: WorldUpdater,
}

impl EvolveComponent {
  pub fn init(&mut self) {
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
    self.eden_component.init();
    self.reset_component.init();
  }

  pub fn make_html(&self) -> String {
    let blight_html: String = self.blight_component.make_html();
    let canvas_html: String = self.canvas_component.make_html();
    let eden_html: String = self.eden_component.make_html();
    let reset_html: String = self.reset_component.make_html();
    [
      String::from("<div id=\"evolve\">"),
      canvas_html,
      String::from("<br>"),
      blight_html,
      eden_html,
      reset_html,
      String::from("</div>"),
    ]
    .join("\n")
  }

  pub fn new(config: EvolveComponentConfiguration) -> Self {
    let EvolveComponentConfiguration {
      frame_period_millis,
    } = config;
    Self {
      blight_component: BlightComponent::new("blight"),
      canvas_component: CanvasComponent::new("canvas"),
      eden_component: EdenComponent::new("eden"),
      frame_period_millis,
      next_update_time: 0.0,
      reset_component: ResetComponent::new("reset"),
      world: World::default(),
      world_updater: WorldUpdater::default(),
    }
  }

  pub fn start(self) {
    spawn_local_loop(self);
  }

  pub fn update(
    &mut self,
    update_time: f64,
  ) {
    if update_time < self.next_update_time {
      return;
    }
    self.next_update_time = update_time + self.frame_period_millis;
    self.blight_component.update(&mut self.world);
    self.eden_component.update(&mut self.world);
    self.reset_component.update(&mut self.world);
    self.world_updater.update(&mut self.world);
    self.canvas_component.paint(&self.world);
  }
}
