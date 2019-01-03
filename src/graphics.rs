mod backends;

//pub use backends::*;

use gl::types::{GLenum, GLuint, GLint, GLboolean, GLsizeiptr, GLfloat, GLchar};

use std::{ptr, mem, ffi::CString, str};
use super::AppState;

pub struct Graphics {
	pub sprite_renderer: Option<SpriteRenderer>,
}

impl Graphics{
	pub fn new() -> Self{
		return Graphics{
			sprite_renderer: None,
		};
	}
	
	pub fn init(&mut self, width: f32, height: f32){
		unsafe {
			gl::Enable(gl::BLEND); 
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
			
			gl::Enable(gl::MULTISAMPLE); 
		}
		
		self.sprite_renderer = Some(SpriteRenderer::new(self, width, height));
	}
	
	pub fn clear(&mut self){
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}
	
	//TODO: Result
	pub fn compile_shader(&self, src: &str, ty: GLenum) -> GLuint{
		let shader;
		unsafe {
			shader = gl::CreateShader(ty);
			let c_str = CString::new(src.as_bytes()).unwrap();
			gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(shader);
		}
		
		return shader;
	}
	
	pub fn get_error(&mut self){
		unsafe{
			let err = gl::GetError();
			if err != gl::NO_ERROR{
				match err {
					gl::INVALID_OPERATION => {
						println!("GL Error: Invalid Operation");
					},
					_=> {
						println!("GL Error: {}", err);
					}
				}
				
			}
		}
	}
}

pub struct VertexArrayObject{
	id: GLuint,
}

impl VertexArrayObject{
	pub fn new() -> Self {
		let mut id = 0;
		
		unsafe{ 
			gl::GenVertexArrays(1, &mut id);
		}
		
		VertexArrayObject {
			id
		}
	}
	
	pub fn enable(&mut self){
		unsafe {
			gl::BindVertexArray(self.id);
		}
	}
}

impl Drop for VertexArrayObject {
	fn drop(&mut self){
		unsafe{
			gl::DeleteVertexArrays(1, &self.id);
		}
	}
}

pub enum BufferType{
	Static,
	Dynamic,
}
	
pub struct VertexBufferObject{
	id: GLuint,
}

impl VertexBufferObject{
	pub fn new() -> Self{
		let mut id = 0;
		
		unsafe {
			gl::GenBuffers(1, &mut id);
		}
		
		VertexBufferObject{
			id
		}
	}
	
	pub fn enable(&mut self){
		unsafe{
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}
	}
	
