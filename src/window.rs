use std::io::Write;
use termion::{color, cursor};

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

  fn shape(&self, neighbors: (Particle, Particle, Particle, Particle)) -> char {
    match (self.pos.clamp(0., 0.91) * 10.) as u32 {
      0 => '0',
      1 => '1',
      2 => '2',
      3 => '3',
      4 => '4',
      5 => '5',
      6 => '6',
      7 => '7',
      8 => '8',
      9 => '9',
      _ => panic!("what"),
    }
    // enum Cat {
    //   H,
    //   L,
    // }
    // let cat = |particle: Particle| {
    //   if particle.pos > 0.6 {
    //     Cat::H
    //   } else {
    //     Cat::L
    //   }
    // };

    // match (
    //   cat(neighbors.0),
    //   cat(neighbors.1),
    //   cat(neighbors.2),
    //   cat(neighbors.3),
    // ) {
    //   (Cat::L, Cat::L, Cat::L, Cat::L) | (Cat::H, Cat::H, Cat::H, Cat::H) => '.',
    //   (Cat::H, Cat::L, Cat::L, Cat::L) => '-',
    //   (Cat::L, Cat::H, Cat::L, Cat::L) => '\'',
    //   (Cat::L, Cat::L, Cat::H, Cat::L) => '-',
    //   (Cat::L, Cat::L, Cat::L, Cat::H) => '.',
    //   (Cat::L, Cat::H, Cat::L, Cat::H) => '|',
    //   (Cat::H, Cat::L, Cat::H, Cat::L) => '-',
    //   (Cat::H, Cat::H, Cat::L, Cat::L) | (Cat::L, Cat::L, Cat::H, Cat::H) => '/',
    //   (Cat::H, Cat::L, Cat::L, Cat::H) | (Cat::L, Cat::H, Cat::H, Cat::L) => '\\',
    //   (Cat::L, Cat::H, Cat::H, Cat::H) => '|',
    //   (Cat::H, Cat::L, Cat::H, Cat::H) => '-',
    //   (Cat::H, Cat::H, Cat::L, Cat::H) => '|',
    //   (Cat::H, Cat::H, Cat::H, Cat::L) => '-',
    // }
  }
}

pub struct Window<W: Write> {
  stdout: W,
  width: u32,
  height: u32,
  grid: Vec<Particle>,
}

impl<W: Write> Window<W> {
  pub fn new(stdout: W, width: u32, height: u32) -> Self {
    let mut s = Self {
      stdout,
      width: SCALE as u32 * width,
      height: SCALE as u32 * height,
      grid: vec![Particle::new(); ((SCALE * SCALE) as u32 * width * height) as usize],
    };
    s.allocate().expect("Failed to initialize window");
    s
  }

  pub fn stdout(&mut self) -> &mut W {
    &mut self.stdout
  }

  fn idx(&self, x: i32, y: i32) -> usize {
    let x = x.clamp(0, self.width as i32 - 1) as u32;
    let y = y.clamp(0, self.height as i32 - 1) as u32;
    (x + y * self.width) as usize
  }

  fn big_idx(&self, x: i32, y: i32) -> usize {
    let x = x.clamp(0, self.width as i32 / SCALE - 1) as u32;
    let y = y.clamp(0, self.height as i32 / SCALE - 1) as u32;
    (x + y * self.width / SCALE as u32) as usize
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

  pub fn click(&mut self, x: u32, y: u32) {
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

  fn allocate(&mut self) -> std::io::Result<()> {
    write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
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

  pub fn render(&mut self) -> std::io::Result<()> {
    write!(self.stdout, "{}", cursor::Up(self.height as u16))?;
    let bigs: Vec<_> = (0..self.height as i32 / SCALE)
      .flat_map(|y| (0..self.width as i32 / SCALE).map(move |x| (x, y)))
      .map(|(x, y)| self.get_big(x, y))
      .collect();
    for y in 0..self.height as i32 / SCALE {
      for x in 0..self.width as i32 / SCALE {
        let particle = bigs[self.big_idx(x, y)];
        write!(
          self.stdout,
          "{}{}",
          color::Fg(color::Rgb(
            (particle.pos * 256.).clamp(0., 255.) as u8,
            ((1. - particle.pos) * 256.).clamp(0., 255.) as u8,
            128
          )),
          particle.shape((
            bigs[self.big_idx(x - 1, y)],
            bigs[self.big_idx(x, y - 1)],
            bigs[self.big_idx(x + 1, y)],
            bigs[self.big_idx(x, y + 1)],
          ))
        )?;
      }
      write!(
        self.stdout,
        "{}{}{}",
        color::Fg(color::Reset),
        cursor::Left(self.width as u16),
        cursor::Down(1)
      )?;
    }
    self.stdout.flush()
  }
}
