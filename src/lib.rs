extern crate gl;
extern crate glutin;
extern crate nalgebra;

mod graphics;
mod resources;

use self::graphics::Graphics;

use glutin::GlContext;

use gl::types::{GLfloat, GLsizeiptr, GLboolean};

use std::ffi::CString;


struct Config {
	width: f64,
	height: f64,
	title: String,
}

impl Config{
	pub fn new() -> Self{
		return Config {
			width: 1920.0,
			height: 1080.0,
			title: String::from("Slash"),
		};
	}
}

struct App{
	graphics: Graphics,
	
	width: f64,
	height: f64,
	title: String,
	
	events_loop: glutin::EventsLoop,
}

impl App{
	fn new() -> Self{
		return App{
			graphics: Graphics::new(),
			
			width: 1920.0,
			height: 1080.0,
			title: String::from("Slash"),
			
			events_loop: glutin::EventsLoop::new(),
		};
	}
	
	fn config(&mut self, c: Config){
		self.width = c.width;
		self.height = c.height;
	}
	
	fn init(&mut self){
	
	}
	
	fn main_loop(&mut self){
		let win_size = glutin::dpi::LogicalSize::new(self.width, self.height);
		
		let window = glutin::WindowBuilder::new()
			.with_dimensions(win_size)
			.with_title(self.title.clone())
			.with_resizable(false);
		let context = glutin::ContextBuilder::new()
			.with_vsync(true);
		let gl_window = glutin::GlWindow::new(window, context, &self.events_loop).unwrap();
		
		unsafe { 
			gl_window.make_current().unwrap() 
		};
		
		self.graphics.init();

		// Load the OpenGL function pointers
		// TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
		gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

		let mut sprite_renderer = graphics::SpriteRenderer::new(&mut self.graphics, self.width as f32, self.height as f32);
		sprite_renderer.enable();
		
		let mut x = 0.0;
		let mut y = 0.0;
		let mut dx = 1.0 / 3.0;
		let mut dy = 2.0;
		let width = 100.0;
		let height = 100.0;
		
		let mut running = true;
		while running {
			self.events_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => running = false,
						glutin::WindowEvent::Resized(logical_size) => {
							let dpi_factor = gl_window.get_hidpi_factor();
							gl_window.resize(logical_size.to_physical(dpi_factor));
						},
						_ => ()
					},
					_ => ()
				}
			});
	
			self.graphics.clear();
			
			x += dx;
			y += dy;
			
			if x as f64 >= self.width - width as f64 || x <= 0.0{
				dx = -dx;
			}
			
			if y as f64 >= self.height - height as f64 || y <= 0.0{
				dy = -dy;
			}
			
			sprite_renderer.draw_rect(x, y, width, height);
			
			gl_window.swap_buffers().unwrap();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{App, Config};
	
    #[test]
    fn setup() {
		let mut app = App::new();
		let mut c = Config::new();
		c.width = 480.0;
		c.height = 360.0;
		
		app.config(c);
		app.init();
		app.main_loop();
    }
}
