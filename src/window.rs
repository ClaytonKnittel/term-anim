use std::io::Write;
use termion::{color, cursor};

pub struct Window {
  width: u32,
  height: u32,
  x: u32,
}

impl Window {
  pub fn new(width: u32, height: u32) -> Self {
    let s = Self {
      width,
      height,
      x: 0,
    };
    s.allocate();
    s
  }

  fn allocate(&self) {
    for _ in 0..self.height {
      println!();
    }
  }

  pub fn render(&mut self) {
    print!("{}", cursor::Up(self.height as u16));
    for y in 0..self.height {
      for x in 0..self.width {
        if x + y * self.width == self.x {
          print!(
            "{}X{}",
            color::Fg(color::Rgb(255, 0, 255)),
            color::Fg(color::Reset)
          );
        } else {
          print!(".");
        }
      }
      print!("{}{}", cursor::Left(self.width as u16), cursor::Down(1));
    }
    std::io::stdout().flush();
    self.x += 1;
  }
}
