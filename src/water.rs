use std::iter;

use termion::color;

use crate::{entity::Entity, util::Draw};

const C: f32 = 0.02;
const SCALE: i32 = 2;

#[derive(Clone, Copy)]
pub struct Particle {
  pos: f32,
  vel: f32,
}

impl Particle {
  fn new() -> Self {
    Self { pos: 0.5, vel: 0. }
  }

  /// Left, Up, Right, Down
  fn perturb(&self, neighbors: (Particle, Particle, Particle, Particle)) -> Self {
    let ddx = neighbors.2.pos + neighbors.0.pos - 2. * self.pos;
    let ddy = neighbors.3.pos + neighbors.1.pos - 2. * self.pos;
    let vel = 0.996 * self.vel + (C * C) * (ddx + ddy) + 0.8 * (C * C) * (0.5 - self.pos);
    let pos = self.pos + self.vel;
    Self { pos, vel }
  }

  fn shape(&self, neighbors: (Particle, Particle, Particle, Particle)) -> Draw {
    //match (self.pos.clamp(0., 0.91) * 10.) as u32 {
    //  0 => '0',
    //  1 => '1',
    //  2 => '2',
    //  3 => '3',
    //  4 => '4',
    //  5 => '5',
    //  6 => '6',
    //  7 => '7',
    //  8 => '8',
    //  9 => '9',
    //  _ => panic!("what"),
    //}

    enum Cat {
      H,
      L,
    }
    let cat = |particle: Particle| {
      if particle.pos > self.pos + 0.009 {
        Cat::H
      } else {
        Cat::L
      }
    };

    let shape = match (
      cat(neighbors.0),
      cat(neighbors.1),
      cat(neighbors.2),
      cat(neighbors.3),
    ) {
      (Cat::L, Cat::L, Cat::L, Cat::L) | (Cat::H, Cat::H, Cat::H, Cat::H) => '.',
      (Cat::H, Cat::L, Cat::L, Cat::L) => '-',
      (Cat::L, Cat::H, Cat::L, Cat::L) => '\'',
      (Cat::L, Cat::L, Cat::H, Cat::L) => '-',
      (Cat::L, Cat::L, Cat::L, Cat::H) => '.',
      (Cat::L, Cat::H, Cat::L, Cat::H) => '|',
      (Cat::H, Cat::L, Cat::H, Cat::L) => '-',
      (Cat::H, Cat::H, Cat::L, Cat::L) | (Cat::L, Cat::L, Cat::H, Cat::H) => '/',
      (Cat::H, Cat::L, Cat::L, Cat::H) | (Cat::L, Cat::H, Cat::H, Cat::L) => '\\',
      (Cat::L, Cat::H, Cat::H, Cat::H) => '|',
      (Cat::H, Cat::L, Cat::H, Cat::H) => '-',
      (Cat::H, Cat::H, Cat::L, Cat::H) => '|',
      (Cat::H, Cat::H, Cat::H, Cat::L) => '-',
    };

    let color = if self.pos > 0.507 {
      color::AnsiValue::grayscale(22)
    } else {
      color::AnsiValue::rgb(0, 2, 5)
    };

    Draw::new(shape).with_fg(color)
  }
}

pub struct Water {
  width: u32,
  height: u32,
  grid: Vec<Particle>,
}

impl Water {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width: SCALE as u32 * width,
      height: SCALE as u32 * height,
      grid: vec![Particle::new(); ((SCALE * SCALE) as u32 * width * height) as usize],
    }
  }

  fn idx(&self, x: i32, y: i32) -> usize {
    let x = x.clamp(0, self.width as i32 - 1) as u32;
    let y = y.clamp(0, self.height as i32 - 1) as u32;
    (x + y * self.width) as usize
  }

  fn big_idx_wh(x: i32, y: i32, width: u32, height: u32) -> usize {
    let x = x.clamp(0, width as i32 / SCALE - 1) as u32;
    let y = y.clamp(0, height as i32 / SCALE - 1) as u32;
    (x + y * width / SCALE as u32) as usize
  }

  fn big_idx(&self, x: i32, y: i32) -> usize {
    Self::big_idx_wh(x, y, self.width, self.height)
  }

  pub fn get(&self, x: i32, y: i32) -> Particle {
    self.grid[self.idx(x, y)]
  }

  pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Particle {
    debug_assert!(x < self.width);
    debug_assert!(y < self.height);
    &mut self.grid[(x + y * self.width) as usize]
  }

  pub fn get_big(&self, x: i32, y: i32) -> Particle {
    let x = x.clamp(0, self.width as i32 / SCALE - 1);
    let y = y.clamp(0, self.height as i32 / SCALE - 1);
    let pos: f32 = (0..SCALE)
      .flat_map(|dy| (0..SCALE).map(move |dx| self.get(SCALE * x + dx, SCALE * y + dy).pos))
      .sum();
    let vel: f32 = (0..SCALE)
      .flat_map(|dy| (0..SCALE).map(move |dx| self.get(SCALE * x + dx, SCALE * y + dy).vel))
      .sum();
    Particle {
      pos: pos / (SCALE * SCALE) as f32,
      vel: vel / (SCALE * SCALE) as f32,
    }
  }

  pub fn advance(&mut self) {
    self.grid = (0..self.height as i32)
      .flat_map(|y| (0..self.width as i32).map(move |x| (x, y)))
      .map(|(x, y)| {
        self.get(x, y).perturb((
          self.get(x - 1, y),
          self.get(x, y - 1),
          self.get(x + 1, y),
          self.get(x, y + 1),
        ))
      })
      .collect();
  }
}

impl Entity for Water {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (Draw, (i32, i32))> + '_> {
    let bigs: Vec<_> = (0..self.height as i32 / SCALE)
      .flat_map(|y| (0..self.width as i32 / SCALE).map(move |x| (x, y)))
      .map(|(x, y)| self.get_big(x, y))
      .collect();
    Box::new(
      (0..self.height as i32 / SCALE)
        .zip(iter::repeat((self, bigs)))
        .flat_map(|(y, (water, bigs))| {
          (0..self.width as i32 / SCALE).map(move |x| {
            let particle = bigs[water.big_idx(x, y)];
            let shape = particle.shape((
              bigs[water.big_idx(x - 1, y)],
              bigs[water.big_idx(x, y - 1)],
              bigs[water.big_idx(x + 1, y)],
              bigs[water.big_idx(x, y + 1)],
            ));
            (shape, (x, y))
          })
        }),
    )
  }

  fn tick(&mut self, _t: usize) {
    for _ in 0..7 {
      self.advance();
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    if 1 <= x && x <= self.width / SCALE as u32 && 1 <= y && y <= self.height / SCALE as u32 {
      for dy in 0..SCALE as u32 {
        for dx in 0..SCALE as u32 {
          self
            .get_mut(SCALE as u32 * (x - 1) + dx, SCALE as u32 * (y - 1) + dy)
            .pos = 1.;
        }
      }
    }
  }
}
