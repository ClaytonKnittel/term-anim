mod bunny;
mod entity;
mod scene;
mod util;
mod water;
mod window;

use std::time::SystemTime;

use bunny::Bunny;
use entity::Entity;
use scene::Scene;
use termion::async_stdin;
use termion::cursor::HideCursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use water::Water;

fn main() {
  let stdout = HideCursor::from(MouseTerminal::from(
    std::io::stdout().lock().into_raw_mode().unwrap(),
  ));
  let mut window = window::Window::new(stdout, 120, 40);
  let mut stdin = async_stdin().events();

  let guard = pprof::ProfilerGuardBuilder::default()
    .frequency(1000)
    .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    .build()
    .unwrap();

  let bunny = Bunny::new();
  let water = Water::new(window.width(), window.height());

  let mut scene = Scene::new();
  scene.add_entity(bunny);
  scene.add_entity(water);

  'outer: for t in 0usize.. {
    let start = SystemTime::now();
    for evt in stdin.by_ref() {
      match evt {
        Ok(Event::Key(Key::Char('q'))) => break 'outer,
        Ok(Event::Mouse(me)) => match me {
          MouseEvent::Press(_, x, y) | MouseEvent::Hold(x, y) => {
            scene.click(x as u32, y as u32);
          }
          _ => (),
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

  if let Ok(report) = guard.report().build() {
    let file = std::fs::File::create("prof.svg").unwrap();
    report.flamegraph(file).unwrap();
  };
}
