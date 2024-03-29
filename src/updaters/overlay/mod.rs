// =============================================================================
//! - Overlay Updater for CroftSoft Evolve
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-09
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{GENES_MAX, OVERLAY_REFRESH_PERIOD_MILLIS};
use crate::models::clock::Clock;
use crate::models::fauna::Fauna;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use js_sys::Date;
use std::rc::Rc;

pub trait OverlayUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait OverlayUpdaterInputs {
  fn get_bug_requested(&self) -> Option<usize>;
  fn get_current_time_millis(&self) -> f64;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_display_change_requested(&self) -> Option<bool>;
  fn get_time_to_update(&self) -> bool;
  fn get_update_rate_display_change_requested(&self) -> Option<bool>;
}

pub trait OverlayUpdaterOptions {
  fn get_pause(&self) -> bool;
  fn get_time_display(&self) -> bool;
  fn get_update_rate_display(&self) -> bool;
}

pub struct OverlayUpdater {
  clock: Rc<RefCell<Clock>>,
  events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
  fauna: Rc<RefCell<Fauna>>,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  metronome: RefCell<DeltaMetronome>,
  options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
  overlay: Rc<RefCell<Overlay>>,
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

  fn make_time_string(&self) -> String {
    let date: Date = Date::new_0();
    date.to_locale_time_string("en-US").into()
  }

  fn make_update_rate_string(&self) -> String {
    format!(
      "Simulation updates per second: {:.3}",
      self.frame_rater.borrow().get_frames_per_second_sampled()
    )
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
    fauna: Rc<RefCell<Fauna>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let metronome = RefCell::new(DeltaMetronome {
      period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      time_millis_next_tick: 0.,
    });
    Self {
      clock,
      events,
      fauna,
      frame_rater,
      inputs,
      metronome,
      options,
      overlay,
    }
  }

  fn update_overlay(&self) {
    let options: Ref<dyn OverlayUpdaterOptions> = self.options.borrow();
    let mut overlay: RefMut<Overlay> = self.overlay.borrow_mut();
    overlay.status_string = self.make_status_string();
    if !options.get_pause() && options.get_update_rate_display() {
      overlay.update_rate_string = self.make_update_rate_string();
    }
    if options.get_time_display() {
      overlay.time_string = self.make_time_string();
    }
    // TODO: Only set updated to true when the overlay data changes
    self.events.borrow_mut().set_updated();
  }
}

impl Updater for OverlayUpdater {
  fn update(&self) {
    let inputs: Ref<dyn OverlayUpdaterInputs> = self.inputs.borrow();
    if inputs.get_bug_requested().is_some()
      || inputs.get_pause_change_requested().is_some()
      || inputs.get_reset_requested()
      || inputs.get_time_display_change_requested().is_some()
      || inputs.get_update_rate_display_change_requested().is_some()
    {
      self.update_overlay();
      return;
    }
    if inputs.get_time_to_update() {
      let current_time_millis: f64 = inputs.get_current_time_millis();
      if self.metronome.borrow_mut().tick(current_time_millis) {
        self.update_overlay();
      }
    }
  }
}
