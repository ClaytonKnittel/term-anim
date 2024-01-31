use crate::{entity::Entity, peach::Peach, track::Track, train::Train};

pub struct TrainScene {
  track: Track,
  train: Train,
  peach: Peach,
}

impl TrainScene {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      track: Track::new(height * 5 / 8, width),
      train: Train::new(5, 4 * width as i32, height * 5 / 8 - 2),
      peach: Peach::new(30, 10),
    }
  }
}

impl Entity for TrainScene {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      self
        .track
        .iterate_tiles()
        .chain(self.train.iterate_tiles())
        .chain(self.peach.iterate_tiles()),
    )
  }

  fn tick(&mut self, t: usize) {
    self.track.tick(t);
    self.train.tick(t);
    self.peach.tick(t);
  }

  fn click(&mut self, x: u32, y: u32) {
    self.track.click(x, y);
    self.train.click(x, y);
    self.peach.click(x, y);
  }

  fn drag(&mut self, x: u32, y: u32) {
    self.track.drag(x, y);
    self.train.drag(x, y);
    self.peach.drag(x, y);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.track.release(x, y);
    self.train.release(x, y);
    self.peach.release(x, y);
  }
}
