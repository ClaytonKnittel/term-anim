use crate::{entity::Entity, util::Draw};

const Z_IDX: i32 = 1000;

enum State {
  Disabled,
  Zoom {
    t: usize,
    pos: (i32, i32),
    radius: u32,
  },
}

pub struct Zoom {
  t: usize,
  width: u32,
  height: u32,
  state: State,
  state2: State,
  disappear: Option<usize>,
}

impl Zoom {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      t: 0,
      width,
      height,
      state: State::Disabled,
      state2: State::Disabled,
      disappear: None,
    }
  }

  pub fn zoom(&mut self, pos: (i32, i32), radius: u32) {
    self.state = State::Zoom {
      t: self.t,
      pos,
      radius,
    };
  }

  pub fn zoom2(&mut self, pos: (i32, i32), radius: u32) {
    self.state2 = State::Zoom {
      t: self.t,
      pos,
      radius,
    };
  }

  pub fn disappear(&mut self) {
    self.disappear = Some(self.t);
  }
}

impl Entity for Zoom {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (Draw, (i32, i32))> + '_> {
    match self.state {
      State::Disabled => Box::new([].into_iter()) as Box<dyn Iterator<Item = (Draw, (i32, i32))>>,
      State::Zoom { t, pos, radius } => {
        const INIT_RADIUS: i32 = 150;
        let dt = self.t - t;

        let r = (INIT_RADIUS - dt as i32).max(radius as i32);

        let (pos2, r2) = match self.state2 {
          State::Disabled => ((-1, -1), 0.),
          State::Zoom { t, pos, radius } => {
            let dt = self.t - t;
            (pos, (dt as i32).min(radius as i32) as f32)
          }
        };

        let factor = match self.disappear {
          Some(initial_t) => (1. - (self.t - initial_t) as f32 / 50.).max(0.),
          None => 1.,
        };

        Box::new((0..self.height as i32).flat_map(move |y| {
          (0..self.width as i32).filter_map(move |x| {
            let dx = (x - pos.0) as f32;
            let dy = (y - pos.1) as f32 * 11. / 5.;
            let dx2 = (x - pos2.0) as f32;
            let dy2 = (y - pos2.1) as f32 * 11. / 5.;
            if (dx * dx + dy * dy > (r * r) as f32 * factor
              && dx2 * dx2 + dy2 * dy2 > r2 * r2 * factor)
              || factor == 0.
            {
              Some((Draw::new(' ').with_z(Z_IDX), (x, y)))
            } else {
              None
            }
          })
        }))
      }
    }
  }

  fn tick(&mut self, t: usize) {
    self.t = t;
  }

  fn click(&mut self, _x: u32, _y: u32) {}
  fn drag(&mut self, _x: u32, _y: u32) {}
  fn release(&mut self, _x: u32, _y: u32) {}
}
