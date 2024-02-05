use std::fmt::Display;

use termion::{color, style};

const G: f32 = -0.1;

pub struct Radiate {
  pub t: usize,
  pub pos: (i32, i32),
}

pub fn move_per_radiate(radiate: &Option<Radiate>, t: usize, pos: (i32, i32)) -> (i32, i32) {
  match radiate {
    Some(Radiate {
      t: radiate_t,
      pos: r_pos,
    }) => {
      let dt = (t - radiate_t).min(50);
      let dx = pos.0 - r_pos.0;
      let dy = pos.1 - r_pos.1;
      let d = dx.pow(2) + dy.pow(2);
      if (d as usize) < dt * dt {
        let scale = dt as f32 / (d as f32).sqrt();
        (
          r_pos.0 + (dx as f32 * scale) as i32,
          r_pos.1 + (dy as f32 * scale) as i32,
        )
      } else {
        pos
      }
    }
    None => pos,
  }
}

pub fn explosion_target_dt(target: (i32, i32), origin: (i32, i32)) -> usize {
  let dx = (target.0 - origin.0) as f32;
  let dy = (target.1 - origin.1) as f32;
  (dx.abs() * 0.3 + dy.abs() * 0.4 + 2.).ceil() as usize
}

pub fn explosion_path(dt: f32, target: (i32, i32), origin: (i32, i32)) -> (i32, i32) {
  let dx = (target.0 - origin.0) as f32;
  let dy = (target.1 - origin.1) as f32;
  let target_t = dx.abs() * 0.3 + dy.abs() * 0.4 + 2.;

  let vx = dx / target_t;
  let vy = dy / target_t + G / 2. * target_t;
  let x_pos = vx * dt;
  let y_pos = vy * dt - G / 2. * (dt * dt);

  let x_pos = if dt < target_t {
    (x_pos as i32) + origin.0
  } else {
    target.0
  };
  let y_pos = if dt < target_t {
    (y_pos as i32) + origin.1
  } else {
    target.1
  };

  (x_pos, y_pos)
}

#[derive(Clone)]
pub struct Draw {
  item: char,
  fg_color: Option<color::AnsiValue>,
  z_idx: i32,
  italic: bool,
}

impl Draw {
  pub fn new(item: char) -> Self {
    Self {
      item,
      fg_color: None,
      z_idx: 0,
      italic: false,
    }
  }

  pub fn item(&self) -> char {
    self.item
  }

  pub fn with_fg(self, color: color::AnsiValue) -> Self {
    Self {
      fg_color: Some(color),
      ..self
    }
  }

  pub fn with_z(self, z_idx: i32) -> Self {
    Self { z_idx, ..self }
  }

  pub fn z_idx(&self) -> i32 {
    self.z_idx
  }

  pub fn with_italic(self) -> Self {
    Self {
      italic: true,
      ..self
    }
  }
}

impl Display for Draw {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fg_str = if let Some(color) = self.fg_color {
      color.fg_string()
    } else {
      color::Reset.fg_str().to_owned()
    };
    let italic_str = if self.italic {
      style::Italic.to_string()
    } else {
      "".to_owned()
    };
    write!(f, "{}{}{}{}", style::Reset, italic_str, fg_str, self.item)
  }
}

impl PartialEq for Draw {
  fn eq(&self, other: &Self) -> bool {
    self.item == other.item
      && self.z_idx == other.z_idx
      && self.italic == other.italic
      && match (self.fg_color, other.fg_color) {
        (Some(color::AnsiValue(c1)), Some(color::AnsiValue(c2))) => c1 == c2,
        (None, None) => true,
        _ => false,
      }
  }
}

impl Eq for Draw {}
