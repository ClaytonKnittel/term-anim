use crate::{entity::Entity, track::Track, train::Train};

enum State {
  Freezing,
  Frozen,
  Moving,
}

pub struct TrainScene {
  state: State,
  track: Track,
  train: Train,
  width: u32,
  height: u32,
}

impl TrainScene {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      state: State::Frozen,
      track: Track::new(height * 5 / 8, width),
      train: Train::new(5, 4 * width as i32, height * 5 / 8 - 2),
      width,
      height,
    }
  }

  pub fn freeze(&mut self) {
    self.state = State::Freezing;
  }

  pub fn unfreeze(&mut self) {
    self.state = State::Moving;
  }

  pub fn train(&self) -> &Train {
    &self.train
  }

  fn train_is_visible(&self) -> bool {
    self.train.left_x() < self.width as i32 && self.train.right_x() >= 0
  }
}

impl Entity for TrainScene {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(self.track.iterate_tiles().chain(self.train.iterate_tiles()))
  }

  fn tick(&mut self, t: usize) {
    match self.state {
      State::Frozen => {}
      State::Freezing => {
        if !self.train_is_visible() {
          self.state = State::Frozen;
        } else {
          self.track.tick(t);
          self.train.tick(t);
        }
      }
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
