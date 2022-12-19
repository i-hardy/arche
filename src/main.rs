use std::{fs};

pub mod dom;
pub mod html;

fn main() {
	let html_string = fs::read_to_string("./examples/test.html").expect("Success!");
	let node_tree = html::parse(html_string);
	println!("{:?}", node_tree)
}
