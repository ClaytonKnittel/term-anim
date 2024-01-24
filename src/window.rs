use std::io::Write;
use termion::{color, cursor};

const C: f32 = 0.1;

#[derive(Clone, Copy)]
pub struct Particle {
  pos: f32,
  vel: f32,
}

impl Particle {
  fn new() -> Self {
    Self { pos: 0., vel: 0. }
  }

  /// Left, Up, Right, Down
  fn perturb(&self, neighbors: (Particle, Particle, Particle, Particle)) -> Self {
    let ddx = neighbors.2.pos + neighbors.0.pos - 2. * self.pos;
    let ddy = neighbors.3.pos + neighbors.1.pos - 2. * self.pos;
    let vel = self.vel + (C * C) * (ddx + ddy);
    let pos = self.pos + C * self.vel;
    Self { pos, vel }
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
      width,
      height,
      grid: vec![Particle::new(); (width * height) as usize],
    };
    s.allocate().expect("Failed to initialize window");
    s
  }

  pub fn stdout(&mut self) -> &mut W {
    &mut self.stdout
  }

  pub fn get(&self, x: i32, y: i32) -> Particle {
    let x = x.clamp(0, self.width as i32 - 1) as u32;
    let y = y.clamp(0, self.height as i32 - 1) as u32;
    self.grid[(x + y * self.width) as usize]
  }

  pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Particle {
    debug_assert!(x < self.width);
    debug_assert!(y < self.height);
    &mut self.grid[(x + y * self.width) as usize]
  }

  pub fn click(&mut self, x: u32, y: u32) {
    if 1 <= x && x <= self.width && 1 <= y && y <= self.height {
      self.get_mut(x - 1, y - 1).pos = 1.;
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
    for y in 0..self.height {
      for x in 0..self.width {
        let particle = self.get(x as i32, y as i32);
        write!(
          self.stdout,
          "{}O",
          color::Fg(color::Rgb(
            (particle.pos * 128.).clamp(0., 128.) as u8,
            ((1. - particle.pos) * 128.).clamp(0., 128.) as u8,
            128
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
