use app::App;
use audio::SoundBuffers;
use game_state::GameState::{self, Menu, Options, Play};
use kanji_fall::KanjiFall;
use sfml::graphics::RenderTarget;
use window::ui;

mod app;
mod audio;
mod config;
mod game_state;
mod kanji;
mod utils;
mod window;
mod kanji_fall;

fn main() {
    let sounds = SoundBuffers::new();
    let mut app = App::new(&sounds);
    app.change_state(GameState::Menu);
    let mut kanji_fall = KanjiFall::new(); // Manages falling kanji background effect

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
        window::handle_events(&mut app);
        if app.current_state == GameState::Menu { // Update falling kanji animation in the background
            kanji_fall.update(&mut app.texts, &app.kanji_dealer.kanjis);
            app.window.clear(App::MENU_BACKGROUND_COLOR);
        } else {
            app.window.clear(App::GAME_BACKGROUND_COLOR);
        }
        app.draw();
        ui::draw(&mut app);
        app.window.display();
    }
    // Save current profile configurations to disk
    app.config.save();
}
