// =============================================================================
//! - Component for the Evolve application
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-17
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
use super::eden::EdenComponent;

pub struct EvolveComponent<const G: usize>;

impl<const G: usize> EvolveComponent<G> {
  pub fn make_html(canvas_element_id: &str) -> String {
    // let blight_id: &str = &[
    //   String::from(canvas_element_id),
    //   String::from("blight"),
    // ]
    // .join("-");
    let canvas_html: String = format!(
      "<canvas id=\"{}\" height=\"600\" width=\"600\"></canvas>",
      canvas_element_id
    );
    let blight_html: String = BlightComponent::<G>::make_html();
    let eden_html: String = EdenComponent::<G>::make_html();
    [
      canvas_html,
      String::from("<br>"),
      blight_html,
      eden_html,
    ]
    .join("\n")
  }
}
