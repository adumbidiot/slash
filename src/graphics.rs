mod backends;

//pub use backends::*;

use gl::types::{GLenum, GLuint, GLint, GLboolean, GLsizeiptr, GLfloat, GLchar};

use std::{ptr, mem, ffi::CString, str};

pub struct Graphics {

}

impl Graphics{
	pub fn new() -> Self{
		return Graphics{
		
		};
	}
	
	pub fn init(&mut self){
		
	}
	
	pub fn clear(&mut self){
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}
	
	
	////INTERNAL FUNCTIONS/////
	
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
}

pub struct ShaderProgram {
	pub id: GLuint
}

impl ShaderProgram {
	pub fn compile(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<Self, String> {
		let mut shader = ShaderProgram{
			id: 0,
		};
		
		unsafe {
			shader.id = gl::CreateProgram();
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
}

impl Drop for ShaderProgram{
	fn drop(&mut self){
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}

pub struct SpriteRenderer {
	program: ShaderProgram,
	vao: GLuint,
	vbo: GLuint, 
	ortho: nalgebra::base::Matrix4<f32>,
}

impl SpriteRenderer {
	pub fn new(graphics: &mut Graphics, width: f32, height: f32) -> Self{
		let mut vao = 0;
		let mut vbo = 0;
		let vertex_data: [GLfloat; 12] = [
			0.0, 0.0, 
			0.0, 1.0, 
			1.0, 0.0, 
			
			1.0, 0.0,
			1.0, 1.0,
			0.0, 1.0,
		];
		
		unsafe{
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
		let vs = graphics.compile_shader(vs_src, gl::VERTEX_SHADER);
		let fs = graphics.compile_shader(fs_src, gl::FRAGMENT_SHADER);
		let ortho = nalgebra::Orthographic3::new(0.0, width, 0.0, height, -1.0, 1.0).unwrap();
		
		let mut shader_program = ShaderProgram::compile(vs, fs).unwrap();
		shader_program.set_uniform_matrix4("Projection", ortho.as_slice());
		
		unsafe {
			gl::DeleteShader(fs);
			gl::DeleteShader(vs);
		}
		
		return SpriteRenderer {
			program: shader_program,
			vao,
			vbo,
			ortho,
		};
	}

	pub fn enable(&mut self){
		self.program.enable();
	}
	
	pub fn draw_rect(&self, x: f32, y: f32, w: f32, h: f32){
		let translation_mat = self.ortho * nalgebra::base::Matrix4::new_translation(&nalgebra::base::Vector3::new(x, y, 0.0));
		let scale_mat = translation_mat * nalgebra::base::Matrix4::new_nonuniform_scaling(&nalgebra::base::Vector3::new(w, h, 0.0));
		self.program.set_uniform_matrix4("Projection", scale_mat.as_slice());

		unsafe{
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
		}
	}
}

impl Drop for SpriteRenderer{
	fn drop(&mut self){
		unsafe {
			gl::DeleteBuffers(1, &self.vbo);
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}
}
