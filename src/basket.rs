use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 40;

#[rustfmt::skip]
const BASKET: [&str; 6] = [
  r#"   ======   "#,
  r#" // (@   \\ "#,
  r#"||(@ (@ (@||"#,
  r#"============"#,
  r#" \\##||##// "#,
  r#"   -~~~~-   "#,
];

pub struct Basket {
  pos: (i32, i32),
}

impl Basket {
  pub fn new(pos: (i32, i32)) -> Self {
    Self { pos }
  }

  pub fn contains_click(&self, pos: (i32, i32)) -> bool {
    let dx = pos.0 - self.pos.0;
    let dy = pos.1 - self.pos.1;
    (0..12).contains(&dx) && (0..6).contains(&dy) && ((dx != 0 && dx != 11) || dy != 0)
  }
}

impl Entity for Basket {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(BASKET.iter().enumerate().flat_map(move |(dy, row)| {
      let y = dy as i32 + self.pos.1;

      row.chars().enumerate().filter_map(move |(dx, c)| {
        let x = dx as i32 + self.pos.0;
        if c == ' ' && !(1..=2).contains(&dy) {
          return None;
        }

        let color = if dy == 1 && (4..=5).contains(&dx) {
          color::AnsiValue::rgb(5, 0, 0)
        } else if dy == 2 && (2..=3).contains(&dx) {
          color::AnsiValue::rgb(5, 0, 1)
        } else if dy == 2 && (5..=6).contains(&dx) {
          color::AnsiValue::rgb(4, 0, 0)
        } else if dy == 2 && (8..=9).contains(&dx) {
          color::AnsiValue::rgb(5, 1, 0)
        } else {
          color::AnsiValue::rgb(2, 1, 0)
        };

        Some((Draw::new(c).with_fg(color).with_z(Z_IDX), (x, y)))
      })
    }))
  }

  fn tick(&mut self, _t: usize) {}

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
