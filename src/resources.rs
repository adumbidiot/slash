use std::collections::HashMap;

pub enum Resource{
	Image{
		width: u32,
		height: u32,
		data: Vec<u8>,
	},
}

pub struct ResourceHandle{
	name: String,
	path: String,
	data: Option<Box<Resource>>,
}

impl ResourceHandle{

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
	
	pub fn load_resource(&mut self, name: &str){
		
	}
	
	pub fn get_resource(&mut self, name: &str){
	
	}
}