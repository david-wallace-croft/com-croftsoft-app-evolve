// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 1996-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-04
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
use crate::constants::{FRAME_PERIOD_MILLIS_MINIMUM, INITIAL_CONFIGURATION};
use crate::functions::web_sys::{get_window, spawn_local_loop, LoopUpdater};
use crate::models::input::Input;
use crate::models::world::World;
use crate::traits::{InputReader, InputWriter, Model};
use web_sys::{Document, HtmlCollection};

pub struct EvolveComponentInitialConfiguration {
  pub frame_period_millis: f64,
}

pub struct EvolveComponent {
  blight_component: BlightComponent,
  canvas_component: CanvasComponent,
  flora_component: FloraComponent,
  frame_period_millis: f64,
  garden_component: GardenComponent,
  initial_configuration: EvolveComponentInitialConfiguration,
  input: Input,
  next_update_time: f64,
  reset_component: ResetComponent,
  speed_component: SpeedComponent,
  world: World,
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
    self.flora_component.init();
    self.garden_component.init();
    self.reset_component.init();
    self.speed_component.init();
    self.input.request_reset();
  }

  pub fn launch() {
    let mut evolve_component = EvolveComponent::default();
    evolve_component.init();
    spawn_local_loop(evolve_component);
  }

  pub fn make_html(&self) -> String {
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

  pub fn new(
    initial_configuration: EvolveComponentInitialConfiguration
  ) -> Self {
    let EvolveComponentInitialConfiguration {
      frame_period_millis,
    } = initial_configuration;
    Self {
      blight_component: BlightComponent::new("blight"),
      canvas_component: CanvasComponent::new("canvas"),
      initial_configuration,
      input: Input::default(),
      flora_component: FloraComponent::new("flora"),
      garden_component: GardenComponent::new("garden"),
      frame_period_millis,
      next_update_time: 0.0,
      reset_component: ResetComponent::new("reset"),
      speed_component: SpeedComponent::new("speed"),
      world: World::default(),
    }
  }

  fn update_frame_rate(&mut self) {
    if !self.input.get_speed_toggle_requested() {
      return;
    }
    if self.frame_period_millis == FRAME_PERIOD_MILLIS_MINIMUM {
      self.frame_period_millis = self.initial_configuration.frame_period_millis;
    } else {
      self.frame_period_millis = FRAME_PERIOD_MILLIS_MINIMUM;
    }
  }
}

impl Default for EvolveComponent {
  fn default() -> Self {
    EvolveComponent::new(INITIAL_CONFIGURATION)
  }
}

impl LoopUpdater for EvolveComponent {
  fn update(
    &mut self,
    update_time: f64,
  ) {
    if update_time < self.next_update_time {
      return;
    }
    self.blight_component.update(&mut self.input);
    self.canvas_component.update(&mut self.input);
    self.flora_component.update(&mut self.input);
    self.garden_component.update(&mut self.input);
    self.reset_component.update(&mut self.input);
    self.speed_component.update(&mut self.input);
    self.world.update(&self.input);
    // crate::updaters::world::update(&self.input, &mut self.world);
    self.canvas_component.paint(&self.world);
    self.update_frame_rate();
    self.next_update_time = update_time + self.frame_period_millis;
    self.input.clear();
  }
}
