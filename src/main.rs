use std::collections::HashMap;

pub mod dom;

fn main() {
	let mut children = Vec::new();
	children.push(dom::text("hello".to_string()));
	let node_tree = dom::element("body".to_string(), HashMap::new(), children);
	println!("{:?}", node_tree)
}
