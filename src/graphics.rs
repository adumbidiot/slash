mod backends;

use crate::primitives::{	
	Rect,
	Point as SlashPoint,
};

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
					gl::INVALID_VALUE => {
						println!("GL Error: Invalid Value");
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
	
	pub fn enable_attribute(&mut self, id: u32){
		unsafe{
			gl::EnableVertexAttribArray(id);
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

pub struct Texture {
	id: GLuint,
}

impl Texture {
	pub fn new() -> Self{
		let mut id = 0;
		
		unsafe{ 
			gl::GenTextures(1, &mut id);
		}
		
		Texture {
			id
		}
	}
	
	pub fn enable(&mut self){
		unsafe{
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}
	
	pub fn set(&mut self, width: i32, height: i32, data: &[u8]){
		unsafe{
			gl::TexImage2D(
				gl::TEXTURE_2D, 
				0, 
				gl::RED as i32, 
				width, 
				height, 
				0, 
				gl::RGB, 
				gl::UNSIGNED_BYTE, 
				std::mem::transmute(&data[0])
			);
		}
	}

	pub fn update(&mut self, rect: &Rect, data: &[u8]){
		unsafe {
			gl::TexSubImage2D(
				gl::TEXTURE_2D,
				0,
				rect.x as i32,
				rect.y as _,
				rect.width as _,
				rect.height as _,
				gl::RED,
				gl::UNSIGNED_BYTE,
				std::mem::transmute(&data[0])
			);
		}
	}
}

impl Drop for Texture {
	fn drop(&mut self){
		unsafe {
			gl::DeleteTextures(1, &self.id);
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
	
	text_vbo: VertexBufferObject,
	text_texture: Texture,
	text_program: ShaderProgram,
	
	font: rusttype::Font<'static>,
	font_cache: rusttype::gpu_cache::Cache<'static>,
	ortho: nalgebra::base::Matrix4<f32>,
	
	test: u32,
}

impl SpriteRenderer {
	pub fn new(graphics: &mut Graphics, width: f32, height: f32) -> Self{
		let quad_vertex_data: [GLfloat; 16] = [
			0.0, 0.0, 	-1.0, -1.0,
			1.0, 0.0, 	1.0, -1.0,
			1.0, 1.0, 	1.0, 1.0,
			0.0, 1.0,	-1.0, 1.0
		];
		
		let mut quad_vao = VertexArrayObject::new();
		quad_vao.enable();
		quad_vao.enable_attribute(0);
		quad_vao.enable_attribute(1);
		quad_vao.enable_attribute(2);
		quad_vao.enable_attribute(3);
		quad_vao.enable_attribute(4);
		
		let mut quad_vbo = VertexBufferObject::new();
		quad_vbo.enable();
		quad_vbo.set(&quad_vertex_data, BufferType::Static);
		
		//TODO: VAO or VBO?
		unsafe{			
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				std::mem::size_of::<GLfloat>() as i32 * 4,
				ptr::null(),
			);
			
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				std::mem::size_of::<GLfloat>() as i32 * 4,
				(std::mem::size_of::<GLfloat>() * 2) as *const _,
			);
		}
		
		let mut line_vbo = VertexBufferObject::new();
		line_vbo.enable();
		
		unsafe{
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				0,
				ptr::null(),
			);
		}
		
		let mut text_vbo = VertexBufferObject::new();
		text_vbo.enable();
		
		unsafe{
			gl::VertexAttribPointer(
				3,
				2,
				gl::FLOAT,
				gl::FALSE as GLboolean,
				std::mem::size_of::<GLfloat>() as i32 * 4,
				ptr::null(),
			);
		}
		
		unsafe{
			gl::VertexAttribPointer(
				4,
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
		let ortho = nalgebra::Orthographic3::new(0.0, width, 0.0, height, -1.0, 1.0).into_inner();
		
		let quad_program = ShaderProgram::compile(quad_vs, quad_fs).expect("Could not compile shader");
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
		
		let line_vs_src = include_str!("line.vs.glsl");
		let line_fs_src = include_str!("line.fs.glsl");
		let line_vs = graphics.compile_shader(line_vs_src, gl::VERTEX_SHADER);
		let line_fs = graphics.compile_shader(line_fs_src, gl::FRAGMENT_SHADER);
		
		let mut line_program = ShaderProgram::compile(line_vs, line_fs).expect("Could not compile shader");
		line_program.enable();
		line_program.set_uniform_matrix4("Projection", ortho.as_slice());
		
		let text_vs_src = include_str!("text.vs.glsl");
		let text_fs_src = include_str!("text.fs.glsl");
		let text_vs = graphics.compile_shader(text_vs_src, gl::VERTEX_SHADER);
		let text_fs = graphics.compile_shader(text_fs_src, gl::FRAGMENT_SHADER);
		
		let mut text_program = ShaderProgram::compile(text_vs, text_fs).expect("Could not compile shader");
		text_program.enable();
		text_program.set_uniform_matrix4("Projection", ortho.as_slice());
	
		unsafe {
			gl::DeleteShader(quad_fs);
			gl::DeleteShader(quad_vs);
			gl::DeleteShader(circle_fs);
			gl::DeleteShader(circle_vs);
			gl::DeleteShader(line_fs);
			gl::DeleteShader(line_vs);
		}
		
		let font_data = include_bytes!("./GoudyStMTT.ttf");
		let font = rusttype::Font::from_bytes(font_data as &[u8]).expect("Error loading font");
		let font_cache = rusttype::gpu_cache::Cache::builder()
			.dimensions(256, 256)
			.build();
		
		unsafe {
			gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
		}
   
		let mut text_texture = Texture::new();
		text_texture.enable();
		text_texture.set(256, 256, &vec![128u8; (256 * 256) as usize]);
		
		unsafe{
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
		}
		
		return SpriteRenderer {
			quad_program,
			quad_vao,
			quad_vbo,
			
			circle_program,
			
			line_vbo,
			line_program,
			
			text_vbo,
			text_texture,
			text_program,
			
			font,
			font_cache,
			ortho,
			test: 0,
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
	
	pub fn enable_text(&mut self){
		self.quad_vao.enable();
		self.text_vbo.enable();
		self.text_texture.enable();
		self.text_program.enable();
	}
	
	pub fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32, color: &Color){
		let translation_mat = self.ortho * nalgebra::base::Matrix4::new_translation(&nalgebra::base::Vector3::new(x, y, 0.0));
		let scale_mat = translation_mat * nalgebra::base::Matrix4::new_nonuniform_scaling(&nalgebra::base::Vector3::new(w, h, 0.0));
		self.quad_program.set_uniform_matrix4("Projection", scale_mat.as_slice());
		
		self.quad_program.set_uniform_vec4("in_color", &color.as_float_array());
		
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
	
	pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: &Color){
		let dx = x2 - x1;
		let dy = y2 - y1;
		
		let magnitude = (f32::powi(dx, 2) + f32::powi(dy, 2)).sqrt();
		let modifier = thickness / magnitude;
		
		let half_dx = (modifier * dx) / 2.0;
		let half_dy = (modifier * dy) / 2.0;
		
		let x3 = x1 + half_dy;
		let y3 = y1 - half_dx;
		
		let x4 = x2 + half_dy;
		let y4 = y2 - half_dx;
		
		let x5 = x1 - half_dy;
		let y5 = y1 + half_dx;
		
		let x6 = x2 - half_dy;
		let y6 = y2 + half_dx;
		
		let data: [GLfloat; 8] = [
			x4, y4,
			x3, y3,
			x5, y5,
			x6, y6,
		];
		
		self.line_vbo.set(&data, BufferType::Dynamic);
		self.line_program.set_uniform_vec4("in_color", &color.as_float_array());
		
		unsafe {
			gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
		}		
	}
	
	//I don't even know go away
	pub fn draw_text(&mut self, point: SlashPoint, data: &str, size: f32){
		let screen_width = 480.0;
		let screen_height = 360.0;
		
		let x = point.x;
		let y = screen_height - point.y;
		
		let font = &self.font;
		let font_cache = &mut self.font_cache;
		let text_texture = &mut self.text_texture;
		
		let glyphs: Vec<_> = font
			.layout(data, rusttype::Scale::uniform(size), rusttype::Point{x, y})
			.map(|glyph| glyph.standalone())
			.map(|glyph|{ 
				font_cache.queue_glyph(0, glyph.clone()); 
				glyph
			}).collect();
		
		font_cache.cache_queued(|rect, data| {
			text_texture.update(&Rect::new(rect.min.x as f32, rect.min.y as f32, rect.width() as f32, rect.height() as f32), &data);
		}).expect("Error updating GPU Texture Cache");
		
		let mut verts = Vec::new();
		glyphs.iter().for_each(|g|{
			if let Ok(Some((uv_rect, screen_rect))) = font_cache.rect_for(0, g) {
				let origin = rusttype::point(0.0, 0.0);
				
				let gl_rect = rusttype::Rect {
					min: origin
						+ (rusttype::vector(
							screen_rect.min.x as f32 / screen_width - 0.5,
							1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
						)) * 2.0,
                    max: origin
						+ (rusttype::vector(
							screen_rect.max.x as f32 / screen_width - 0.5,
							1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
                        )) * 2.0,
                };
				
				let local_verts: [f32; 24] = [	
					gl_rect.min.x, gl_rect.max.y, uv_rect.min.x, uv_rect.max.y,
					gl_rect.min.x, gl_rect.min.y, uv_rect.min.x, uv_rect.min.y,
					gl_rect.max.x, gl_rect.min.y, uv_rect.max.x, uv_rect.min.y,
					gl_rect.max.x, gl_rect.min.y, uv_rect.max.x, uv_rect.min.y,
					gl_rect.max.x, gl_rect.max.y, uv_rect.max.x, uv_rect.max.y,
					gl_rect.min.x, gl_rect.max.y, uv_rect.min.x, uv_rect.max.y,
				];
				verts.extend_from_slice(&local_verts);
			}
		});
		
		self.text_vbo.set(&verts, BufferType::Dynamic);
		
		unsafe {
			gl::DrawArrays(gl::TRIANGLES, 0, verts.len() as i32 / 4);
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
	
	pub fn as_float_array(&self) -> [f32; 4]{
		return [
			self.r as f32 / 255.0, 
			self.g as f32 / 255.0, 
			self.b as f32 / 255.0, 
			self.a as f32 / 255.0,
		];
	}	
}