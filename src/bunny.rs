use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 10;

enum BunnyState {
  Sleep,
  Wake,
  Walk1,
  Walk2,
}

pub struct Bunny {
  state: BunnyState,
  pos: (i32, i32),
}

impl Bunny {
  pub fn new() -> Self {
    Self {
      state: BunnyState::Walk1,
      pos: (0, 0),
    }
  }

  pub fn shift(&mut self) {
    match self.state {
      BunnyState::Wake => {}
      BunnyState::Sleep => {}
      BunnyState::Walk1 => {
        self.state = BunnyState::Walk2;
        self.pos.0 += 1;
      }
      BunnyState::Walk2 => {
        self.state = BunnyState::Walk1;
      }
    };
  }
}

#[rustfmt::skip]
const BUNNY1: [&str; 3] = [
  r#"(\(\"#,
  r#"(-.-)"#,
  r#"o_(")(")"#,
];

#[rustfmt::skip]
const BUNNY2: [&str; 3] = [
  r#"(\(\"#,
  r#"(o.o)"#,
  r#"o_(")(")"#,
];

#[rustfmt::skip]
const BUNNY3: [&str; 4] = [
  r#" (\(\"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#" (")(")"#,
];

#[rustfmt::skip]
const BUNNY4: [&str; 4] = [
  r#" (\(\"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#"(") (")"#,
];

impl Entity for Bunny {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (Draw, (i32, i32))> + '_> {
    let bunny_str: &[&str] = match self.state {
      BunnyState::Sleep => &BUNNY1,
      BunnyState::Wake => &BUNNY2,
      BunnyState::Walk1 => &BUNNY3,
      BunnyState::Walk2 => &BUNNY4,
    };

    Box::new(bunny_str.iter().enumerate().flat_map(move |(y, row)| {
      let y = y as i32 + self.pos.1;

      row.chars().enumerate().filter_map(move |(x, c)| {
        let x = x as i32 + self.pos.0;
        if c == ' ' {
          return None;
        }

        Some((
          Draw::new(c)
            .with_fg(color::AnsiValue::grayscale(22))
            .with_z(Z_IDX),
          (x, y),
        ))
      })
    }))
  }

  fn tick(&mut self, t: usize) {
    if t % 16 == 0 {
      self.shift();
    }
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, dx: i32, dy: i32) {}
  fn release(&mut self, x: u32, y: u32) {}
}
