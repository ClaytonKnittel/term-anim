use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 30;
const G: f32 = -0.1;

#[rustfmt::skip]
const PEACH: [&str; 2] = [
  r#" ,"#,
  r#"(@"#,
];

#[derive(PartialEq, Eq)]
enum PeachState {
  Idle,
  /// (dx, dy) is the distance to the peach from where the mouse is held.
  Held {
    dx: i32,
    dy: i32,
  },
  Explode {
    t: usize,
    target_letters: Vec<(char, (i32, i32))>,
  },
}

pub struct Peach {
  x: i32,
  y: i32,
  t: usize,
  color: color::AnsiValue,
  state: PeachState,
}

impl Peach {
  pub fn new(x: i32, y: i32, color: color::AnsiValue) -> Self {
    Self {
      x,
      y,
      t: 0,
      color,
      state: PeachState::Idle,
    }
  }

  pub fn exploded(&self) -> bool {
    matches!(
      self.state,
      PeachState::Explode {
        t: _,
        target_letters: _,
      }
    )
  }

  pub fn explode(&mut self, target_letters: Vec<(char, (i32, i32))>) {
    self.state = PeachState::Explode {
      t: self.t,
      target_letters,
    };
  }

  pub fn hitbox(&self) -> (i32, i32) {
    (self.x + 1, self.y + 1)
  }
}

impl Entity for Peach {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    match &self.state {
      PeachState::Idle | PeachState::Held { dx: _, dy: _ } => Box::new(
        [
          (
            Draw::new(',')
              .with_fg(color::AnsiValue::rgb(2, 1, 0))
              .with_z(Z_IDX),
            (self.x + 1, self.y),
          ),
          (
            Draw::new('(').with_fg(self.color).with_z(Z_IDX),
            (self.x, self.y + 1),
          ),
          (
            Draw::new('@').with_fg(self.color).with_z(Z_IDX),
            (self.x + 1, self.y + 1),
          ),
        ]
        .into_iter(),
      ),
      PeachState::Explode { t, target_letters } => {
        Box::new(target_letters.iter().map(move |(c, (x, y))| {
          let dt = (self.t - t) as f32;
          let dx = (x - self.x) as f32;
          let dy = (y - self.y) as f32;
          let target_t = dx.abs() * 0.5 + 2.;

          let vx = dx / target_t;
          let vy = dy / target_t + G / 2. * target_t;
          let x_pos = vx * dt;
          let y_pos = vy * dt - G / 2. * (dt * dt);

          let x_pos = if dt < target_t {
            (x_pos as i32) + self.x
          } else {
            *x
          };
          let y_pos = if dt < target_t {
            (y_pos as i32) + self.y
          } else {
            *y
          };

          (
            Draw::new(*c).with_fg(self.color).with_z(Z_IDX),
            (x_pos, y_pos),
          )
        }))
      }
    }
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
  }

  fn click(&mut self, x: u32, y: u32) {
    let dx = self.x - x as i32;
    let dy = self.y - y as i32;
    if (-1..=0).contains(&dx)
      && (-1..=0).contains(&dy)
      && (dx != 0 || dy != 0)
      && self.state == PeachState::Idle
    {
      self.state = PeachState::Held { dx, dy };
    }
  }

  fn drag(&mut self, x: u32, y: u32) {
    match self.state {
      PeachState::Held { dx, dy } => {
        self.x = x as i32 + dx;
        self.y = y as i32 + dy;
      }
      PeachState::Idle
      | PeachState::Explode {
        t: _,
        target_letters: _,
      } => {}
    }
  }

  fn release(&mut self, _x: u32, _y: u32) {
    match self.state {
      PeachState::Held { dx: _, dy: _ } => {
        self.state = PeachState::Idle;
      }
      PeachState::Idle
      | PeachState::Explode {
        t: _,
        target_letters: _,
      } => {}
    }
  }
}
