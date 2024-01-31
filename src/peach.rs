use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 30;

#[rustfmt::skip]
const PEACH: [&str; 2] = [
  r#" ,"#,
  r#"(@"#,
];

enum PeachState {
  Idle,
  /// (dx, dy) is the distance to the peach from where the mouse is held.
  Held {
    dx: i32,
    dy: i32,
  },
}

pub struct Peach {
  x: i32,
  y: i32,
  state: PeachState,
}

impl Peach {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      x,
      y,
      state: PeachState::Idle,
    }
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

  fn click(&mut self, x: u32, y: u32) {
    let dx = self.x - x as i32;
    let dy = self.y - y as i32;
    if (-2..=1).contains(&dx) && (-2..=1).contains(&dy) && (dx != 1 || dy != 1) {
      self.state = PeachState::Held { dx, dy };
    }
  }

  fn drag(&mut self, x: u32, y: u32) {
    match self.state {
      PeachState::Held { dx, dy } => {
        self.x = x as i32 + dx;
        self.y = y as i32 + dy;
      }
      PeachState::Idle => {}
    }
  }

  fn release(&mut self, _x: u32, _y: u32) {
    self.state = PeachState::Idle;
  }
}
