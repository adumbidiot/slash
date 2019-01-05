extern crate gl;
extern crate glutin;
extern crate nalgebra;
extern crate rusttype;

pub mod graphics;
pub mod subsystems;
pub mod primitives;
//mod resources;

use self::graphics::Graphics;
use crate::subsystems::{Window, Event};

pub enum SlashError{
	NoWindow,
}

pub type SlashResult<T> = Result<T, SlashError>;

pub struct AppState {
	width: f64,
	height: f64,
	title: String,
}

impl AppState{
	pub fn new() -> Self{
		return AppState {
			width: 1920.0,
			height: 1080.0,
			title: String::from("Slash"),
		};
	}
}

pub struct App<'a>{
	graphics: Graphics,
	window: Window,
	
	app_state: AppState,
	
	running: bool,
	state: Box<State + 'a>,
}

impl<'a> App<'a>{
	pub fn new() -> Self{
		let app_state = AppState::new();
		return App{
			graphics: Graphics::new(),
			window: Window::new(),
			
			app_state,

			running: false,
			state: Box::new(DefaultState::new()),
		};
	}
	
	pub fn init_app_state(&mut self, app_state: AppState){
		self.app_state = app_state;
	}
	
	pub fn set_state<T: State + 'a>(&mut self, state: T){
		self.state = Box::new(state);
	}
	
	pub fn init(&mut self){
		self.window.init(&self.app_state);
		self.graphics.init(self.app_state.width as f32, self.app_state.height as f32);
		self.running = true;
	}
	
	pub fn main_loop(&mut self) -> SlashResult<()>{
		self.window.update();
		
		let state = &mut self.state;
		
		while let Some(event) = self.window.handle_event() {
			match event {
				Event::Close => self.running = false,
				_=> {
					
				}
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
	fn new() -> Self where Self: Sized;
	fn init(&mut self, _app: &mut App){}
	fn handle_event(&mut self, event: &Event, window: &Window){}
	fn update(&mut self, _state: &AppState){}
	fn render(&mut self, graphics: &mut Graphics, _state: &AppState){}
}

struct DefaultState;

impl State for DefaultState{
	fn new() -> Self{
		DefaultState {
		
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{
		App, 
		AppState, 
		State, 
		graphics::{
			Color, 
			Graphics
		},
		subsystems::{Window, Event},
		primitives::Point,
	};
	
	struct GameBoard{
		block_width: f32,
		block_height: f32,
		board_state: [u8; 9],
		turn: u8,
		
		board_color_1: Color,
		board_color_2: Color,
	}
	
	impl State for GameBoard{
		fn new() -> Self{
			GameBoard {
				block_width: 100.0,
				block_height: 100.0,
				board_state: [0; 9],
				turn: 1,
				
				board_color_1: Color::from_rgba(255, 0, 0, 255),
				board_color_2: Color::from_rgba(119, 119, 119, 255),
			}
		}
		
		fn handle_event(&mut self, event: &Event, window: &Window){
			match event{
				Event::Click{x, y} => {
					let window_height = window.get_height();
					let board_width = 3.0 * self.block_width;
					let bottom = window_height - self.block_height * 3.0;
					let right = board_width;
					
					if *x > 0.0 && *x < right && *y < window_height && *y > bottom {
						let y_index = ((window_height - y) / self.block_height) as usize;
						let index = (x / self.block_width) as usize + (3 * y_index);
						
						println!("Index: {}", index);
						
						if self.board_state[index] == 0{
							self.board_state[index] = self.turn;
							self.turn = if self.turn == 1 {
								2
							}else{
								1
							}
						}
					}
				},
				_=> {
				
				}
			}
		}
		
		fn update(&mut self, state: &AppState){

		}
		
		fn render(&mut self, graphics: &mut Graphics, state: &AppState){
			graphics.get_error();
			let sprite_renderer = graphics.sprite_renderer.as_mut().expect("No Sprite Renderer");
			sprite_renderer.enable_quad();
			sprite_renderer.draw_rect(0.0, 0.0, state.width as f32, state.height as f32, &Color::from_rgba(48, 48, 48, 255));
			
			for i in 0..9 {
				let color = if i % 2 == 0{
					&self.board_color_1
				}else{
					&self.board_color_2
				};
				
				let x = (i % 3) as f32 * self.block_height;
				let y = (state.height as f32 - self.block_height) - (i / 3) as f32 * self.block_height;
				sprite_renderer.draw_rect(x, y, self.block_width, self.block_height, color);
			}
			
			sprite_renderer.enable_line();
			for i in 0..9{
				if self.board_state[i] == 1 {
					let x1 = (i % 3) as f32 * self.block_height;
					let x2 = x1 + self.block_width;
					let y1 = state.height as f32 - (i / 3) as f32 * self.block_height;
					let y2 = y1 - self.block_height;
					sprite_renderer.draw_line(x1, y2, x2, y1, 10.0, &Color::from_rgba(0, 0, 0, 255));
					sprite_renderer.draw_line(x1, y1, x2, y2, 10.0, &Color::from_rgba(0, 0, 0, 255));
				}
			}
			
			sprite_renderer.enable_circle();
			for i in 0..9 {
				if self.board_state[i] == 2 {
					let radius = self.block_width / 2.0;
					let x = radius + ((i % 3)as f32 * self.block_width);
					let y = state.height as f32 - radius - ((i / 3) as f32 * self.block_height);
					sprite_renderer.draw_circle(x, y, self.block_width, self.block_height);
				}
			}
			
			sprite_renderer.enable_text();
			sprite_renderer.draw_text(Point::new(1.0, 1.0), "Welcome to Tic Tac Toe!", 50.0);
			let turn_str = if self.turn == 1{
				"Turn: X"
			}else{
				"Turn: O"
			};
			
			let size = 20.0;
			sprite_renderer.draw_text(Point::new(3.0 * self.block_width, state.height as f32 - size), turn_str, size);
		}
	}
	
    #[test]
    fn tic_tac_toe() {
		let mut app = App::new();
		let mut app_state = AppState::new();
		
		app_state.width = 480.0;
		app_state.height = 360.0;
		app_state.title = String::from("Tic Tac Toe Test");
		
		app.init_app_state(app_state);
		app.set_state(GameBoard::new());
		app.init();
		
		while app.running && app.main_loop().is_ok(){
			
		}
    }
}
