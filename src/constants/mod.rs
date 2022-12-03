// =============================================================================
//! - Constants for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 1996-2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-12-03
//! - Rust since: 2022-11-27
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

#![allow(dead_code)]

pub static FONT_NAME: &str = "TimesRoman";
pub static FRAME_ICON_FILENAME: &str = "/images/david.png";
pub static SHUTDOWN_CONFIRMATION_PROMPT: &str = "Close CroftSoft Evolve?";
pub static TITLE: &str = "CroftSoft Evolve";
pub static VERSION: &str = "0.1.0";

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
// TODO: FONT_STYLE bold
pub const FRAME_RATE: f64 = 1.0;
// TODO: FRAME_SIZE
pub const GENES_MAX: usize = 8;
pub const INIT_GROWTH_RATE: usize = 10;
pub const MAX_ENERGY: usize = 60;
pub const MAX_GROWTH_RATE: usize = SPACE_HEIGHT * SPACE_WIDTH;
pub const MIN_GROWTH_RATE: usize = 0;
pub const MOVE_COST: usize = 1;
pub const SPACE_HEIGHT: usize = 100;
pub const SPACE_WIDTH: usize = 100;
pub const SPINNER_STEP_SIZE: usize = 1;
pub const TEXT_MARGIN: usize = 10;
