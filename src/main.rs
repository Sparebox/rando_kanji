use sfml::window::{Event, Key};

mod window;


fn main() {
    let mut window = window::init();
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => { /* Do nothing */ }
            }
        }
    }
}
