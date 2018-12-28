use std::collections::HashMap;

pub enum Resource{
	Image,
}

pub struct ResourceHandle{
	name: String,
	path: String,
	data: Option<Box<Resource>>,
}

pub struct ResourceManager{
	map: HashMap<String, ResourceHandle>,
}

impl ResourceManager{
	pub fn new() -> Self{
		ResourceManager{
			map: HashMap::new(),
		}
	}
	
	pub fn add_resource(&mut self, name: String, path: String){
		
	}
	
	pub fn get_resource(&mut self, name: &str){
	
	}
}