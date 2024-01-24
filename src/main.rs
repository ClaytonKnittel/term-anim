mod window;

use std::io::Write;
use termion::async_stdin;
use termion::cursor::HideCursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
  let stdout = HideCursor::from(MouseTerminal::from(
    std::io::stdout().lock().into_raw_mode().unwrap(),
  ));
  let mut window = window::Window::new(stdout, 60, 40);
  let mut stdin = async_stdin().events();

  'outer: loop {
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
    std::thread::sleep(std::time::Duration::from_millis(20));
    for _ in 0..10 {
      window.advance();
    }
    window.render().unwrap();
  }
}
