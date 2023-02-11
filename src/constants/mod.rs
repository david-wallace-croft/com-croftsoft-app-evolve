// =============================================================================
//! - Constants for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-11-27
//! - Updated: 2023-02-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::configuration::Configuration;

pub static INFO: &str =
  "CroftSoft Evolve v0.8.5-SNAPSHOT Copyright 2022-2023 CroftSoft Inc";

pub const BABY_ENERGY: usize = 10;
pub const BIRTH_ENERGY: usize = 30;
pub const BIRTH_ENERGY_COST: usize = 20;
pub const BUGS_MAX: usize = 10_000;
pub const EDEN_HEIGHT: usize = 2;
pub const EDEN_WIDTH: usize = 2;
pub const EDEN_X0: usize = (SPACE_WIDTH - EDEN_WIDTH) / 2;
pub const EDEN_X1: usize = EDEN_X0 + EDEN_WIDTH - 1;
pub const EDEN_Y0: usize = (SPACE_WIDTH - EDEN_WIDTH) / 2;
pub const EDEN_Y1: usize = EDEN_Y0 + EDEN_HEIGHT - 1;
pub const FLORA_ENERGY: usize = 20;
pub const FRAMES_PER_SECOND: f64 = 1.0;
pub const GENES_MAX: usize = 8;
pub const FLORA_GROWTH_RATE_INIT: usize = 10;
pub const FLORA_GROWTH_RATE_MAX: usize = 20;
pub const LOCATION_COUNT: usize = SPACE_HEIGHT * SPACE_WIDTH;
pub const MAX_ENERGY: usize = 60;
pub const MOVE_COST: usize = 1;
pub const OVERLAY_REFRESH_PERIOD_MILLIS: f64 = 1_000.;
pub const PAINT_OFFSET: f64 = (1. - PAINT_SCALE) / 2.;
pub const PAINT_SCALE: f64 = 0.5;
pub const SPACE_HEIGHT: usize = 100;
pub const SPACE_WIDTH: usize = 100;

pub const CONFIGURATION: Configuration = Configuration {
  frame_period_millis: 1_000.0 / FRAMES_PER_SECOND,
};
