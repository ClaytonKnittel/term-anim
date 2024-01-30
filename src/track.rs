use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 20;

pub struct Track {
  y: u32,
  width: u32,
}

impl Track {
  pub fn new(y: u32, width: u32) -> Self {
    Self { y, width }
  }
}

impl Entity for Track {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new((0..self.width).map(|x| {
      let tile = if x % 4 == 1 { '+' } else { '=' };
      let col = color::AnsiValue::grayscale(20);

      (
        Draw::new(tile).with_fg(col).with_z(Z_IDX),
        (x as i32, self.y as i32),
      )
    }))
  }

  fn tick(&mut self, t: usize) {}

  fn click(&mut self, x: u32, y: u32) {}
}
