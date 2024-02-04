use termion::color;

use crate::{dialog::Dialog, entity::Entity, util::Draw};

const Z_IDX: i32 = 10;

enum BunnyState {
  Sleep,
  Wake,
  Walk1,
  Walk2,
  Blink { t: usize },
}

enum Direction {
  Left,
  Right,
}

enum BunnyIntent {
  Idle,
}

enum BunnyStage {
  Sleep1,
  // Wake up, ask for help finding carrot.
  Speak1 { t: usize, dialog_idx: u32 },
  AwaitDecision1,
}

pub struct Bunny {
  state: BunnyState,
  stage: BunnyStage,
  direction: Direction,
  intent: BunnyIntent,
  dialog: Option<Dialog>,
  pos: (i32, i32),
  t: usize,
}

impl Bunny {
  pub fn new(pos: (i32, i32)) -> Self {
    Self {
      state: BunnyState::Sleep,
      stage: BunnyStage::Sleep1,
      direction: Direction::Right,
      intent: BunnyIntent::Idle,
      dialog: None,
      pos,
      t: 0,
    }
  }
}

#[rustfmt::skip]
const RIGHT_SLEEP: [&str; 3] = [
  r#"(\(\"#,
  r#"(-.-)"#,
  r#"o_(")(")"#,
];

#[rustfmt::skip]
const LEFT_SLEEP: [&str; 3] = [
  r#"    /)/)"#,
  r#"   (-.-)"#,
  r#"(")(")_o"#,
];

#[rustfmt::skip]
const RIGHT_WAKE: [&str; 3] = [
  r#"(\(\"#,
  r#"(o.o)"#,
  r#"o_(")(")"#,
];

#[rustfmt::skip]
const LEFT_WAKE: [&str; 3] = [
  r#"    /)/)"#,
  r#"   (o.o)"#,
  r#"(")(")_o"#,
];

#[rustfmt::skip]
const RIGHT_STEP1: [&str; 4] = [
  r#" (\(\"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#" (")(")"#,
];

#[rustfmt::skip]
const RIGHT_STEP2: [&str; 4] = [
  r#" (\(\"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#"(") (")"#,
];

#[rustfmt::skip]
const RIGHT_BLINK: [&str; 4] = [
  r#" (\/)"#,
  r#" (>.<)"#,
  r#" (>_<)"#,
  r#" (")(")"#,
];

#[rustfmt::skip]
const LEFT_STEP1: [&str; 4] = [
  r#"  /)/)"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#"(")(")"#,
];

#[rustfmt::skip]
const LEFT_STEP2: [&str; 4] = [
  r#"  /)/)"#,
  r#" (o.o)"#,
  r#" (>_<)"#,
  r#"(") (")"#,
];

#[rustfmt::skip]
const LEFT_BLINK: [&str; 4] = [
  r#"  (\/)"#,
  r#" (>.<)"#,
  r#" (>_<)"#,
  r#"(")(")"#,
];

impl Entity for Bunny {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (Draw, (i32, i32))> + '_> {
    let bunny_str: &[&str] = match (&self.state, &self.direction) {
      (BunnyState::Sleep, Direction::Left) => &LEFT_SLEEP,
      (BunnyState::Sleep, Direction::Right) => &RIGHT_SLEEP,
      (BunnyState::Wake, Direction::Left) => &LEFT_WAKE,
      (BunnyState::Wake, Direction::Right) => &RIGHT_WAKE,
      (BunnyState::Walk1, Direction::Left) => &LEFT_STEP1,
      (BunnyState::Walk1, Direction::Right) => &RIGHT_STEP1,
      (BunnyState::Walk2, Direction::Left) => &LEFT_STEP2,
      (BunnyState::Walk2, Direction::Right) => &RIGHT_STEP2,
      (BunnyState::Blink { t: _ }, Direction::Left) => &LEFT_BLINK,
      (BunnyState::Blink { t: _ }, Direction::Right) => &RIGHT_BLINK,
    };

    let bunny_iter = bunny_str.iter().enumerate().flat_map(move |(y, row)| {
      let y = y as i32 + self.pos.1;

      row.chars().enumerate().filter_map(move |(x, c)| {
        let x = x as i32 + self.pos.0;
        if c == ' ' {
          return None;
        }

        Some((
          Draw::new(c)
            .with_fg(color::AnsiValue::grayscale(22))
            .with_z(Z_IDX),
          (x, y),
        ))
      })
    });

    match &self.dialog {
      Some(dialog) => Box::new(bunny_iter.chain(dialog.iterate_tiles())),
      None => Box::new(bunny_iter),
    }
  }

  fn tick(&mut self, t: usize) {
    self.t = t;

    match self.stage {
      BunnyStage::Sleep1 => {}
      BunnyStage::Speak1 {
        t: initial_t,
        dialog_idx,
      } => {
        let dt = t - initial_t;

        match dialog_idx {
          0 => {
            if dt == 50 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 5, self.pos.1),
                "Oh! Hello there!".to_string(),
              ));
            }
          }
          1 => {
            if dt == 50 {
              self.state = BunnyState::Walk1;
            } else if dt == 100 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 6, self.pos.1),
                "I am so hungry, and my favorite food is carrots.".to_string(),
              ));
            }
          }
          2 => {
            if dt == 10 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 6, self.pos.1),
                "Would you help me find a carrot?".to_string(),
              ));
              self.stage = BunnyStage::AwaitDecision1;
            }
          }
          _ => unreachable!(),
        }
      }
      BunnyStage::AwaitDecision1 => {
        if let BunnyState::Blink { t: initial_t } = self.state {
          let dt = t - initial_t;
          if dt == 6 {
            self.state = BunnyState::Walk1;
          }
        }
      }
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    let x = x as i32;
    let y = y as i32;
    let clicked_bunny =
      self.pos.0 <= x && x < self.pos.0 + 8 && self.pos.1 <= y && y < self.pos.1 + 4;

    match self.stage {
      BunnyStage::Sleep1 => {
        if clicked_bunny {
          self.stage = BunnyStage::Speak1 {
            t: self.t,
            dialog_idx: 0,
          };
          self.state = BunnyState::Wake;
        }
      }
      BunnyStage::Speak1 { t, dialog_idx } => {
        if match dialog_idx {
          0 => self.t >= t + 50,
          1 => self.t >= t + 100,
          2 => false,
          _ => unreachable!(),
        } {
          self.stage = BunnyStage::Speak1 {
            t: self.t,
            dialog_idx: dialog_idx + 1,
          };
          self.dialog = None;
        }
      }
      BunnyStage::AwaitDecision1 => {
        if clicked_bunny {
          self.state = BunnyState::Blink { t: self.t };
        }
      }
    }
  }

  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
