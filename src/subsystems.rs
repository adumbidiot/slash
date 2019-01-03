use std::collections::VecDeque;

use glutin::GlContext;

use super::AppState;

pub struct Window {
	window: Option<glutin::GlWindow>,
	events_loop: Option<glutin::EventsLoop>,
	
	event_queue: VecDeque<Event>,
}

impl Window{
	pub fn new() -> Self{
		Window{
			window: None,
			events_loop: None,
			
			event_queue: VecDeque::new(),
		}
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
		
		self.window = Some(window);
		self.events_loop = Some(events_loop);
	}
	
	pub fn handle_event(&mut self) -> Option<Event> {
		let window = self.window.as_mut()?;
		let queue = &mut self.event_queue;
		
		self.events_loop.as_mut()?.poll_events(|event| {
			match event {
				glutin::Event::WindowEvent{ event, .. } => match event {
					glutin::WindowEvent::CloseRequested =>  queue.push_back(Event::Close),
					glutin::WindowEvent::Resized(logical_size) => {
						let dpi_factor = window.get_hidpi_factor();
						window.resize(logical_size.to_physical(dpi_factor));
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