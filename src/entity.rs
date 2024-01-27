use std::io::Write;

use crate::window::Window;

pub trait Entity {
  fn render<W: Write>(&self, window: &mut Window<W>);
}
