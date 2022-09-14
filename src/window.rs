use sfml::{
    graphics::{RenderWindow},
    window::{Event, Key, Style, VideoMode}, system::Vector2i,
};

use crate::app::App;

pub fn init() -> RenderWindow {
    let mut window = RenderWindow::new(
        VideoMode::new(App::INIT_WIN_SIZE.x, App::INIT_WIN_SIZE.y, 16),
        "Rando Kanji ・ ランド漢字",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(App::FPS_LIMIT);
    window
}

pub fn handle_events(app: &mut App) {
    while let Some(event) = app.window.poll_event() {
        app.egui.add_event(&event);
        match event {
            Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => app.window.close(),
            Event::MouseButtonPressed { button: _, x, y } => app.update_buttons(Vector2i::new(x, y), true),
            Event::MouseMoved { x, y } => app.update_buttons(Vector2i::new(x, y), false),
            _ => {/* Do nothing */},
        }
    }
}

#[derive(Clone, Copy)]
pub enum ViewEnum {
    GameButtonsView,
    DefaultView,
}

pub mod ui {
    use sfml::{system::{Vector2f, Vector2i}, graphics::{
        Color, RectangleShape, Transformable, Shape, Text, Rect, RenderTarget, RenderWindow}};

    use crate::app::App;

    use super::ViewEnum;

    #[derive(Clone, Copy)]
    pub struct AnswerData {
        pub correct_index: u8,
        pub index_to_test: u8,
        pub button_id: u8,
        pub kanji: char,
    }

    #[derive(Clone, Copy)]
    pub enum ButtonAction {
        GotoGame,
        GotoOptions,
        GotoMenu,
        CheckAnswer(AnswerData),
        ToggleRomaji,
        ResetConfig,
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

        pub fn as_sf_text(&mut self, sf_text: &mut Text) {
            sf_text.set_string(&self.string);
            sf_text.set_position(self.pos);
            sf_text.set_fill_color(self.color);
            sf_text.set_character_size(self.font_base_size + App::FONT_SIZE);
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
        pub view_index: ViewEnum,
        pub action: ButtonAction,
        pub id: u8,
        color_overridden: bool,
    }

    impl <'a>TextButton<'a> {
        pub fn new(
            string: &str, 
            pos: Vector2f, 
            fg_color: Color, 
            bg_color: Color, 
            app: &App, 
            action: ButtonAction,
            view_index: ViewEnum,
        ) -> Self {
            let text = TextDescriptor::new(string, pos, fg_color, true);
            let mut button_dimensions = Text::new(string, &app.font, App::FONT_SIZE).global_bounds();
            button_dimensions.width += App::FONT_SIZE as f32 / 2.0;
            button_dimensions.height += App::FONT_SIZE as f32 / 2.0;

            let mut shape = RectangleShape::from_rect(button_dimensions);
            shape.set_position(pos - button_dimensions.size() / 2.0 + Vector2f::new(0.0, 10.0));
            shape.set_outline_color(bg_color);
            shape.set_outline_thickness(2.0);
            shape.set_fill_color(Color::TRANSPARENT);
            Self {
                text,
                shape,
                action,
                view_index,
                id: Self::generate_id_from_pos(pos),
                color_overridden: false,
            }
        }

        pub fn generate_id_from_pos(pos: Vector2f) -> u8 {
            (pos.x + pos.y) as u8
        }

        pub fn check_for_mouse_hover(&mut self, mouse_pos: Vector2i) {
            let mouse_pos = Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
            if self.color_overridden {
                return;
            }
            if self.shape.global_bounds().contains(mouse_pos) {
                self.set_color(Color::GREEN, false);
            } else {
                self.set_color(Color::WHITE, false);
            }
        }

        pub fn check_for_mouse_press(&self, mouse_pos: Vector2i) -> Option<ButtonAction> {
            let mouse_pos = Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
            if self.shape.global_bounds().contains(mouse_pos) {
               Some(self.action)
            } else {
                None
            }
        }

        pub fn set_color(&mut self, color: Color, lock_color: bool) {
            self.color_overridden = lock_color;
            self.shape.set_outline_color(color);
            self.text.color = color;
        }

        pub fn get_width(&self) -> f32 {
            self.shape.size().x
        }

        pub fn get_height(&self) -> f32 {
            self.shape.size().y
        }

        pub fn draw(&mut self, window: &mut RenderWindow, text: &mut Text) {
            self.text.as_sf_text(text);
            window.draw(&self.shape);
            window.draw(text);
        }

    }
}