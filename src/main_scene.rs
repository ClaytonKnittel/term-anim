use rand::Rng;

use crate::{bunny::Bunny, entity::Entity, landscape::Landscape, train_scene::TrainScene};

pub struct MainScene {
  bunny: Bunny,
  landscape: Landscape,
  train_scene: TrainScene,
}

impl MainScene {
  pub fn new<R: Rng>(width: u32, height: u32, r: &mut R) -> Self {
    Self {
      bunny: Bunny::new(),
      landscape: Landscape::new(width, height, r),
      train_scene: TrainScene::new(width, height),
    }
  }
}

impl Entity for MainScene {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      self
        .bunny
        .iterate_tiles()
        .chain(self.landscape.iterate_tiles())
        .chain(self.train_scene.iterate_tiles()),
    )
  }

  fn tick(&mut self, t: usize) {
    self.bunny.tick(t);
    self.landscape.tick(t);
    self.train_scene.tick(t);
  }

  fn click(&mut self, x: u32, y: u32) {
    self.bunny.click(x, y);
    self.landscape.click(x, y);
    self.train_scene.click(x, y);
  }

  fn drag(&mut self, x: u32, y: u32) {
    self.bunny.drag(x, y);
    self.landscape.drag(x, y);
    self.train_scene.drag(x, y);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.bunny.release(x, y);
    self.landscape.release(x, y);
    self.train_scene.release(x, y);
  }
}
