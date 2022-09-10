use sfml::{
    graphics::RenderWindow,
    window::{Event, Key, Style, VideoMode}, system::Vector2i,
};

use crate::App;

pub fn init() -> RenderWindow {
    let mut window = RenderWindow::new(
        VideoMode::new(App::INIT_WIN_SIZE.x, App::INIT_WIN_SIZE.y, 16),
        "Rando Kanji ・ ランド漢字",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_framerate_limit(App::FPS_LIMIT);
    window
}

pub fn handle_events(app: &mut App) {
    while let Some(event) = app.window.poll_event() {
        match event {
            Event::Resized { width, height } => {
                app.on_resize(width as f32, height as f32);
            },
            Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => app.window.close(),
            Event::MouseButtonPressed { button: _, x, y } => app.update_buttons(Vector2i::new(x, y), true),
            Event::MouseMoved { x, y } => app.update_buttons(Vector2i::new(x, y), false),
            _ => {/* Do nothing */},
        }
    }
}

pub mod ui {
    use sfml::{system::{Vector2f, Vector2i}, graphics::{Color, RectangleShape, Transformable, Shape, Text, Rect}};

    use crate::App;

    #[derive(Clone)]
    pub enum ButtonAction {
        GotoGame,
        GotoOptions,
        GotoMenu,
        CheckAnswer,
        ExitGame,
    }

    #[derive(Clone)]
    pub struct TextDescriptor {
        pub string: String,
        pub pos: Vector2f,
        pub bounds: Rect<f32>,
        pub color: Color,
        pub font_base_size: u32,
        pub center: bool,
    }

    impl TextDescriptor {
        pub fn new(string: &str, pos: Vector2f, color: Color, center: bool) -> Self {
            Self {
                string: string.to_string(),
                pos,
                bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
                color,
                font_base_size: 0,
                center,
            }
        }

        pub fn as_sf_text(&mut self, sf_text: &mut Text, font_height: u32) {
            sf_text.set_string(&self.string);
            sf_text.set_position(self.pos);
            sf_text.set_fill_color(self.color);
            sf_text.set_character_size(self.font_base_size + font_height);
            self.bounds = sf_text.global_bounds();
            if self.center {
                let width = sf_text.global_bounds().width;
                let height = sf_text.global_bounds().height;
                sf_text.move_(Vector2f::new(-width / 2.0, -height / 2.0));
            }
        }
    }

    #[derive(Clone)]
    pub struct TextButton<'a> {
        pub text: TextDescriptor,
        pub shape: RectangleShape<'a>,
        pub action: ButtonAction,
    }

    impl <'a>TextButton<'a> {
        pub fn new(string: &str, pos: Vector2f, fg_color: Color, bg_color: Color, app: &App, action: ButtonAction) -> Self {
            let text = TextDescriptor::new(string, pos, fg_color, true);
            let mut button_dimensions = Text::new(string, &app.font, app.font_height).global_bounds();
            button_dimensions.width += app.font_height as f32 / 2.0;
            button_dimensions.height += app.font_height as f32 / 2.0;

            let mut shape = RectangleShape::from_rect(button_dimensions);
            shape.set_position(pos - button_dimensions.size() / 2.0 + Vector2f::new(0.0, 10.0));
            shape.set_outline_color(bg_color);
            shape.set_outline_thickness(1.0);
            shape.set_fill_color(Color::TRANSPARENT);
            Self {
                text,
                shape,
                action
            }
        }

        pub fn check_for_mouse_hover(&mut self, mouse_pos: Vector2i) {
            let mouse_pos = Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
            if self.shape.global_bounds().contains(mouse_pos) {
                self.shape.set_outline_color(Color::GREEN);
                self.text.color = Color::GREEN;
            } else {
                self.shape.set_outline_color(Color::WHITE);
                self.text.color = Color::WHITE;
            }
        }

        pub fn check_for_mouse_press(&self, mouse_pos: Vector2i, app: &mut App) {
            let mouse_pos = Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
            if self.shape.global_bounds().contains(mouse_pos) {
                app.execute_button_action(&self.action);
            }
        }
    }
}