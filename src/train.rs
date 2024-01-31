use termion::color;

use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 21;

#[rustfmt::skip]
const TRAIN_ENGINE: [&str; 3] = [
  r#"      _---------------------------------------="#,
  r#"  __/_2 ||  o o o o o o o o o o o o o o o  || |"#,
  r#"(___---______________________________---______|"#,
];

#[rustfmt::skip]
const TRAIN_CABOOSE: [&str; 3] = [
  r#"---------------------------------------_      "#,
  r#" ||  o o o o o o o o o o o o o o o  || c_\__  "#,
  r#"______---______________________________---___)"#,
];

#[rustfmt::skip]
const TRAIN_CABIN: [&str; 3] = [
  r#"---------------------------------------="#,
  r#" ||  o o o o o o o o o o o o o o o  || |"#,
  r#"______---_____________________---______|"#,
];

pub struct Train {
  // Total count of cabin + engine cars (one engine on each end).
  len: u32,
  y: u32,
  x: i32,
  orig_x: i32,
}

impl Train {
  pub fn new(len: u32, x: i32, y: u32) -> Self {
    Self {
      len,
      x,
      y,
      orig_x: x,
    }
  }

  pub fn reset(&mut self) {
    self.x = self.orig_x;
  }
}

impl Entity for Train {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    let engine_len = TRAIN_ENGINE[0].chars().count() as u32;
    let cabin_len = TRAIN_CABIN[0].chars().count() as u32;
    let caboose_len = TRAIN_CABOOSE[0].chars().count() as u32;

    Box::new((0..self.len).flat_map(move |car_idx| {
      let car_tiles = if car_idx == 0 {
        &TRAIN_ENGINE
      } else if car_idx == self.len - 1 {
        &TRAIN_CABOOSE
      } else {
        &TRAIN_CABIN
      };

      let offset = if car_idx > 0 {
        engine_len + (car_idx - 1) * cabin_len
      } else {
        0
      } as i32;
      let x_offset = self.x + offset;

      car_tiles.iter().enumerate().flat_map(move |(y, row)| {
        let y = y as i32 + self.y as i32;

        row.chars().enumerate().filter_map(move |(x, c)| {
          if c == ' '
            && ((car_idx == 0 && x < 6)
              || (car_idx == self.len - 1 && x >= caboose_len as usize - 6))
          {
            return None;
          }

          let x = x as i32 + x_offset;
          Some((
            Draw::new(c)
              .with_fg(color::AnsiValue::grayscale(23))
              .with_z(Z_IDX),
            (x, y),
          ))
        })
      })
    }))
  }

  fn tick(&mut self, _t: usize) {
    self.x -= 2;

    let engine_len = TRAIN_ENGINE[0].chars().count() as i32;
    let cabin_len = TRAIN_CABIN[0].chars().count() as i32;
    let caboose_len = TRAIN_CABOOSE[0].chars().count() as i32;
    if engine_len + cabin_len * (self.len as i32 - 2) + caboose_len + self.x < -300 {
      self.reset();
    }
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
