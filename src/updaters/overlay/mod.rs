// =============================================================================
//! - Overlay Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-09
//! - Updated: 2023-02-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{GENES_MAX, OVERLAY_REFRESH_PERIOD_MILLIS};
use crate::engine::update_timer::UpdateTimer;
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub trait OverlayUpdaterInputs {
  fn get_update_time_millis(&self) -> f64;
}

pub struct OverlayUpdater {
  clock: Rc<RefCell<Clock>>,
  fauna: Rc<RefCell<Fauna>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  overlay: Rc<RefCell<Overlay>>,
  update_timer: UpdateTimer,
}

impl OverlayUpdater {
  fn make_genes_average_string(&self) -> String {
    let mut gene_x_string = String::from("X:");
    let mut gene_y_string = String::from("Y:");
    let mut bugs_alive: usize = 0;
    for bug in self.fauna.borrow().bugs.iter() {
      if bug.energy > 0 {
        bugs_alive += 1;
      }
    }
    for i in 0..GENES_MAX {
      let mut x_sum: usize = 0;
      let mut y_sum: usize = 0;
      for bug in self.fauna.borrow().bugs.iter() {
        if bug.energy > 0 {
          if bug.genes_x[i] {
            x_sum += 1;
          }
          if bug.genes_y[i] {
            y_sum += 1;
          }
        }
      }
      if x_sum as f64 / bugs_alive as f64 >= 0.5 {
        gene_x_string.push('1');
      } else {
        gene_x_string.push('0');
      }
      if y_sum as f64 / bugs_alive as f64 >= 0.5 {
        gene_y_string.push('1');
      } else {
        gene_y_string.push('0');
      }
    }
    let mut result = String::from(&gene_x_string);
    result.push(' ');
    result.push_str(&gene_y_string);
    result
  }

  fn make_status_string(&self) -> String {
    let genes_average_string = self.make_genes_average_string();
    let bugs_alive = self.fauna.borrow().bugs.iter().fold(0, |count, bug| {
      if bug.energy > 0 {
        count + 1
      } else {
        count
      }
    });
    let time = self.clock.borrow().time;
    format!(
      "Average Movement Genes {} Time:{} Alive:{}",
      genes_average_string, time, bugs_alive,
    )
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    fauna: Rc<RefCell<Fauna>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let update_timer = UpdateTimer {
      update_period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      update_time_millis_next: 0.,
    };
    Self {
      clock,
      fauna,
      inputs,
      overlay,
      update_timer,
    }
  }
}

impl Updater for OverlayUpdater {
  fn update(&mut self) {
    let update_time_millis = self.inputs.borrow().get_update_time_millis();
    if self.update_timer.before_next_update_time(update_time_millis) {
      return;
    }
    self.overlay.borrow_mut().status_string = self.make_status_string();
  }
}
