use rand::{rngs::StdRng, Rng};
use termion::color;

use crate::{basket::Basket, dialog::Dialog, entity::Entity, train_scene::TrainScene, util::Draw};

const Z_IDX: i32 = 10;
// const STEP_PERIOD: usize = 10;
const STEP_PERIOD: usize = 1;

const LETTERS: [(char, (i32, i32)); 20] = [
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
];

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

#[derive(PartialEq, Eq)]
enum BunnyStage {
  Sleep1,
  // Wake up, ask for help finding carrot.
  Speak1 { t: usize, dialog_idx: u32 },
  AwaitDecision1,
  WalkToBasket { t: usize, init_pos: (i32, i32) },
  BasketDialog { t: usize, dialog_idx: u32 },
  AwaitPeachDestruction,
}

pub struct Bunny<'a> {
  state: BunnyState,
  stage: BunnyStage,
  direction: Direction,
  dialog: Option<Dialog>,
  pos: (i32, i32),
  t: usize,
  basket: Basket,
  train_scene: TrainScene,
  unused_letters: Vec<usize>,
  rng: &'a mut StdRng,
}

impl<'a> Bunny<'a> {
  pub fn new(pos: (i32, i32), width: u32, height: u32, rng: &'a mut StdRng) -> Self {
    Self {
      state: BunnyState::Sleep,
      stage: BunnyStage::Sleep1,
      direction: Direction::Right,
      dialog: None,
      pos,
      t: 0,
      basket: Basket::new((9, 10)),
      train_scene: TrainScene::new(width, height),
      unused_letters: (0..20).collect(),
      rng,
    }
  }

  fn dt_to_completion(&self, init_pos: (i32, i32), target_pos: (i32, i32)) -> usize {
    let dx = target_pos.0 - init_pos.0;
    let dy = target_pos.1 - init_pos.1;
    STEP_PERIOD * (dx.unsigned_abs() as usize + 2 * dy.unsigned_abs() as usize)
  }

  fn interpolate_pos(&mut self, dt: usize, init_pos: (i32, i32), target_pos: (i32, i32)) {
    if dt % STEP_PERIOD != 0 {
      return;
    }
    let step_num = dt / STEP_PERIOD;

    let dx = target_pos.0 - init_pos.0;
    let dy = target_pos.1 - init_pos.1;
    if step_num <= dx.unsigned_abs() as usize {
      if dx < 0 {
        self.direction = Direction::Left;
      } else {
        self.direction = Direction::Right;
      }
      if step_num % 2 != 0 {
        self.pos = (init_pos.0 + step_num as i32 * dx.signum(), init_pos.1);
      }
      self.state = if (dx < 0) ^ (step_num % 2 == 0) {
        BunnyState::Walk2
      } else {
        BunnyState::Walk1
      };
    } else if step_num <= (dx.unsigned_abs() + 2 * dy.unsigned_abs()) as usize {
      if dx < 0 {
        self.direction = Direction::Left;
      } else {
        self.direction = Direction::Right;
      }
      self.state = if (dx < 0) ^ (step_num % 2 == 0) {
        BunnyState::Walk2
      } else {
        BunnyState::Walk1
      };
      self.pos = (
        target_pos.0,
        init_pos.1 + ((step_num as i32 - dx.abs()) / 2) * dy.signum(),
      );
    }
  }

