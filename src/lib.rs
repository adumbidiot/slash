extern crate gl;
extern crate glutin;
extern crate nalgebra;

pub mod graphics;
pub mod subsystems;
//mod resources;

use self::graphics::Graphics;
use self::subsystems::{Window, Event};

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

		while let Some(event) = self.window.handle_event() {
			match event {
				Event::Close => self.running = false,
				_=> {
					
				}
			}
		}
		
		self.graphics.clear();
		
		let state = &mut self.state;
		state.update(&self.app_state);
		state.render(&mut self.graphics, &self.app_state);
		
		return Ok(());
	}
}

pub trait State {
	fn new() -> Self where Self: Sized;
	fn init(&mut self, app: &mut App){}
	fn update(&mut self, state: &AppState){}
	fn render(&mut self, graphics: &mut Graphics, state: &AppState){}
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
		}
	};
	
	struct GameBoard{
		block_width: f32,
		block_height: f32,
		board_state: [u8; 9],
		x: f32,
		y: f32,
		dx: f32,
		dy: f32,
	}
	
	impl State for GameBoard{
		fn new() -> Self{
			GameBoard {
				block_width: 100.0,
				block_height: 100.0,
				board_state: [0; 9],
				
				x: 0.0,
				y: 0.0,
				dx: 1.0 / 3.0,
				dy: 2.0,
			}
		}
		
		fn update(&mut self, state: &AppState){
			self.x += self.dx;
			self.y += self.dy;
			
			if self.x as f64 >= state.width - self.block_width as f64 || self.x <= 0.0{
				self.dx = -self.dx;
			}
				
			if self.y as f64 >= state.height - self.block_height as f64 || self.y <= 0.0{
				self.dy = -self.dy;
			}
		}
		
		fn render(&mut self, graphics: &mut Graphics, state: &AppState){
			graphics.get_error();
			let sprite_renderer = graphics.sprite_renderer.as_mut().expect("No Sprite Renderer");
			sprite_renderer.enable_quad();
			sprite_renderer.draw_rect(0.0, 0.0, state.width as f32, state.height as f32, &Color::from_rgba(48, 48, 48, 255));
			
			for i in 0..9 {
				let color = if i % 2 == 0{
					Color::from_rgba(255, 0, 0, 255)
				}else{
					Color::from_rgba(119, 119, 119, 255)
				};
				
				let x = (i % 3) as f32 * self.block_width;
				let y = (state.height - 100.0) as f32 - (i / 3) as f32 * self.block_height;
				sprite_renderer.draw_rect(x, y, self.block_width, self.block_height, &color);
			}
			
			sprite_renderer.draw_rect(self.x, self.y, self.block_width, self.block_height, &Color::from_rgba(255, 255, 6, 133));
			
			sprite_renderer.enable_circle();
			sprite_renderer.draw_circle(50.0, state.height as f32 - 50.0, 100.0, 100.0);
			
			sprite_renderer.enable_line();
			sprite_renderer.draw_line(50.0, 100.0, 100.0, 50.0, 10.0);
			sprite_renderer.draw_line(100.0, 100.0, 50.0, 50.0, 10.0);
			
			sprite_renderer.draw_line(180.0, 100.0, 40.0, 40.0, 10.0);
		}
	}
	
    #[test]
    fn setup() {
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
