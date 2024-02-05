use termion::color::{self, AnsiValue};

use crate::{
  entity::Entity,
  util::{explosion_path, move_per_radiate, Draw, Radiate},
  water::Water,
};

const Z_IDX: i32 = 20;
const DEBRIS_Z_IDX: i32 = 6;
const FLYING_DEBRIS_Z_IDX: i32 = 28;

#[rustfmt::skip]
const HOLE: [&str; 4] = [
  r#" ===L"#,
  r#"H   H"#,
  r#"HL _H"#,
  r#" === "#,
];

const fn const_rgb(r: u8, g: u8, b: u8) -> color::AnsiValue {
  AnsiValue(16 + 36 * r + 6 * g + b)
}

const KAZOO: [(char, (i32, i32), color::AnsiValue, color::AnsiValue); 11] = [
  ('/', (0, 0), const_rgb(4, 0, 0), const_rgb(4, 0, 0)),
  ('\\', (-1, -1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('|', (0, -1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('/', (1, -1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('-', (-1, 0), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('-', (-2, 0), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('-', (1, 0), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('-', (2, 0), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('/', (-1, 1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('|', (0, 1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
  ('\\', (1, 1), const_rgb(5, 5, 5), const_rgb(2, 2, 2)),
];

pub struct Hole {
  t: usize,
  pos: (i32, i32),
  queued_dirt: Vec<(bool, char, (i32, i32))>,
  flung_dirt: Vec<(usize, bool, char, (i32, i32))>,
  kazoo: Option<(usize, (i32, i32))>,
  radiate: Option<Radiate>,
}

impl Hole {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      t: 0,
      pos,
      queued_dirt: Vec::new(),
      flung_dirt: Vec::new(),
      kazoo: None,
      radiate: None,
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
        self.kazoo = Some((self.t, (24, 5)));
        false
      }
    }
  }

  pub fn set_kazoo_pos(&mut self, pos: (i32, i32)) {
    self.kazoo = self.kazoo.map(|(t, _)| (t, pos))
  }

  pub fn radiate(&mut self, pos: (i32, i32)) {
    self.radiate = Some(Radiate { t: self.t, pos });
  }

  fn debris_pos(
    &self,
    (t, targeted, _, (x, y)): &(usize, bool, char, (i32, i32)),
  ) -> ((i32, i32), bool) {
    let mut pos = explosion_path(
      (self.t - t) as f32,
      (*x, *y),
      (self.pos.0 + 2, self.pos.1 + 2),
    );
    let resting = pos == (*x, *y);
    if !targeted {
      pos = move_per_radiate(&self.radiate, self.t, pos);
    }
    (pos, resting)
  }

  pub fn maybe_dunk(&mut self, water: &mut Water) {
    self.flung_dirt = self
      .flung_dirt
      .clone()
      .into_iter()
      .filter(|&(t, targeted, c, pos)| {
        let (pos, resting) = self.debris_pos(&(t, targeted, c, pos));
        if resting && water.is_wet(pos) {
          water.click(pos.0 as u32, pos.1 as u32);
          false
        } else {
          true
        }
      })
      .collect();
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
          let (pos, resting) = self.debris_pos(&(*t, *targeted, *c, (*x, *y)));
          let z_idx = if resting {
            DEBRIS_Z_IDX
          } else {
            FLYING_DEBRIS_Z_IDX
          };
          (
            Draw::new(*c)
              .with_fg(color::AnsiValue::rgb(2, 1, 0))
              .with_z(z_idx + if *targeted { 1 } else { 0 }),
            pos,
          )
        }))
        .chain(match self.kazoo {
          Some((kazoo_t, kazoo_pos)) => {
            Box::new(KAZOO.iter().map(move |(c, (dx, dy), color1, color2)| {
              let (x, y) = explosion_path(
                (self.t - kazoo_t) as f32,
                kazoo_pos,
                (self.pos.0 + 2, self.pos.1 + 2),
              );
              let color = if (self.t / 30) % 2 == 0 {
                *color1
              } else {
                *color2
              };
              (Draw::new(*c).with_fg(color).with_z(Z_IDX), (x + dx, y + dy))
            })) as Box<dyn Iterator<Item = (Draw, (i32, i32))>>
          }
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
