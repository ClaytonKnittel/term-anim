use rand::Rng;
use termion::color;

use crate::{
  entity::Entity,
  util::{explosion_path, explosion_target_dt, Draw},
};

const Z_IDX: i32 = 26;
const DEBRIS_Z_IDX: i32 = 25;

pub struct Carrot {
  t: usize,
  pos: (i32, i32),
  appear: Option<usize>,
  upside_down: bool,
  no_head: bool,
  target_letters: Vec<(char, (i32, i32))>,
  debris: Vec<(usize, char, (i32, i32), bool)>,
}

impl Carrot {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      t: 0,
      pos,
      appear: None,
      upside_down: false,
      no_head: false,
      target_letters: Vec::new(),
      debris: Vec::new(),
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

  pub fn set_target_letters(&mut self, letters: Vec<(char, (i32, i32))>) {
    self.target_letters = letters;
  }

  pub fn scatter<R: Rng>(&mut self, rng: &mut R) {
    const RADIUS: i32 = 15;
    self.debris.append(
      &mut (0..15)
        .map(|_| {
          let mut dx = rng.gen_range(-RADIUS..=RADIUS);
          let mut dy = rng.gen_range(-RADIUS..=RADIUS);
          while dx * dx + dy * dy > (RADIUS * RADIUS) || self.pos.0 + dx < 0 || self.pos.1 + dy < 0
          {
            dx = rng.gen_range(-RADIUS..=RADIUS);
            dy = rng.gen_range(-RADIUS..=RADIUS);
          }

          let letter = rng.gen_range('a'..='z');
          (self.t, letter, (self.pos.0 + dx, self.pos.1 + dy), false)
        })
        .collect(),
    );
    if let Some((c, pos)) = self.target_letters.pop() {
      self.debris.push((self.t, c, pos, true));
    }
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
      })
      .chain(self.debris.iter().map(move |(t, c, (x, y), targeted)| {
        (
          Draw::new(*c)
            .with_fg(color::AnsiValue::rgb(5, 1, 0))
            .with_z(DEBRIS_Z_IDX + if *targeted { 1 } else { 0 }),
          explosion_path((self.t - t) as f32, (*x, *y), self.pos),
        )
      })),
    )
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
    self.debris = self
      .debris
      .clone()
      .into_iter()
      .filter(|(t, _, pos, targeted)| {
        *targeted || 2 * explosion_target_dt(*pos, self.pos) > self.t - t
      })
      .collect();
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
