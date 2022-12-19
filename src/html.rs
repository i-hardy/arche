use std::{collections::HashMap};

use crate::dom;

pub fn parse(source: String) -> dom::Node {
	let mut nodes = Parser { position: 0, input: source }.parse_nodes();
	
	if nodes.len()== 1 {
		nodes.swap_remove(0)
	} else {
		dom::element("html".to_string(), HashMap::new(), nodes)
	}
}

struct Parser {
	position: usize,
	input: String,
}

impl Parser {
	fn parse_node(&mut self) -> dom::Node {
		match self.next_char() {
				'<' => self.parse_element_or_comment(),
				_ => self.parse_text(),
		}
	}
	
	fn parse_element_or_comment(&mut self) -> dom::Node {
		assert!(self.consume_char() == '<');
		match self.next_char() {
			'!' => self.parse_comment(),
			_ => self.parse_element(),
		}
	}
	
	fn parse_comment(&mut self) -> dom::Node {
		assert!(self.consume_char() == '!');
		loop {
			let delimiter = self.consume_char();
			if delimiter == '-' && self.next_char() != delimiter {
				break;
			}
		}
		self.consume_whitespace();
		let comment = self.consume_while(|c| c != '-');
		
		assert!(self.consume_char() == '-');
    assert!(self.consume_char() == '-');
    assert!(self.consume_char() == '>');
		return dom::comment(comment.trim_end().to_string());
	}
	
	fn parse_text(&mut self) -> dom::Node {
		dom::text(self.consume_while(|c| c != '<'))
	}
	
	fn parse_element(&mut self) -> dom::Node {
		let tag_name = self.parse_tag_name();
		let attributes = self.parse_attributes();
		assert!(self.consume_char() == '>');
		
		let children = self.parse_nodes();
		
		assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.consume_char() == '>');
		
		return dom::element(tag_name, attributes, children);
	}
	
	fn parse_nodes(&mut self) -> Vec<dom::Node> {
		let mut nodes = Vec::new();
		loop {
			self.consume_whitespace();
			if self.ended() || self.starts_with("</") {
				break;
			}
			nodes.push(self.parse_node());
		}
		return nodes;
	}
	
	fn parse_attributes(&mut self) -> dom::AttrMap {
		let mut attributes = HashMap::new();
		loop {
				self.consume_whitespace();
				if self.next_char() == '>' {
					break;
				}
				let (name, value) = self.parse_attr();
				attributes.insert(name, value);
		}
		return attributes;
	}
	
	fn parse_attr(&mut self) -> (String, String) {
		let name = self.parse_tag_name();
		assert!(self.consume_char() == '=');
		let value = self.parse_attr_value();
		return (name, value);
	}
	
	fn parse_attr_value(&mut self) -> String {
		let open_quote = self.consume_char();
		assert!(open_quote == '"' || open_quote == '\'');
		
		let value = self.consume_while(|c| c != open_quote);
		assert!(self.consume_char() == open_quote);
		return value;
	}
		
	fn consume_whitespace(&mut self) {
		self.consume_while(char::is_whitespace);
	}
	
	fn parse_tag_name(&mut self) -> String {
		self.consume_while(|character| match character {
			'a'..='z' | 'A'..='Z' | '0'..='9' => true,
			_ => false
		})
	}
	
	fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
		let mut result = String::new();
		while !self.ended() && test(self.next_char()) {
				result.push(self.consume_char());
		}
		return result;
	}
	
	fn consume_char(&mut self) -> char {
		let mut iterable = self.input[self.position..].char_indices();
		let (_, current_char) = iterable.next().unwrap();
    let (next_position, _) = iterable.next().unwrap_or((1, ' '));
		self.position += next_position;
    return current_char;
	}
	
	fn next_char(&self) -> char {
		self.input[self.position..].chars().next().unwrap()
	}
	
	fn starts_with(&self, test_str: &str) -> bool {
		self.input[self.position..].starts_with(test_str)
	}
	
	fn ended(&self) -> bool {
		self.position >= self.input.len()
	}
}
