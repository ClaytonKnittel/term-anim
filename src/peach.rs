use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 30;

#[rustfmt::skip]
const PEACH: [&str; 2] = [
  r#" ,"#,
  r#"(@"#,
];

pub struct Peach {
  x: i32,
  y: i32,
}

impl Peach {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

impl Entity for Peach {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      [
        (
          Draw::new(',')
            .with_fg(color::AnsiValue::rgb(2, 1, 0))
            .with_z(Z_IDX),
          (self.x + 1, self.y),
        ),
        (
          Draw::new('(')
            .with_fg(color::AnsiValue::rgb(5, 0, 1))
            .with_z(Z_IDX),
          (self.x, self.y + 1),
        ),
        (
          Draw::new('@')
            .with_fg(color::AnsiValue::rgb(5, 0, 1))
            .with_z(Z_IDX),
          (self.x + 1, self.y + 1),
        ),
      ]
      .into_iter(),
    )
  }

  fn tick(&mut self, t: usize) {}

  fn click(&mut self, x: u32, y: u32) {}
}
