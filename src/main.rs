mod window;

fn main() {
  let mut window = window::Window::new(10, 10);

  for _ in 0..10 {
    window.render();
    std::thread::sleep(std::time::Duration::from_millis(200));
  }
}
