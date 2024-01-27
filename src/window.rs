use std::io::Write;
use termion::{clear, cursor};

use crate::util::Draw;

pub struct Window<W: Write> {
  stdout: W,
  width: u32,
  height: u32,
  canvas: Vec<Option<Draw>>,
}

impl<W: Write> Window<W> {
  pub fn new(stdout: W, width: u32, height: u32) -> Self {
    let mut s = Self {
      stdout,
      width,
      height,
      canvas: (0..(width * height)).map(|_| None).collect(),
    };
    s.allocate().expect("Failed to initialize window");
    s
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn stdout(&mut self) -> &mut W {
    &mut self.stdout
  }

  fn idx(&self, x: u32, y: u32) -> usize {
    (x + y * self.width) as usize
  }

  fn get(&self, x: u32, y: u32) -> &Option<Draw> {
    self.canvas.get(self.idx(x, y)).unwrap()
  }

  fn get_mut(&mut self, x: u32, y: u32) -> &mut Option<Draw> {
    let idx = self.idx(x, y);
    self.canvas.get_mut(idx).unwrap()
  }

  fn allocate(&mut self) -> std::io::Result<()> {
    write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
  }

  pub fn reset(&mut self) {
    self.canvas = (0..(self.width * self.height)).map(|_| None).collect();
  }

  pub fn draw(&mut self, draw: Draw, pos: (u32, u32)) {
    *self.get_mut(pos.0, pos.1) = Some(self.get(pos.0, pos.1).clone().map_or(
      draw.clone(),
      |cur_el| {
        if cur_el.z_idx() < draw.z_idx() {
          draw
        } else {
          cur_el
        }
      },
    ))
  }

  pub fn render(&mut self) -> std::io::Result<()> {
    write!(self.stdout, "{}", cursor::Up(self.height as u16),)?;
    for y in 0..self.height {
      for x in 0..self.width {
        if let Some(draw) = self.get(x, y).clone() {
          write!(self.stdout, "{}", draw)?;
        } else {
          write!(self.stdout, " ")?;
        }
      }
      write!(
        self.stdout,
        "{}{}",
        cursor::Left(self.width as u16),
        cursor::Down(1)
      )?;
    }
    Ok(())
  }
}
