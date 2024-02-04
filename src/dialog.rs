use std::iter;

use crate::{entity::Entity, util::Draw};

const MAX_LINE_LEN: usize = 40;
const Z_IDX: i32 = 10;

pub struct Dialog {
  src: (i32, i32),
  text: String,
}

impl Dialog {
  pub fn new(src: (i32, i32), text: String) -> Self {
    Self { src, text }
  }

  fn to_lines(&self) -> Vec<String> {
    let mut text = self.text.as_str();
    let mut lines = Vec::new();
    loop {
      if text.chars().count() <= MAX_LINE_LEN {
        lines.push(text.to_string());
        return lines;
      }
      let last_idx =
        text
          .chars()
          .take(MAX_LINE_LEN)
          .enumerate()
          .fold(
            MAX_LINE_LEN,
            |last_idx, (idx, c)| {
              if c == ' ' {
                idx
              } else {
                last_idx
              }
            },
          );
      lines.push(text[..last_idx].to_string());
      text = &text[last_idx + 1..];
    }
  }
}

/*   ------------
   /              \
  |  Sample text  |
  |              /
  L ------------
*/

impl Entity for Dialog {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    let lines = self.to_lines();
    let num_lines = lines.len() as i32;
    let max_line_len = lines
      .iter()
      .map(|line| line.chars().count())
      .max()
      .expect("Cannot render empty text in dialog box") as i32;

    Box::new(
      [
        (Draw::new('L').with_z(Z_IDX), self.src),
        (Draw::new(' ').with_z(Z_IDX), (self.src.0 + 1, self.src.1)),
        (
          Draw::new(' ').with_z(Z_IDX),
          (self.src.0 + 1, self.src.1 - num_lines - 3),
        ),
        (
          Draw::new(' ').with_z(Z_IDX),
          (self.src.0 + max_line_len + 3, self.src.1),
        ),
        (
          Draw::new(' ').with_z(Z_IDX),
          (self.src.0 + max_line_len + 3, self.src.1 - num_lines - 3),
        ),
        (
          Draw::new('/').with_z(Z_IDX),
          (self.src.0, self.src.1 - num_lines - 2),
        ),
        (
          Draw::new('/').with_z(Z_IDX),
          (self.src.0 + max_line_len + 4, self.src.1 - 1),
        ),
        (
          Draw::new('\\').with_z(Z_IDX),
          (self.src.0 + max_line_len + 4, self.src.1 - num_lines - 2),
        ),
      ]
      .into_iter()
      .chain((0..max_line_len + 1).map(|dx| {
        (
          Draw::new('-').with_z(Z_IDX),
          (self.src.0 + dx + 2, self.src.1),
        )
      }))
      .chain((0..max_line_len + 1).map(move |dx| {
        (
          Draw::new('-').with_z(Z_IDX),
          (self.src.0 + dx + 2, self.src.1 - num_lines - 3),
        )
      }))
      .chain((0..max_line_len + 3).map(|dx| {
        (
          Draw::new(' ').with_z(Z_IDX),
          (self.src.0 + dx + 1, self.src.1 - 1),
        )
      }))
      .chain((0..num_lines).flat_map(move |dy| {
        vec![
          (
            Draw::new(' ').with_z(Z_IDX),
            (self.src.0 + 1, self.src.1 - 2 - dy),
          ),
          (
            Draw::new(' ').with_z(Z_IDX),
            (self.src.0 + 2, self.src.1 - 2 - dy),
          ),
          (
            Draw::new(' ').with_z(Z_IDX),
            (self.src.0 + max_line_len + 3, self.src.1 - 2 - dy),
          ),
          (
            Draw::new(' ').with_z(Z_IDX),
            (self.src.0 + max_line_len + 4, self.src.1 - 2 - dy),
          ),
        ]
        .into_iter()
      }))
      .chain((0..max_line_len + 3).map(move |dx| {
        (
          Draw::new(' ').with_z(Z_IDX),
          (self.src.0 + dx + 1, self.src.1 - num_lines - 2),
        )
      }))
      .chain((0..num_lines + 1).map(|dy| {
        (
          Draw::new('|').with_z(Z_IDX),
          (self.src.0, self.src.1 - dy - 1),
        )
      }))
      .chain((0..num_lines).map(move |dy| {
        (
          Draw::new('|').with_z(Z_IDX),
          (self.src.0 + max_line_len + 5, self.src.1 - dy - 2),
        )
      }))
      .chain(lines.into_iter().enumerate().flat_map(move |(row, line)| {
        line
          .chars()
          .chain(iter::repeat(' '))
          .take(max_line_len as usize)
          .collect::<Vec<_>>()
          .into_iter()
          .enumerate()
          .map(move |(col, c)| {
            (
              Draw::new(c).with_z(Z_IDX),
              (
                self.src.0 + col as i32 + 3,
                self.src.1 - num_lines - 1 + row as i32,
              ),
            )
          })
      })),
    )
  }

  fn tick(&mut self, _t: usize) {}

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
