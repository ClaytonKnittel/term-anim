use termion::color;

use crate::{
  entity::Entity,
  util::{explosion_path, Draw},
};

const Z_IDX: i32 = 20;
const DEBRIS_Z_IDX: i32 = 6;

#[rustfmt::skip]
const HOLE: [&str; 4] = [
  r#" ===L"#,
  r#"H   H"#,
  r#"HL _H"#,
  r#" === "#,
];

pub struct Hole {
  t: usize,
  pos: (i32, i32),
  queued_dirt: Vec<(bool, char, (i32, i32))>,
  flung_dirt: Vec<(usize, bool, char, (i32, i32))>,
  kazoo: Option<usize>,
}

impl Hole {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      t: 0,
      pos,
      queued_dirt: Vec::new(),
      flung_dirt: Vec::new(),
      kazoo: None,
    }
  }

  pub fn contains_click(&self, pos: (i32, i32)) -> bool {
    let dx = pos.0 - self.pos.0;
    let dy = pos.1 - self.pos.1;
    (0..5).contains(&dx) && (0..4).contains(&dy)
  }

  pub fn add_dirt(&mut self, dirt: Vec<(bool, char, (i32, i32))>) {
    self.queued_dirt = dirt;
  }

  pub fn fling(&mut self) -> bool {
    match self.queued_dirt.pop() {
      Some(dirt) => {
        self.flung_dirt.push((self.t, dirt.0, dirt.1, dirt.2));
        true
      }
      None => {
        self.kazoo = Some(self.t);
        false
      }
    }
  }
}

impl Entity for Hole {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      HOLE
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
          let y = y as i32 + self.pos.1;

          row.chars().enumerate().filter_map(move |(x, c)| {
            let x = x as i32 + self.pos.0;
            if (x == 0 || x == 3) && (y == 0 || y == 3) && c == ' ' {
              return None;
            }

            Some((
              Draw::new(c)
                .with_fg(color::AnsiValue::rgb(2, 1, 0))
                .with_z(Z_IDX),
              (x, y),
            ))
          })
        })
        .chain(self.flung_dirt.iter().map(|(t, targeted, c, (x, y))| {
          (
            Draw::new(*c)
              .with_fg(color::AnsiValue::rgb(2, 1, 0))
              .with_z(DEBRIS_Z_IDX + if *targeted { 1 } else { 0 }),
            explosion_path(
              (self.t - t) as f32,
              (*x, *y),
              (self.pos.0 + 2, self.pos.1 + 2),
            ),
          )
        }))
        .chain(match self.kazoo {
          Some(kazoo_t) => Box::new(
            [(
              Draw::new('/')
                .with_fg(color::AnsiValue::rgb(4, 0, 0))
                .with_z(Z_IDX),
              explosion_path(
                (self.t - kazoo_t) as f32,
                (24, 5),
                (self.pos.0 + 2, self.pos.1 + 2),
              ),
            )]
            .into_iter(),
          ) as Box<dyn Iterator<Item = (Draw, (i32, i32))>>,
          None => Box::new([].into_iter()),
        }),
    )
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
