use gl::types::{GLenum, GLuint, GLint};
use gl::types::GLchar;

use std::ptr;
use std::ffi::CString;
use std::str;

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
	
	pub fn free(self){
		unsafe{
			gl::DeleteProgram(self.id);
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