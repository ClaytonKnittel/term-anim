use crate::entity::Entity;

pub struct Scene {
  entities: Vec<Box<dyn Entity>>,
}

impl Scene {
  pub fn new() -> Self {
    Self { entities: vec![] }
  }

  pub fn add_entity<E: Entity + 'static>(&mut self, entity: E) {
    self.entities.push(Box::new(entity));
  }
}

impl Entity for Scene {
  fn iterate_tiles(&self) -> Box<dyn Iterator<Item = (crate::util::Draw, (i32, i32))> + '_> {
    Box::new(
      self
        .entities
        .iter()
        .flat_map(|entity| entity.iterate_tiles()),
    )
  }

  fn tick(&mut self, t: usize) {
    self.entities.iter_mut().for_each(|entity| entity.tick(t));
  }

  fn click(&mut self, x: u32, y: u32) {
    self
      .entities
      .iter_mut()
      .for_each(|entity| entity.click(x, y));
  }

  fn drag(&mut self, x: u32, y: u32) {
    self
      .entities
      .iter_mut()
      .for_each(|entity| entity.drag(x, y));
  }

  fn release(&mut self, x: u32, y: u32) {
    self
      .entities
      .iter_mut()
      .for_each(|entity| entity.release(x, y));
  }
}
