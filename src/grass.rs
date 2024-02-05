use rand::Rng;
use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 2;

struct Shreek {
  t: usize,
  pos: (i32, i32),
}

pub struct Grass {
  t: usize,
  width: u32,
  height: u32,
  grid: Vec<Draw>,
  shreek: Option<Shreek>,
}

impl Grass {
  pub fn new<R: Rng>(width: u32, height: u32, rand: &mut R) -> Self {
    Self {
      t: 0,
      width,
      height,
      grid: (0..(width * height))
        .map(|_| Self::rand_tile(rand))
        .collect(),
      shreek: None,
    }
  }

  fn rand_tile<R: Rng>(rand: &mut R) -> Draw {
    let c = match rand.gen_range(0..69) {
      0..=9 => ';',
      10..=19 => ':',
      20..=29 => '~',
      30..=39 => '*',
      40..=49 => '/',
      50..=59 => '\\',
      60..=63 => '$',
      64..=68 => '%',
      _ => unreachable!(),
    };

    let r = rand.gen_range(0..8);
    let green = r % 3 + r / 3 + 1;
    let red = r / 3;

    Draw::new(c).with_fg(color::AnsiValue::rgb(red, green, 0))
  }

  fn idx(&self, x: u32, y: u32) -> usize {
    (x + y * self.width) as usize
  }

  pub fn has_tile(&self, x: u32, y: u32) -> bool {
    self.grid[self.idx(x, y)].item() != ' '
  }

  pub fn delete_tile(&mut self, x: u32, y: u32) {
    let idx = self.idx(x, y);
    self.grid[idx] = Draw::new(' ');
  }

  pub fn shreek(&mut self, pos: (i32, i32)) {
    self.shreek = Some(Shreek { t: self.t, pos });
  }

  fn should_italic(&self, pos: (i32, i32)) -> bool {
    match self.shreek {
      Some(Shreek { t, pos: (x, y) }) => {
        let dt = self.t - t;
        let d = (pos.0 - x).pow(2) as f32 + ((pos.1 - y) as f32 * 11. / 5.).powi(2);
        let sqrtd = d.sqrt();
        let diff = dt as f32 - sqrtd;
        sqrtd <= 100. && diff <= 50. && (dt * dt) as f32 >= d && (diff as i32) % 12 <= 4
      }
      None => false,
    }
  }
}

impl Entity for Grass {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new((0..self.height).flat_map(move |y| {
      (0..self.width).filter_map(move |x| {
        let mut tile = self.grid[self.idx(x, y)].clone();
        if tile.item() == ' ' {
          return None;
        }
        if self.should_italic((x as i32, y as i32)) {
          tile = tile.with_italic();
        }

        Some((tile.with_z(Z_IDX), (x as i32, y as i32)))
      })
    }))
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
    if let Some(Shreek { t, pos: _ }) = self.shreek {
      if self.t - t > 125 {
        self.shreek = None;
      }
    }
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
