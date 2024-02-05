use std::iter;

use rand::Rng;
use termion::color;

use crate::{
  entity::Entity,
  util::{explosion_path, move_per_radiate, Draw, Radiate},
  water::Water,
};

const Z_IDX: i32 = 30;
const DEBRIS_Z_IDX: i32 = 6;

#[derive(Clone, PartialEq, Eq)]
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
    rand_letters: Vec<(char, (i32, i32))>,
  },
}

pub struct Peach {
  x: i32,
  y: i32,
  t: usize,
  color: color::AnsiValue,
  state: PeachState,
  radiate: Option<Radiate>,
}

impl Peach {
  pub fn new(x: i32, y: i32, color: color::AnsiValue) -> Self {
    Self {
      x,
      y,
      t: 0,
      color,
      state: PeachState::Idle,
      radiate: None,
    }
  }

  pub fn exploded(&self) -> bool {
    matches!(
      self.state,
      PeachState::Explode {
        t: _,
        target_letters: _,
        rand_letters: _
      }
    )
  }

  pub fn explode<R: Rng>(&mut self, target_letters: Vec<(char, (i32, i32))>, rng: &mut R) {
    const RADIUS: i32 = 70;
    self.state = PeachState::Explode {
      t: self.t,
      target_letters,
      rand_letters: (0..200)
        .map(|_| {
          let mut dx = rng.gen_range(-RADIUS..=RADIUS);
          let mut dy = rng.gen_range(-RADIUS..=RADIUS);
          while dx * dx + dy * dy > (RADIUS * RADIUS) || self.x + dx < 0 || self.y + dy < 0 {
            dx = rng.gen_range(-RADIUS..=RADIUS);
            dy = rng.gen_range(-RADIUS..=RADIUS);
          }

          let letter = rng.gen_range('a'..='z');
          (letter, (self.x + dx, self.y + dy))
        })
        .collect(),
    };
  }

  pub fn hitbox(&self) -> (i32, i32) {
    (self.x + 1, self.y + 1)
  }

  pub fn radiate(&mut self, pos: (i32, i32)) {
    self.radiate = Some(Radiate { t: self.t, pos });
  }

  fn debris_pos(
    &self,
    (t, targeted, _, (x, y)): &(usize, bool, char, (i32, i32)),
  ) -> ((i32, i32), bool) {
    let mut pos = explosion_path((self.t - t) as f32, (*x, *y), (self.x, self.y));
    let resting = pos == (*x, *y);
    if !targeted {
      pos = move_per_radiate(&self.radiate, self.t, pos);
    }
    (pos, resting)
  }

  pub fn maybe_dunk(&mut self, water: &mut Water) {
    if let PeachState::Explode {
      t,
      target_letters,
      rand_letters,
    } = self.state.clone()
    {
      self.state = PeachState::Explode {
        t,
        target_letters,
        rand_letters: rand_letters
          .clone()
          .into_iter()
          .filter(|&(c, pos)| {
            let (pos, resting) = self.debris_pos(&(t, false, c, pos));
            if resting && water.is_wet(pos) {
              water.click(pos.0 as u32, pos.1 as u32);
              false
            } else {
              true
            }
          })
          .collect(),
      };
    }
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
      PeachState::Explode {
        t,
        target_letters,
        rand_letters,
      } => Box::new(
        (target_letters
          .iter()
          .zip(iter::repeat(true))
          .chain(rand_letters.iter().zip(iter::repeat(false))))
        .map(move |((c, (x, y)), targeted)| {
          let (pos, _) = self.debris_pos(&(*t, targeted, *c, (*x, *y)));
          (
            Draw::new(*c)
              .with_fg(self.color)
              .with_z(DEBRIS_Z_IDX + if targeted { 1 } else { 0 }),
            pos,
          )
        }),
      ),
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
        rand_letters: _,
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
        rand_letters: _,
      } => {}
    }
  }
}
