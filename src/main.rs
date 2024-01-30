mod bunny;
mod entity;
mod grass;
mod landscape;
mod scene;
mod track;
mod train;
mod util;
mod water;
mod window;

use std::time::SystemTime;

use bunny::Bunny;
use entity::Entity;
use landscape::Landscape;
use rand::{rngs, SeedableRng};
use scene::Scene;
use termion::async_stdin;
use termion::cursor::HideCursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use track::Track;
use train::Train;

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

  let bunny = Bunny::new();
  let landscape = Landscape::new(window.width(), window.height(), &mut r);
  let track = Track::new(window.height() * 5 / 8, window.width());
  let train = Train::new(5, window.width() as i32, window.height() * 5 / 8 - 2);

  let mut scene = Scene::new();
  scene.add_entity(bunny);
  scene.add_entity(landscape);
  scene.add_entity(track);
  scene.add_entity(train);

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

  //if let Ok(report) = guard.report().build() {
  //  let file = std::fs::File::create("prof.svg").unwrap();
  //  report.flamegraph(file).unwrap();
  //};
}
