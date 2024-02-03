use termion::color;

use crate::{dialog::Dialog, entity::Entity, util::Draw};

const Z_IDX: i32 = 10;

enum BunnyState {
  Sleep,
  Wake,
  Walk1,
  Walk2,
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
  Speak1 { t: usize },
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

  pub fn shift(&mut self) {
    match self.intent {
      BunnyIntent::Idle => {}
    }
    // match self.state {
    //   BunnyState::Wake => {}
    //   BunnyState::Sleep => {}
    //   BunnyState::Walk1 => {
    //     self.state = BunnyState::Walk2;
    //     self.pos.0 += 1;
    //   }
    //   BunnyState::Walk2 => {
    //     self.state = BunnyState::Walk1;
    //   }
    // };
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
      BunnyStage::Speak1 { t: initial_t } => {
        let dt = t - initial_t;

        if dt == 50 {
          self.dialog = Some(Dialog::new(
            (self.pos.0 + 5, self.pos.1),
            "Oh! Hello there!".to_string(),
          ));
        } else if dt == 150 {
          self.dialog = None;
        } else if dt == 200 {
          self.state = BunnyState::Walk1;
        } else if dt == 250 {
          self.dialog = Some(Dialog::new(
            (self.pos.0 + 4, self.pos.1),
            "I am so hungry, and my favorite food is carrots.".to_string(),
          ));
        } else if dt == 380 {
          self.dialog = None;
        } else if dt == 400 {
          self.dialog = Some(Dialog::new(
            (self.pos.0 + 4, self.pos.1),
            "Would you help me find a carrot?".to_string(),
          ));
        } else if dt == 500 {
          self.dialog = None;
        }
      }
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    let x = x as i32;
    let y = y as i32;
    if self.pos.0 <= x && x < self.pos.0 + 8 && self.pos.1 <= y && y < self.pos.1 + 4 {
      match self.stage {
        BunnyStage::Sleep1 => {
          self.stage = BunnyStage::Speak1 { t: self.t };
          self.state = BunnyState::Wake;
        }
        BunnyStage::Speak1 { t: _ } => {}
      }
    }
  }

  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
