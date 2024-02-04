use crate::{entity::Entity, track::Track, train::Train};

enum State {
  Frozen,
  Moving,
}

pub struct TrainScene {
  state: State,
  track: Track,
  train: Train,
}

impl TrainScene {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      state: State::Frozen,
      track: Track::new(height * 5 / 8, width),
      train: Train::new(5, 4 * width as i32, height * 5 / 8 - 2),
    }
  }

  pub fn freeze(&mut self) {
    self.state = State::Frozen;
  }

  pub fn unfreeze(&mut self) {
    self.state = State::Moving;
  }
}

impl Entity for TrainScene {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(self.track.iterate_tiles().chain(self.train.iterate_tiles()))
  }

  fn tick(&mut self, t: usize) {
    match self.state {
      State::Frozen => {}
      State::Moving => {
        self.track.tick(t);
        self.train.tick(t);
      }
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    self.track.click(x, y);
    self.train.click(x, y);
  }

  fn drag(&mut self, x: u32, y: u32) {
    self.track.drag(x, y);
    self.train.drag(x, y);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.track.release(x, y);
    self.train.release(x, y);
  }
}
