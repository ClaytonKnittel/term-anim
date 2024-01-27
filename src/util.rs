use std::fmt::Display;

use termion::color;

#[derive(Clone)]
pub struct Draw {
  item: char,
  fg_color: Option<color::AnsiValue>,
  bg_color: Option<color::AnsiValue>,
  z_idx: i32,
}

impl Draw {
  pub fn new(item: char) -> Self {
    Self {
      item,
      fg_color: None,
      bg_color: None,
      z_idx: 0,
    }
  }

  pub fn with_fg(self, color: color::AnsiValue) -> Self {
    Self {
      fg_color: Some(color),
      ..self
    }
  }

  pub fn with_bg(self, color: color::AnsiValue) -> Self {
    Self {
      bg_color: Some(color),
      ..self
    }
  }

  pub fn with_z(self, z_idx: i32) -> Self {
    Self { z_idx, ..self }
  }

  pub fn z_idx(&self) -> i32 {
    self.z_idx
  }
}

impl Display for Draw {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fg_str = if let Some(color) = self.fg_color {
      color.fg_string()
    } else {
      color::Reset.fg_str().to_owned()
    };
    let bg_str = if let Some(color) = self.bg_color {
      color.bg_string()
    } else {
      color::Reset.bg_str().to_owned()
    };
    write!(f, "{}{}{}", fg_str, bg_str, self.item)
  }
}
