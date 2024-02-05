use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 23;

pub struct Carrot {
  t: usize,
  pos: (i32, i32),
  appear: Option<usize>,
}

impl Carrot {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      t: 0,
      pos,
      appear: None,
    }
  }

  pub fn appear(&mut self) {
    self.appear = Some(self.t);
  }
}

impl Entity for Carrot {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      [
        (
          Draw::new('W')
            .with_fg(color::AnsiValue::rgb(0, 1, 0))
            .with_z(Z_IDX),
          self.pos,
        ),
        (
          Draw::new('H')
            .with_fg(color::AnsiValue::rgb(5, 1, 0))
            .with_z(Z_IDX),
          (self.pos.0, self.pos.1 + 1),
        ),
        (
          Draw::new('V')
            .with_fg(color::AnsiValue::rgb(5, 1, 0))
            .with_z(Z_IDX),
          (self.pos.0, self.pos.1 + 2),
        ),
      ]
      .into_iter()
      .take(match self.appear {
        Some(initial_t) => self.t - initial_t,
        None => 0,
      }),
    )
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
