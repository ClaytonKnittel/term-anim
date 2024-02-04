use termion::color;

use crate::{entity::Entity, peach::Peach, util::Draw};

const Z_IDX: i32 = 25;
const BG_Z_IDX: i32 = 20;

#[rustfmt::skip]
const BASKET: [&str; 6] = [
  r#"   ======   "#,
  r#" //      \\ "#,
  r#"||        ||"#,
  r#"============"#,
  r#" \\##||##// "#,
  r#"   -~~~~-   "#,
];

pub struct Basket {
  pos: (i32, i32),
  peaches: Vec<Peach>,
}

impl Basket {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      pos,
      peaches: vec![
        Peach::new(pos.0 + 2, pos.1 + 1, color::AnsiValue::rgb(5, 0, 0)),
        Peach::new(pos.0 + 5, pos.1 + 1, color::AnsiValue::rgb(5, 0, 1)),
        Peach::new(pos.0 + 8, pos.1 + 1, color::AnsiValue::rgb(4, 0, 0)),
        Peach::new(pos.0 + 4, pos.1, color::AnsiValue::rgb(5, 1, 0)),
      ],
    }
  }

  pub fn contains_click(&self, pos: (i32, i32)) -> bool {
    let dx = pos.0 - self.pos.0;
    let dy = pos.1 - self.pos.1;
    (0..12).contains(&dx) && (0..6).contains(&dy) && ((dx != 0 && dx != 11) || dy != 0)
  }

  pub fn peaches_mut(&mut self) -> impl Iterator<Item = &mut Peach> {
    self.peaches.iter_mut()
  }

  pub fn num_peaches(&self) -> usize {
    self.peaches.len()
  }

  pub fn peach_at_mut(&mut self, idx: usize) -> &mut Peach {
    &mut self.peaches[idx]
  }

  pub fn splode(&mut self) {
    self.peaches.first_mut().unwrap().explode(vec![
      ('H', (60, 5)),
      ('a', (61, 5)),
      ('p', (62, 5)),
      ('p', (63, 5)),
      ('y', (64, 5)),
      ('B', (59, 7)),
      ('i', (60, 7)),
      ('r', (61, 7)),
      ('t', (62, 7)),
      ('h', (63, 7)),
      ('d', (64, 7)),
      ('a', (65, 7)),
      ('y', (66, 7)),
      ('E', (60, 9)),
      ('u', (61, 9)),
      ('g', (62, 9)),
      ('e', (63, 9)),
      ('n', (64, 9)),
      ('i', (65, 9)),
      ('a', (66, 9)),
    ])
  }
}

impl Entity for Basket {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      BASKET
        .iter()
        .enumerate()
        .flat_map(move |(dy, row)| {
          let y = dy as i32 + self.pos.1;

          row.chars().enumerate().filter_map(move |(dx, c)| {
            let x = dx as i32 + self.pos.0;
            if c == ' ' && !(1..=2).contains(&dy) {
              return None;
            }

            Some((
              Draw::new(c)
                .with_fg(color::AnsiValue::rgb(2, 1, 0))
                .with_z(if c == ' ' { BG_Z_IDX } else { Z_IDX }),
              (x, y),
            ))
          })
        })
        .chain(self.peaches.iter().flat_map(|peach| peach.iterate_tiles())),
    )
  }

  fn tick(&mut self, t: usize) {
    for peach in self.peaches.iter_mut() {
      peach.tick(t);
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    for peach in self.peaches.iter_mut() {
      peach.click(x, y);
    }
  }

  fn drag(&mut self, x: u32, y: u32) {
    for peach in self.peaches.iter_mut() {
      peach.drag(x, y);
    }
  }

  fn release(&mut self, x: u32, y: u32) {
    for peach in self.peaches.iter_mut() {
      peach.release(x, y);
    }
  }
}