	pub fn set(&mut self, data: &[GLfloat], buffer_type: BufferType){
		let data_size: GLsizeiptr = (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
		let buffer_type = match buffer_type {
			BufferType::Static => gl::STATIC_DRAW,
			BufferType::Dynamic => gl::DYNAMIC_DRAW,
			_=> gl::STATIC_DRAW,
		};
		
		unsafe{
			gl::BufferData(
				gl::ARRAY_BUFFER, 
				data_size, 
				mem::transmute(&data[0]), 
				buffer_type
			);
		}
	}
}

impl Drop for VertexBufferObject{
	fn drop(&mut self){
		unsafe{
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

pub struct ElementBufferObject{
	id: GLuint,
}

impl ElementBufferObject{
	pub fn new() -> Self {
		let mut id = 0;
		
		unsafe {
			gl::GenBuffers(1, &mut id);
		}
		
		ElementBufferObject{
			id,
		}
	}
	
	pub fn enable(&mut self){
		unsafe{
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
		}
	}
	
	pub fn set(&mut self, data: &[GLuint]){
		let data_size: GLsizeiptr = (data.len() * mem::size_of::<GLuint>()) as GLsizeiptr;
		unsafe{
			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER, 
				data_size, 
				mem::transmute(&data[0]), 
				gl::STATIC_DRAW
			);
		}
	}
}

impl Drop for ElementBufferObject{
	fn drop(&mut self){
		unsafe{
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

pub struct ShaderProgram {
	pub id: GLuint
}

impl ShaderProgram {
	pub fn new() -> Self {		
		ShaderProgram {
			id: unsafe {
				gl::CreateProgram()
			}
		}
	}
	
	pub fn compile(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<Self, String> {
		let mut shader = ShaderProgram::new();
		
		unsafe {
			gl::AttachShader(shader.id, vertex_shader);
			gl::AttachShader(shader.id, fragment_shader);
			gl::LinkProgram(shader.id);
		}
		
		let mut status = gl::FALSE as GLint;
		
		unsafe {
			gl::GetProgramiv(shader.id, gl::LINK_STATUS, &mut status);
		}
		
		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			unsafe{
				gl::GetProgramiv(shader.id, gl::INFO_LOG_LENGTH, &mut len);
			}
			
			let mut buf = Vec::with_capacity(len as usize);
			
			unsafe{
				buf.set_len((len as usize) - 1);
				gl::GetProgramInfoLog(shader.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
			}
			
			return Err(String::from_utf8(buf).expect("Invalid UTF8"));
		}
		
		return Ok(shader);
	}
	
	pub fn enable(&self){
		unsafe {
			gl::UseProgram(self.id);
		}
	}
	
	pub fn set_uniform_matrix4(&self, var: &str, mat_slice: &[f32]){
		let var_str = CString::new(var).unwrap();
		
		unsafe {
			let gl_projection = gl::GetUniformLocation(self.id, var_str.as_ptr());
			gl::UniformMatrix4fv(gl_projection, 1, gl::FALSE, mat_slice.as_ptr());
		}
	}
	
	pub fn set_uniform_vec4(&self, var: &str, data: &[f32]){
		let var_str = CString::new(var).unwrap();
		
		unsafe {
			let loc = gl::GetUniformLocation(self.id, var_str.as_ptr());
			gl::Uniform4fv(loc, 1, data.as_ptr());
		}
	}
	
	pub fn set_float(&self, var:&str, data: f32){
		let var_str = CString::new(var).unwrap();
		
		unsafe {
			let loc = gl::GetUniformLocation(self.id, var_str.as_ptr());
			gl::Uniform1f(loc, data);
		}
	}
}

impl Drop for ShaderProgram{
	fn drop(&mut self){
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}

pub struct SpriteRenderer {
	quad_program: ShaderProgram,
	quad_vao: VertexArrayObject,
	quad_vbo: VertexBufferObject,
	
	circle_program: ShaderProgram,
	
	line_program: ShaderProgram,
	line_vbo: VertexBufferObject,
	
	ortho: nalgebra::base::Matrix4<f32>,
}

impl SpriteRenderer {
	pub fn new(graphics: &mut Graphics, width: f32, height: f32) -> Self{
		let quad_vertex_data: [GLfloat; 16] = [
			0.0, 0.0, 	-1.0, -1.0,
			1.0, 0.0, 	1.0, -1.0,
			1.0, 1.0, 	1.0, 1.0,
			0.0, 1.0,	-1.0, 1.0
		];
		
		let mut circle_vertex_data = Vec::new();
		let circle_vert_count = 200;
		let radius = 1.0;
		for i in 0..circle_vert_count {
			let heading = 2.0 * std::f32::consts::PI * i as f32 / circle_vert_count as f32;
			circle_vertex_data.push(heading.cos() * radius);
			circle_vertex_data.push(heading.sin() * radius);
		}
		
		let mut quad_vao = VertexArrayObject::new();
		quad_vao.enable();
		
		let mut quad_vbo = VertexBufferObject::new();
		quad_vbo.enable();
		quad_vbo.set(&quad_vertex_data, BufferType::Static);
		
		//TODO: VAO or VBO?
		unsafe{			
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				std::mem::size_of::<GLfloat>() as i32 * 4,
				ptr::null(),
			);
			
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				std::mem::size_of::<GLfloat>() as i32 * 4,
				(std::mem::size_of::<GLfloat>() * 2) as *const _,
			);
			
		}
		
		let quad_vs_src = include_str!("quad.vs.glsl");
		let quad_fs_src = include_str!("quad.fs.glsl");
		let quad_vs = graphics.compile_shader(quad_vs_src, gl::VERTEX_SHADER);
		let quad_fs = graphics.compile_shader(quad_fs_src, gl::FRAGMENT_SHADER);
		let ortho = nalgebra::Orthographic3::new(0.0, width, 0.0, height, -1.0, 1.0).unwrap();
		
		let mut quad_program = ShaderProgram::compile(quad_vs, quad_fs).expect("Could not compile shader");
		quad_program.enable();
		quad_program.set_uniform_matrix4("Projection", ortho.as_slice());
		quad_program.set_uniform_vec4("in_color", &vec![0.0, 1.0, 1.0, 1.0]);
				
		let circle_vs_src = include_str!("circle.vs.glsl");
		let circle_fs_src = include_str!("circle.fs.glsl");
		let circle_vs = graphics.compile_shader(circle_vs_src, gl::VERTEX_SHADER);
		let circle_fs = graphics.compile_shader(circle_fs_src, gl::FRAGMENT_SHADER);
		
		let circle_program = ShaderProgram::compile(circle_vs, circle_fs).expect("Could not compile shader");
		circle_program.enable();
		circle_program.set_uniform_matrix4("Projection", ortho.as_slice());
		circle_program.set_float("border_width", 15.0);
		
		
		let mut line_vbo = VertexBufferObject::new();
		line_vbo.enable();
		
		unsafe{			
			gl::EnableVertexAttribArray(3);
			gl::VertexAttribPointer(
				3,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				0,
				ptr::null(),
			);
		}
		
		let line_vs_src = include_str!("line.vs.glsl");
		let line_fs_src = include_str!("line.fs.glsl");
		let line_vs = graphics.compile_shader(line_vs_src, gl::VERTEX_SHADER);
		let line_fs = graphics.compile_shader(line_fs_src, gl::FRAGMENT_SHADER);
		
		let mut line_program = ShaderProgram::compile(line_vs, line_fs).expect("Could not compile shader");
		line_program.enable();
		line_program.set_uniform_matrix4("Projection", ortho.as_slice());
	
		unsafe {
			gl::DeleteShader(quad_fs);
			gl::DeleteShader(quad_vs);
			gl::DeleteShader(circle_fs);
			gl::DeleteShader(circle_vs);
			gl::DeleteShader(line_fs);
			gl::DeleteShader(line_vs);
		}
		
		return SpriteRenderer {
			quad_program,
			quad_vao,
			quad_vbo,
			
			circle_program,
			
			line_vbo,
			line_program,
			
			ortho,
		};
	}

	pub fn enable_quad(&mut self){
		self.quad_vao.enable();
		self.quad_vbo.enable();
		self.quad_program.enable();
	}
	
	pub fn enable_circle(&mut self){
		self.quad_vao.enable();
		self.quad_vbo.enable();
		self.circle_program.enable();
	}
	
	pub fn enable_line(&mut self){
		self.quad_vao.enable();
		self.line_vbo.enable();
		self.line_program.enable();
	}
	
	pub fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32, color: &Color){
		let translation_mat = self.ortho * nalgebra::base::Matrix4::new_translation(&nalgebra::base::Vector3::new(x, y, 0.0));
		let scale_mat = translation_mat * nalgebra::base::Matrix4::new_nonuniform_scaling(&nalgebra::base::Vector3::new(w, h, 0.0));
		self.quad_program.set_uniform_matrix4("Projection", scale_mat.as_slice());
		let colors: [f32; 4] = [
			color.r as f32 / 255.0, 
			color.g as f32 / 255.0, 
			color.b as f32 / 255.0, 
			color.a as f32 / 255.0
		];
		
		self.quad_program.set_uniform_vec4("in_color", &colors);
		
		unsafe{
			gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
		}
	}
	
	pub fn draw_circle(&self, x: f32, y: f32, w: f32, h: f32){
		let translation_mat = self.ortho * nalgebra::base::Matrix4::new_translation(&nalgebra::base::Vector3::new(x - w / 2.0, y - h / 2.0, 0.0));
		let scale_mat = translation_mat * nalgebra::base::Matrix4::new_nonuniform_scaling(&nalgebra::base::Vector3::new(w, h, 0.0));
		self.circle_program.set_uniform_matrix4("Projection", scale_mat.as_slice());
		
		unsafe{
			gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
		}
	}
	
	pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32){
		let dx = x2 - x1;
		let dy = y2 - y1;
		
		let magnitude = (f32::powi(dx, 2) + f32::powi(dy, 2)).sqrt();
		let modifier = thickness / magnitude;
		
		let half_dx = (modifier * dx) / 2.0;
		let half_dy = (modifier * dy) / 2.0;
		
		let x3 = x1 + half_dx;
		let y3 = y1 - half_dy;
		
		let x4 = x2 + half_dx;
		let y4 = y2 - half_dy;
		
		let x5 = x1 - half_dx;
		let y5 = y1 + half_dy;
		
		let x6 = x2 - half_dx;
		let y6 = y2 + half_dy;
		
		let data: [GLfloat; 8] = [
			x4, y4,
			x3, y3,
			x5, y5,
			x6, y6,
		];
		
		self.line_vbo.set(&data, BufferType::Dynamic);
		
		unsafe {
			gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
		}		
	}
}

pub struct Color{
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color{
	pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self{
		Color {
			r,
			g,
			b,
			a,
		}
	}
}