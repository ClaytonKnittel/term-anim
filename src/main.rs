mod basket;
mod bunny;
mod carrot;
mod dialog;
mod entity;
mod grass;
mod hole;
mod landscape;
mod peach;
mod scene;
mod track;
mod train;
mod train_scene;
mod util;
mod water;
mod window;
mod zoom;

use std::time::SystemTime;

use bunny::Bunny;
use entity::Entity;
use rand::{rngs, SeedableRng};
use scene::Scene;
use termion::async_stdin;
use termion::cursor::HideCursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
  let stdout = HideCursor::from(MouseTerminal::from(
    std::io::stdout().lock().into_raw_mode().unwrap(),
  ));
  let mut window = window::Window::new(stdout, 120, 40);
  let mut stdin = async_stdin().events();

  //let guard = pprof::ProfilerGuardBuilder::default()
  //  .frequency(1000)
  //  .blocklist(&["libc", "libgcc", "pthread", "vdso"])
  //  .build()
  //  .unwrap();

  let mut r = rngs::StdRng::seed_from_u64(27418995609531717u64);

  let bunny = Bunny::new(window.width(), window.height(), &mut r);
  // let text = Dialog::new((20, 20), "Sample text".to_string());

  let mut scene = Scene::new();
  scene.add_entity(Box::new(bunny));

  'outer: for t in 0usize.. {
    let start = SystemTime::now();
    for evt in stdin.by_ref() {
      match evt {
        Ok(Event::Key(Key::Char('q'))) => break 'outer,
        Ok(Event::Mouse(me)) => match me {
          MouseEvent::Press(_, x, y) => {
            scene.click(x as u32 - 1, y as u32 - 1);
          }
          MouseEvent::Hold(x, y) => {
            scene.drag(x as u32 - 1, y as u32 - 1);
          }
          MouseEvent::Release(x, y) => {
            scene.release(x as u32 - 1, y as u32 - 1);
          }
        },
        Err(_) => break 'outer,
        _ => {}
      }
    }
    window.reset();
    scene.tick(t);
    scene.render(&mut window);
    window.render().expect("Failed 2 render");
    let end = SystemTime::now();

    let sleep_duration =
      std::time::Duration::from_millis(20).saturating_sub(end.duration_since(start).unwrap());
    std::thread::sleep(sleep_duration);
  }

  //if let Ok(report) = guard.report().build() {
  //  let file = std::fs::File::create("prof.svg").unwrap();
  //  report.flamegraph(file).unwrap();
  //};
}
