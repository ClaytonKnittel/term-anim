mod window;

use std::time::SystemTime;

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

  let guard = pprof::ProfilerGuardBuilder::default()
    .frequency(1000)
    .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    .build()
    .unwrap();

  'outer: loop {
    let start = SystemTime::now();
    while let Some(evt) = stdin.next() {
      match evt {
        Ok(Event::Key(Key::Char('q'))) => break 'outer,
        Ok(Event::Mouse(me)) => match me {
          MouseEvent::Press(_, x, y) | MouseEvent::Hold(x, y) => {
            window.click(x as u32, y as u32);
          }
          _ => (),
        },
        Err(_) => break 'outer,
        _ => {}
      }
    }
    for _ in 0..7 {
      window.advance();
    }
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
