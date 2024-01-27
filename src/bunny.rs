use std::io::Write;

use termion::color;

use crate::{entity::Entity, util::Draw, window::Window};

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

  pub fn iterate_tiles(&self, w: u32, h: u32) -> impl Iterator<Item = (Draw, (u32, u32))> + '_ {
    let bunny_str: &[&str] = match self.state {
      BunnyState::Sleep => &BUNNY1,
      BunnyState::Wake => &BUNNY2,
      BunnyState::Walk1 => &BUNNY3,
      BunnyState::Walk2 => &BUNNY4,
    };

    bunny_str
      .iter()
      .enumerate()
      .filter_map(move |(y, row)| {
        let y = y as i32 + self.pos.1;
        if y < 0 || y >= h as i32 {
          return None;
        }

        Some(row.chars().enumerate().filter_map(move |(x, c)| {
          let x = x as i32 + self.pos.0;
          if x < 0 || x >= w as i32 || c == ' ' {
            return None;
          }

          Some((
            Draw::new(c)
              .with_fg(color::AnsiValue::grayscale(22))
              .with_z(1),
            (x as u32, y as u32),
          ))
        }))
      })
      .flatten()
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
  fn render<W: Write>(&self, window: &mut Window<W>) {
    self
      .iterate_tiles(window.width(), window.height())
      .for_each(|(draw, pos)| window.draw(draw, pos))
  }
}
