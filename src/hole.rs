use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 20;

#[rustfmt::skip]
const HOLE: [&str; 4] = [
  r#" ===L"#,
  r#"H   H"#,
  r#"HL _H"#,
  r#" === "#,
];

pub struct Hole {
  pos: (i32, i32),
}

impl Hole {
  pub fn new(pos: (i32, i32)) -> Self {
    Self { pos }
  }
}

impl Entity for Hole {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(HOLE.iter().enumerate().flat_map(move |(y, row)| {
      let y = y as i32 + self.pos.1;

      row.chars().enumerate().filter_map(move |(x, c)| {
        let x = x as i32 + self.pos.0;
        if (x == 0 || x == 3) && (y == 0 || y == 3) && c == ' ' {
          return None;
        }

        Some((
          Draw::new(c)
            .with_fg(color::AnsiValue::rgb(2, 1, 0))
            .with_z(Z_IDX),
          (x, y),
        ))
      })
    }))
  }

  fn tick(&mut self, _t: usize) {}

  fn click(&mut self, x: u32, y: u32) {
    todo!()
  }

  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
