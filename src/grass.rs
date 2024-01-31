use rand::Rng;
use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 2;

pub struct Grass {
  width: u32,
  height: u32,
  grid: Vec<Draw>,
}

impl Grass {
  pub fn new<R: Rng>(width: u32, height: u32, rand: &mut R) -> Self {
    Self {
      width,
      height,
      grid: (0..(width * height))
        .map(|_| Self::rand_tile(rand))
        .collect(),
    }
  }

  fn rand_tile<R: Rng>(rand: &mut R) -> Draw {
    let c = match rand.gen_range(0..8) {
      0 => '\'',
      1 => '"',
      2 => ';',
      3 => ':',
      4 => ',',
      5 => '.',
      6 => '`',
      7 => '~',
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
}

impl Entity for Grass {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new((0..self.height).flat_map(move |y| {
      (0..self.width).filter_map(move |x| {
        let tile = self.grid[self.idx(x, y)].clone();
        if tile.item() == ' ' {
          return None;
        }

        Some((tile.with_z(Z_IDX), (x as i32, y as i32)))
      })
    }))
  }

  fn tick(&mut self, t: usize) {}

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
