use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 26;

pub struct Carrot {
  t: usize,
  pos: (i32, i32),
  appear: Option<usize>,
  upside_down: bool,
  no_head: bool,
}

impl Carrot {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      t: 0,
      pos,
      appear: None,
      upside_down: false,
      no_head: false,
    }
  }

  pub fn set_pos(&mut self, pos: (i32, i32)) {
    self.pos = pos;
  }

  pub fn delete_head(&mut self) {
    self.no_head = true;
  }

  pub fn appear(&mut self) {
    self.appear = Some(self.t);
  }

  pub fn make_upside_down(&mut self) {
    self.upside_down = true;
  }
}

impl Entity for Carrot {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      if self.upside_down {
        [
          (
            Draw::new('^')
              .with_fg(color::AnsiValue::rgb(5, 1, 0))
              .with_z(if self.no_head { -1 } else { Z_IDX }),
            self.pos,
          ),
          (
            Draw::new('H')
              .with_fg(color::AnsiValue::rgb(5, 1, 0))
              .with_z(Z_IDX),
            (self.pos.0, self.pos.1 + 1),
          ),
          (
            Draw::new('M')
              .with_fg(color::AnsiValue::rgb(0, 1, 0))
              .with_z(Z_IDX),
            (self.pos.0, self.pos.1 + 2),
          ),
        ]
      } else {
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
      }
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
