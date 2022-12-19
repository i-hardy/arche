use std::collections::{HashMap};

#[derive(Debug)]
pub struct Node {
	pub children: Vec<Node>,
	pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
	Element(ElementData),
	Text(String)
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
	pub tag_name: String,
	pub attributes: AttrMap,
}

pub fn text(content: String) -> Node {
	Node { children: Vec::new(), node_type: NodeType::Text(content) }
}

pub fn element(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
	Node {
		children,
		node_type: NodeType::Element(ElementData {
			tag_name,
			attributes,	
		})
	}
}