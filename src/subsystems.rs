use std::collections::VecDeque;

use glutin::GlContext;

use super::AppState;

pub struct Window {
	window: Option<glutin::GlWindow>,
	events_loop: Option<glutin::EventsLoop>,
	
	mouse_x: f32,
	mouse_y: f32,
	width: f32,
	height: f32,
	
	event_queue: VecDeque<Event>,
}

impl Window{
	pub fn new() -> Self{
		Window{
			window: None,
			events_loop: None,
			
			mouse_x: 0.0,
			mouse_y: 0.0,
			width: 0.0,
			height: 0.0,
			
			event_queue: VecDeque::new(),
		}
	}
	
	pub fn get_width(&self) -> f32{
		return self.width;
	}
	
	pub fn get_height(&self) -> f32 {
		return self.height;
	}
	
	pub fn init(&mut self, state: &AppState){
		let win_size = glutin::dpi::LogicalSize::new(state.width, state.height);
		let window_builder = glutin::WindowBuilder::new()
			.with_dimensions(win_size)
			.with_title(state.title.clone())
			.with_resizable(false);
		let context = glutin::ContextBuilder::new()
			.with_vsync(true);
		
		let events_loop = glutin::EventsLoop::new();
		let window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();
		
		unsafe { 
			window.make_current().unwrap() 
		};
		
		// Load the OpenGL function pointers
		// TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
		gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
		
		self.width = state.width as f32;
		self.height = state.height as f32;
		self.window = Some(window);
		self.events_loop = Some(events_loop);
	}
	
	pub fn handle_event(&mut self) -> Option<Event> {
		let Window {window, event_queue, mouse_x, mouse_y, height, ..} = self;
		let window = window.as_mut()?;

		
		self.events_loop.as_mut()?.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent{event, .. } => match event {
					glutin::WindowEvent::CloseRequested =>  event_queue.push_back(Event::Close),
					glutin::WindowEvent::Resized(logical_size) => {
						let dpi_factor = window.get_hidpi_factor();
						window.resize(logical_size.to_physical(dpi_factor));
					},
					glutin::WindowEvent::CursorMoved{position, ..} => {
						let (x, y): (f64, f64) = position.into();
						*mouse_x = x as f32;
						*mouse_y = *height - y as f32;
					},
					glutin::WindowEvent::MouseInput{state, ..} => {
						match state {
							glutin::ElementState::Released => {
								event_queue.push_back(Event::Click{x: *mouse_x, y: *mouse_y})
							},
							_=> ()
						}
					},
					_ => ()
				},
				_ => ()
			}
		});
			
		return self.event_queue.pop_front();
	}
	
	pub fn update(&mut self){
		self.window.as_mut().expect("No Window").swap_buffers().unwrap();
	}
}

pub enum Event{
	Close,
	Click {
		x: f32,
		y: f32,
	}
}