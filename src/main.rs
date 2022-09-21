use app::App;
use audio::SoundBuffers;
use game_state::GameState::{self, Menu, Options, Play};
use sfml::graphics::{Color, RenderTarget};
use window::ui;

mod app;
mod audio;
mod config;
mod game_state;
mod kanji;
mod window;
mod utils;

fn main() {
    let sounds = SoundBuffers::new();
    let mut app = App::new(&sounds);
    app.change_state(GameState::Menu);

    // Update loop
    while app.window.is_open() {
        if app.is_switching_state {
            match app.current_state {
                Menu => GameState::init_menu_state(&mut app),
                Options => GameState::init_options_state(&mut app),
                Play => GameState::init_play_state(&mut app),
            }
            app.is_switching_state = false;
        }

        app.window.clear(Color::rgb(10, 10, 10));
        window::handle_events(&mut app);
        app.draw();
        ui::draw(&mut app);
        app.window.display();
    }
    // Save configurations to disk
    app.config.to_file(App::CONFIG_PATH);
}
