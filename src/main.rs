extern crate cairo;

use std::{fs};

pub mod parser;
pub mod css;
pub mod cssom;
pub mod dom;
pub mod html;
pub mod render;

fn main() {
	let html_string = fs::read_to_string("./examples/test.html").expect("Failed to read file");
	let node_tree = html::parse(html_string);
	
	let mut renderer = render::Renderer::new(600, 600);
	
	renderer.to_image(node_tree)
}
