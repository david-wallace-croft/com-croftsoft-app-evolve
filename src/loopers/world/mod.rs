use crate::models::world::World;
use crate::painters::world::WorldPainter;
use crate::updaters::world::WorldUpdater;
use web_sys::CanvasRenderingContext2d;

pub struct WorldLooper<'a, const G: usize> {
  pub world_painter: WorldPainter<'a, G>,
  pub world_updater: WorldUpdater<G>,
}

impl<'a, const G: usize> WorldLooper<'a, G> {
  pub fn loop_once(
    &self,
    world: &mut World<G>,
  ) {
    self.world_updater.update(world);
    self.world_painter.paint(world);
  }
}
