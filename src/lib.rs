extern crate gl;
extern crate glutin;
extern crate nalgebra;
extern crate rusttype;

pub mod graphics;
pub mod primitives;
pub mod subsystems;
//mod resources;

use self::graphics::Graphics;
use crate::subsystems::{
    Event,
    Window,
};

pub enum SlashError {
    NoWindow,
}

pub type SlashResult<T> = Result<T, SlashError>;

pub struct AppState {
    pub width: f64,
    pub height: f64,
    pub title: String,
}

impl AppState {
    pub fn new() -> Self {
        return AppState {
            width: 1920.0,
            height: 1080.0,
            title: String::from("Slash"),
        };
    }
}

pub struct App<'a> {
    graphics: Graphics,
    window: Window,

    pub app_state: AppState,

    pub running: bool,
    pub state: Box<State + 'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let app_state = AppState::new();
        return App {
            graphics: Graphics::new(),
            window: Window::new(),

            app_state,

            running: false,
            state: Box::new(DefaultState::new()),
        };
    }

    pub fn init_app_state(&mut self, app_state: AppState) {
        self.app_state = app_state;
    }

    pub fn set_state<T: State + 'a>(&mut self, state: T) {
        self.state = Box::new(state);
    }

    pub fn init(&mut self) {
        self.window.init(&self.app_state);
        self.graphics
            .init(self.app_state.width as f32, self.app_state.height as f32);
        self.state.init(&mut self.window, &mut self.graphics);
        self.running = true;
    }

    pub fn main_loop(&mut self) -> SlashResult<()> {
        self.window.update();

        let state = &mut self.state;

        while let Some(event) = self.window.handle_event() {
            match event {
                Event::Close => self.running = false,
                _ => {}
            }
            state.handle_event(&event, &self.window);
        }

        self.graphics.clear();

        state.update(&self.app_state);
        state.render(&mut self.graphics, &self.app_state);

        return Ok(());
    }
}

pub trait State {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&mut self, window: &mut Window, graphics: &mut Graphics) {}
    fn handle_event(&mut self, event: &Event, window: &Window) {}
    fn update(&mut self, _state: &AppState) {}
    fn render(&mut self, graphics: &mut Graphics, _state: &AppState) {}
}

struct DefaultState;

impl State for DefaultState {
    fn new() -> Self {
        DefaultState {}
    }
}
