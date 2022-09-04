use std::{cell::RefCell, rc::Rc};

use sfml::{graphics::{Text, Transformable, RenderTarget, Color}, system::{Vector2f, Vector2i}, window::Event};

use crate::{window::ui::{TextDescriptor, TextButton}, App};

pub enum StateEnum {
    Menu,
    Play,
}

pub trait GameState {
    fn update(&mut self, app: &mut App);
    fn draw(&mut self, app: &mut App);
    fn on_resize(&mut self, width: f32, height: f32, app: &App);
    fn handle_events(&self, app: &mut App, event: &Event);
    fn update_buttons(&self, app: &mut App, mouse_pos: Vector2i, check_press: bool);
}

pub struct MenuState<'a> {
    pub texts: Vec<TextDescriptor>,
    pub buttons: Rc<RefCell<Vec<TextButton<'a>>>>
}

impl <'a>GameState for MenuState<'a> {
    fn update(&mut self, app: &mut App) {
        
    }

    fn draw(&mut self, app: &mut App) {
        // Draw texts
        let mut text = sfml::graphics::Text::new("", &app.font, 0);
        for t in self.texts.iter_mut() {
            Self::text_from_descriptor(&mut text, t, app.font_height);
            app.window.draw(&text);
        }
        // Draw text buttons
        for button in self.buttons.borrow_mut().iter_mut() {
            app.window.draw(&button.shape);
            Self::text_from_descriptor(&mut text, &mut button.text, app.font_height);
            app.window.draw(&text);
        }
        app.window.display();
    }

    fn on_resize(&mut self, width: f32, height: f32, app: &App) {
        for text in self.texts.iter_mut() {
            let x_percentage = text.pos.x / app.win_size.x;
            let y_percentage = text.pos.y / app.win_size.y;
            text.pos.x = x_percentage * width;
            text.pos.y = y_percentage * height;
        }

        for button in self.buttons.borrow_mut().iter_mut() {
            let mut pos = button.shape.position();
            let mut size = button.shape.size();
            let aspect = size.x / size.y;
            let x_percentage = pos.x / app.win_size.x;
            let y_percentage = pos.y / app.win_size.y;
            let height_percentage = size.y / app.win_size.y;
            pos.x = x_percentage * width;
            pos.y = y_percentage * height;
            size.y = height_percentage * height;
            size.x = aspect * size.y;
            button.shape.set_position(pos);
            button.shape.set_size(size);
            button.text.pos = pos + size / 2.0;
        }
    }

    fn handle_events(&self, app: &mut App, event: &Event) {
        match event {
            Event::MouseButtonPressed { button, x, y } => self.update_buttons(app, Vector2i::new(*x, *y), true),
            Event::MouseMoved { x, y } => self.update_buttons(app, Vector2i::new(*x, *y), false),
            _ => {/* Do nothing */},
        }
    }

    fn update_buttons(&self, app: &mut App, mouse_pos: Vector2i, check_press: bool) {
        for button in self.buttons.clone().borrow_mut().iter_mut() {
            if check_press {
                button.check_mouse_press(mouse_pos, app);
            } else {
                button.check_mouse_hover(mouse_pos);
            }
        }
    }
    
}

impl <'a>MenuState<'a> {

    pub fn new(app: &App) -> Self {
        let mut texts = Vec::new();
        let buttons = Rc::new(RefCell::new(Vec::new()));
        let height_offset: f32 = app.font_height as f32 * (5.0 / 3.0);
        let mut title = TextDescriptor::new("Rando Kanji ・ ランド漢字", Vector2f::new(app.window.size().x as f32 / 2.0, height_offset), Color::WHITE, true);
        title.font_base_size = 10;
        texts.push(title);
    
        let mut button = TextButton::new(
            "Play",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset + 100.0), 
            Color::WHITE,
            Color::WHITE,
            app,
            |app| app.change_state(StateEnum::Play),
        );
        buttons.borrow_mut().push(button.clone());
    
        button = TextButton::new(
            "Exit",
            Vector2f::new(app.window.size().x as f32 / 2.0, height_offset + 200.0),
            Color::WHITE,
            Color::WHITE,
            app,
            |app| app.window.close(),
        );
        buttons.borrow_mut().push(button);
    
        Self {
            texts,
            buttons
        }
    }

    pub fn text_from_descriptor(sf_text: &mut Text, descriptor: &mut TextDescriptor, font_height: u32) {
        sf_text.set_string(&descriptor.string);
        sf_text.set_position(descriptor.pos);
        sf_text.set_fill_color(descriptor.color);
        sf_text.set_character_size(descriptor.font_base_size + font_height);
        descriptor.bounds = sf_text.global_bounds();
        if descriptor.center {
            let width = sf_text.global_bounds().width;
            let height = sf_text.global_bounds().height;
            sf_text.move_(Vector2f::new(-width / 2.0, -height / 2.0));
        }
    }
}

pub struct PlayState<'a> {
    pub texts: Vec<TextDescriptor>,
    pub buttons: Rc<RefCell<Vec<TextButton<'a>>>>
}

impl <'a>PlayState<'a> {
    pub fn new() -> Self {
        let texts = Vec::<>::new();
        let buttons = Rc::new(RefCell::new(Vec::new()));

        Self {
            texts,
            buttons,
        }
    }
}

impl <'a>GameState for PlayState<'a> {
    fn update(&mut self, app: &mut App) {
        todo!()
    }

    fn draw(&mut self, app: &mut App) {
        todo!()
    }

    fn on_resize(&mut self, width: f32, height: f32, app: &App) {
        todo!()
    }

    fn handle_events(&self, app: &mut App, event: &Event) {
        todo!()
    }

    fn update_buttons(&self, app: &mut App, mouse_pos: Vector2i, check_press: bool) {
        todo!()
    }
}