  fn random_guaranteed_letters(&mut self) -> Vec<(char, (i32, i32))> {
    const NUM_TARGETS: u32 = 6;
    const TOTAL_LEN: u32 = LETTERS.len() as u32;

    let idx = self.unused_letters.len() as u32 * NUM_TARGETS / TOTAL_LEN;
    let num_to_take = (idx * TOTAL_LEN / NUM_TARGETS) - ((idx - 1) * TOTAL_LEN / NUM_TARGETS);
    (0..num_to_take)
      .map(|_| {
        let rand_idx = self.rng.gen_range(0..self.unused_letters.len());
        LETTERS[self.unused_letters.remove(rand_idx)]
      })
      .collect()
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

impl<'a> Entity for Bunny<'a> {
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

    let bunny_iter = bunny_str
      .iter()
      .enumerate()
      .flat_map(move |(y, row)| {
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
      })
      .chain(self.basket.iterate_tiles())
      .chain(self.train_scene.iterate_tiles());

    match &self.dialog {
      Some(dialog) => Box::new(bunny_iter.chain(dialog.iterate_tiles())),
      None => Box::new(bunny_iter),
    }
  }

  fn tick(&mut self, t: usize) {
    self.train_scene.tick(t);
    self.basket.tick(t);
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
      BunnyStage::WalkToBasket {
        t: initial_t,
        init_pos,
      } => {
        const TARGET: (i32, i32) = (22, 11);
        let dt = t - initial_t;
        if dt > self.dt_to_completion(init_pos, TARGET) {
          self.stage = BunnyStage::BasketDialog { t, dialog_idx: 0 };
          self.state = BunnyState::Walk1;
        } else {
          self.interpolate_pos(t - initial_t, init_pos, TARGET);
        }
      }
      BunnyStage::BasketDialog {
        t: initial_t,
        dialog_idx,
      } => {
        let dt = t - initial_t;

        match dialog_idx {
          0 => {
            if dt == 50 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 7, self.pos.1),
                "Why, this basket seems to be full of peaches!".to_string(),
              ));
            }
          }
          1 => {
            if dt == 10 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 7, self.pos.1),
                "It's a shame that I don't like peaches. Maybe if I can \
                 figure out how to open a peach, there will be a carrot inside."
                  .to_string(),
              ));
            }
          }
          2 => {
            if dt == 10 {
              self.dialog = Some(Dialog::new(
                (self.pos.0 + 7, self.pos.1),
                "Hey, are those train tracks? Maybe if the peaches collide \
                 with the nose of a passing train, they will open!"
                  .to_string(),
              ));
            }
          }
          _ => unreachable!(),
        }
      }
      BunnyStage::AwaitPeachDestruction => {
        if let BunnyState::Blink { t: initial_t } = self.state {
          let dt = t - initial_t;
          if dt == 6 {
            self.state = BunnyState::Walk1;
          }
        }

        for peach_idx in 0..self.basket.num_peaches() {
          if !self.basket.peach_at_mut(peach_idx).exploded()
            && self
              .train_scene
              .train()
              .collides_with_front(self.basket.peach_at_mut(peach_idx).hitbox())
          {
            let letters = self.random_guaranteed_letters();
            self
              .basket
              .peach_at_mut(peach_idx)
              .explode(letters, self.rng);
          }
        }
      }
    }
  }

  fn click(&mut self, x: u32, y: u32) {
    if let BunnyStage::AwaitPeachDestruction = self.stage {
      self.basket.click(x, y);
    }

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
        if self.basket.contains_click((x, y)) {
          self.stage = BunnyStage::WalkToBasket {
            t: self.t,
            init_pos: self.pos,
          };
          self.dialog = None;
        }
      }
      BunnyStage::WalkToBasket { t: _, init_pos: _ } => {}
      BunnyStage::BasketDialog { t, dialog_idx } => {
        if match dialog_idx {
          0 => self.t >= t + 50,
          1 => self.t >= t + 10,
          2 => self.t >= t + 10,
          _ => unreachable!(),
        } {
          if dialog_idx == 2 {
            self.stage = BunnyStage::AwaitPeachDestruction;
            self.train_scene.unfreeze();
          } else {
            self.stage = BunnyStage::BasketDialog {
              t: self.t,
              dialog_idx: dialog_idx + 1,
            };
          }
          self.dialog = None;
        }
      }
      BunnyStage::AwaitPeachDestruction => {
        if clicked_bunny {
          self.state = BunnyState::Blink { t: self.t };
        }
      }
    }
  }

  fn drag(&mut self, x: u32, y: u32) {
    self.basket.drag(x, y);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.basket.release(x, y);
  }
}
