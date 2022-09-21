use sfml::{
    graphics::RenderWindow,
    system::Vector2i,
    window::{Event, Key, Style, VideoMode},
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
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => app.window.close(),
            Event::MouseButtonPressed { button: _, x, y } => {
                app.update_buttons(Vector2i::new(x, y), true)
            }
            Event::MouseMoved { x, y } => app.update_buttons(Vector2i::new(x, y), false),
            _ => { /* Do nothing */ }
        }
    }
}

#[derive(Clone, Copy)]
pub enum ViewEnum {
    GameButtonsView,
    DefaultView,
}

pub mod ui {
    use egui_sfml::egui::{self, Context, FontFamily, FontId, TextStyle};
    use sfml::{
        graphics::{
            Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Text, Transformable, Font, View,
        },
        system::{Vector2f, Vector2i},
    };

    use crate::{
        app::App,
        config::Config,
        game_state::GameState::{self},
    };

    use super::ViewEnum;

    pub fn draw(app: &mut App) {
        app.egui.do_frame(|ctx| match app.current_state {
            GameState::Menu => draw_menu_egui(
                &mut app.window,
                &mut app.current_state,
                &mut app.is_switching_state,
                ctx),
            GameState::Play => draw_play_egui(
                &mut app.current_state,
                &mut app.is_switching_state,
                &mut app.window,
                &app.main_view,
                ctx),
            GameState::Options => draw_options_egui(
                &mut app.config,
                &mut app.current_state,
                &mut app.is_switching_state,
                &mut app.showing_confirm_dialog,
                ctx,
            ),
        });
        app.egui.draw(&mut app.window, None);
    }

    pub fn set_custom_egui_font(ctx: &Context) {
        let mut font_defs = egui::FontDefinitions::default();
        font_defs.font_data.insert(
            "Honoka-Shin".to_string(),
            egui::FontData::from_static(include_bytes!(
                "../res/font/Honoka-Shin-Antique-Maru_R.otf"
            )),
        );
        font_defs
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Honoka-Shin".to_string());

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(30.0, FontFamily::Proportional),
            ),
            (   
                TextStyle::Body,
                FontId::new(50.0, FontFamily::Proportional)),
            (
                TextStyle::Monospace,
                FontId::new(14.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(50.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(10.0, FontFamily::Proportional),
            ),
        ]
        .into();
        style.spacing.item_spacing = egui::vec2(20.0, 20.0);
        ctx.set_fonts(font_defs);
        ctx.set_style(style);
    }

    fn draw_play_egui(
        state: &mut GameState,
        is_switching_state: &mut bool,
        window: &mut RenderWindow,
        main_view: &View,
        ctx: &Context
    ) {
        window.set_view(main_view);
        egui::Area::new("ButtonArea")
            .movable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::default())
            .show(ctx, |ui| {
                
                if ui.button("Menu").clicked() {
                    *is_switching_state = true;
                    *state = GameState::Menu;
                }
                ui.add_space(20.0);
            });
    }

    fn draw_options_egui(
        config: &mut Config,
        state: &mut GameState,
        is_switching_state: &mut bool,
        showing_dialog: &mut bool,
        ctx: &Context,
    ) {
        egui::Area::new("Configurations")
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
            .show(ctx, |ui| {
                if *showing_dialog {
                    ui.set_enabled(false);
                }
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui.checkbox(&mut config.romaji_enabled, "Toggle Rōmaji ローマ字").clicked() && config.show_meaning_enabled {
                        config.show_meaning_enabled = false;
                    }
                    if ui.checkbox(&mut config.show_meaning_enabled, "Show meaning 意味付き").clicked() && config.show_meaning_enabled {
                        config.romaji_enabled = false;
                    }
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui.button("Back").clicked() {
                        *state = GameState::Menu;
                        *is_switching_state = true;
                    }
                    ui.add_space(20.0);
                    if ui.button("Reset configurations").clicked() {
                        *showing_dialog = !*showing_dialog;
                    }
                });
        });
        if *showing_dialog {
            egui::Window::new("Reset configurations and statistics?")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 100.0))
                .show(ctx, |ui| {
                    ui.horizontal_top(|ui|{
                        if ui.button("Yes").clicked() {
                            *config = Config::default();
                            *showing_dialog = false;
                        }
                        if ui.button("Cancel").clicked() {
                            *showing_dialog = false;
                        }
                    })
                });
        }
    }

    fn draw_menu_egui(window: &mut RenderWindow, state: &mut GameState, is_switching_state: &mut bool, ctx: &Context) {
        egui::Area::new("MenuArea")
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Play").clicked() {
                        *is_switching_state = true;
                        *state = GameState::Play;
                    }
                    ui.add_space(50.0);
                    if ui.button("Options").clicked() {
                        *is_switching_state = true;
                        *state = GameState::Options;
                    }
                    ui.add_space(50.0);
                    if ui.button("Exit").clicked() {
                        window.close();
                    }
                });
            });
    }

    #[derive(Clone, Copy)]
    pub struct AnswerData {
        pub correct_index: u8,
        pub index_to_test: u8,
        pub button_id: u8,
        pub kanji: char,
    }

    #[derive(Clone, Copy)]
    pub enum ButtonAction {
        CheckAnswer(AnswerData),
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
        pub view: ViewEnum,
        pub action: ButtonAction,
        pub id: u8,
        color_overridden: bool,
    }

    impl<'a> TextButton<'a> {
        pub fn new(
            string: &str,
            pos: Vector2f,
            fg_color: Color,
            bg_color: Color,
            font: &Font,
            action: ButtonAction,
            view: ViewEnum,
        ) -> Self {
            let text = TextDescriptor::new(string, pos, fg_color, true);
            let mut button_dimensions =
                Text::new(string, font, App::FONT_SIZE).global_bounds();
            button_dimensions.width += App::FONT_SIZE as f32;
            button_dimensions.height += App::FONT_SIZE as f32;

            let mut shape = RectangleShape::from_rect(button_dimensions);
            shape.set_position(pos - button_dimensions.size() / 2.0 + Vector2f::new(3.0, 10.0));
            shape.set_outline_color(bg_color);
            shape.set_outline_thickness(2.0);
            shape.set_fill_color(Color::TRANSPARENT);
            Self {
                text,
                shape,
                action,
                view,
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
            if self.shape.global_bounds().contains(mouse_pos) && sfml::window::mouse::Button::Left.is_pressed() {
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
