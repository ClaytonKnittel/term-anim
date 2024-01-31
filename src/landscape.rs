use rand::Rng;

use crate::{entity::Entity, grass::Grass, water::Water};

pub struct Landscape {
  grass: Grass,
  water: Water,
}

impl Landscape {
  pub fn new<R: Rng>(width: u32, height: u32, rand: &mut R) -> Self {
    let mut grass = Grass::new(width, height, rand);
    let mut water = Water::new(width, height);

    let shoreline = height * 5 / 6;
    (shoreline..height).for_each(|y| (0..width).for_each(|x| grass.delete_tile(x, y)));
    (width / 3..width).for_each(|x| grass.delete_tile(x, shoreline - 1));
    (width / 2..width).for_each(|x| grass.delete_tile(x, shoreline - 2));
    (width * 4 / 5..width).for_each(|x| grass.delete_tile(x, shoreline - 3));

    (0..height).for_each(|y| {
      (0..width).for_each(|x| {
        if grass.has_tile(x, y) {
          water.big_fix(x, y);
        }
      })
    });

    Self { grass, water }
  }
}

impl Entity for Landscape {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(self.grass.iterate_tiles().chain(self.water.iterate_tiles()))
  }

  fn tick(&mut self, t: usize) {
    self.grass.tick(t);
    self.water.tick(t);
  }

  fn click(&mut self, x: u32, y: u32) {
    self.grass.click(x, y);
    self.water.click(x, y);
  }

  fn drag(&mut self, dx: i32, dy: i32) {
    self.grass.drag(dx, dy);
    self.water.drag(dx, dy);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.grass.release(x, y);
    self.water.release(x, y);
  }
}
