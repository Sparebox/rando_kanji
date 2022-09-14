use app::App;
use audio::SoundBuffers;
use egui_sfml::egui;
use game_state::{GameState::{self, Menu, Play, Options}};
use sfml::graphics::{Color, RenderTarget};

mod kanji;
mod window;
mod game_state;
mod audio;
mod config;
mod app;

fn main() {
    let sounds = SoundBuffers::new();
    let mut app = App::new(&sounds);
    app.change_state(GameState::Menu);
    
    // Update loop
    while app.window.is_open() {
        if app.is_switching_state {
            app.is_switching_state = false;
            match app.current_state {
                Menu    => GameState::init_menu_state(&mut app),
                Options => GameState::init_options_state(&mut app),
                Play    => GameState::init_play_state(&mut app),
            }
        }

        app.window.clear(Color::rgb(10, 10, 10));
        window::handle_events(&mut app);
        app.draw();
        //app.egui.draw(&mut app.window, None);
        app.window.display();
    }
    // Save configurations to disk
    app.config.to_file(App::CONFIG_PATH);
}

