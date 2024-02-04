use rand::rngs::StdRng;

use crate::{bunny::Bunny, entity::Entity, landscape::Landscape};

pub struct MainScene<'a> {
  bunny: Bunny<'a>,
  landscape: Landscape,
}

impl<'a> MainScene<'a> {
  pub fn new(width: u32, height: u32, r: &'a mut StdRng) -> Self {
    let landscape = Landscape::new(width, height, r);
    Self {
      bunny: Bunny::new(
        (width as i32 / 2 - 10, height as i32 / 2 - 10),
        width,
        height,
        r,
      ),
      landscape,
    }
  }
}

impl<'a> Entity for MainScene<'a> {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      self
        .bunny
        .iterate_tiles()
        .chain(self.landscape.iterate_tiles()),
    )
  }

  fn tick(&mut self, t: usize) {
    self.bunny.tick(t);
    self.landscape.tick(t);
  }

  fn click(&mut self, x: u32, y: u32) {
    self.bunny.click(x, y);
    self.landscape.click(x, y);
  }

  fn drag(&mut self, x: u32, y: u32) {
    self.bunny.drag(x, y);
    self.landscape.drag(x, y);
  }

  fn release(&mut self, x: u32, y: u32) {
    self.bunny.release(x, y);
    self.landscape.release(x, y);
  }
}
