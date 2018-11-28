extern crate gl;
extern crate glutin;
extern crate nalgebra;

mod graphics;

use self::graphics::Graphics;

use glutin::GlContext;

use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLboolean;

use std::mem;
use std::ptr;
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
		
let vs_src = "
#version 150
in vec2 position;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * vec4(position.x, position.y, 0.0, 1.0);
}";

let fs_src = "
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(0.5, 1.0, 1.0, 1.0);
}";



		let vertex_data: [GLfloat; 12] = [
			-100.5, 100.5, 
			100.5, 100.5, 
			-100.5, -100.5, 
			
			-100.5, -100.5,
			100.5, -100.5,
			100.5, 100.5,
		];
		
		let mut vao = 0;
		let mut vbo = 0;
		
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			
			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			
			let vertex_size: GLsizeiptr = (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
			gl::BufferData(
				gl::ARRAY_BUFFER, 
				vertex_size, 
				mem::transmute(&vertex_data[0]), 
				gl::STATIC_DRAW
			);
			
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				0,
				ptr::null(),
			);
		}
		
		let vs = self.graphics.compile_shader(vs_src, gl::VERTEX_SHADER);
		let fs = self.graphics.compile_shader(fs_src, gl::FRAGMENT_SHADER);
		let projection_mat = nalgebra::Orthographic3::new(0.0, 300.0, 0.0, 300.0, -1.0, 1.0).unwrap();
		
		let mut shader_program = graphics::ShaderProgram::compile(vs, fs).unwrap();
		shader_program.enable();
		shader_program.set_uniform_matrix4("Projection", projection_mat.as_slice());
		

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
			unsafe {
				gl::DrawArrays(gl::TRIANGLES, 0, 6);
			}
			
			gl_window.swap_buffers().unwrap();
		}
		
		shader_program.free();
		unsafe {
			gl::DeleteShader(fs);
			gl::DeleteShader(vs);
			gl::DeleteBuffers(1, &vbo);
			gl::DeleteVertexArrays(1, &vao);
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
