use std::{collections::HashMap};

use crate::{dom, parser::Parser};

pub fn parse(source: String) -> dom::Node {
	let mut nodes = HTMLParser::new(source).parse_nodes();
	
	if nodes.len()== 1 {
		nodes.swap_remove(0)
	} else {
		dom::element("html".to_string(), HashMap::new(), nodes)
	}
}

struct HTMLParser {
	parser: Parser
}

impl HTMLParser {
	fn new(input: String) -> HTMLParser {
		HTMLParser { parser: Parser::new(0, input) }
	}
	
	fn parse_node(&mut self) -> dom::Node {
		match self.parser.next_char() {
				'<' => self.parse_element_or_comment(),
				_ => self.parse_text(),
		}
	}
	
	fn parse_element_or_comment(&mut self) -> dom::Node {
		assert!(self.parser.consume_char() == '<');
		match self.parser.next_char() {
			'!' => self.parse_comment(),
			_ => self.parse_element(),
		}
	}
	
	fn parse_comment(&mut self) -> dom::Node {
		assert!(self.parser.consume_char() == '!');
		loop {
			let delimiter = self.parser.consume_char();
			if delimiter == '-' && self.parser.next_char() != delimiter {
				break;
			}
		}
		self.parser.consume_whitespace();
		let comment = self.parser.consume_while(|c| c != '-');
		
		assert!(self.parser.consume_char() == '-');
    assert!(self.parser.consume_char() == '-');
    assert!(self.parser.consume_char() == '>');
		return dom::comment(comment.trim_end().to_string());
	}
	
	fn parse_text(&mut self) -> dom::Node {
		dom::text(self.parser.consume_while(|c| c != '<'))
	}
	
	fn parse_element(&mut self) -> dom::Node {
		let tag_name = self.parse_tag_name();
		let attributes = self.parse_attributes();
		assert!(self.parser.consume_char() == '>');
		
		let children = self.parse_nodes();
		
		assert!(self.parser.consume_char() == '<');
    assert!(self.parser.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.parser.consume_char() == '>');
		
		return dom::element(tag_name, attributes, children);
	}
	
	fn parse_nodes(&mut self) -> Vec<dom::Node> {
		let mut nodes = Vec::new();
		loop {
			self.parser.consume_whitespace();
			if self.parser.ended() || self.parser.starts_with("</") {
				break;
			}
			nodes.push(self.parse_node());
		}
		return nodes;
	}
	
	fn parse_attributes(&mut self) -> dom::AttrMap {
		let mut attributes = HashMap::new();
		loop {
				self.parser.consume_whitespace();
				if self.parser.next_char() == '>' {
					break;
				}
				let (name, value) = self.parse_attr();
				attributes.insert(name, value);
		}
		return attributes;
	}
	
	fn parse_attr(&mut self) -> (String, String) {
		let name = self.parse_tag_name();
		assert!(self.parser.consume_char() == '=');
		let value = self.parse_attr_value();
		return (name, value);
	}
	
	fn parse_attr_value(&mut self) -> String {
		let open_quote = self.parser.consume_char();
		assert!(open_quote == '"' || open_quote == '\'');
		
		let value = self.parser.consume_while(|c| c != open_quote);
		assert!(self.parser.consume_char() == open_quote);
		return value;
	}
	
	fn parse_tag_name(&mut self) -> String {
		self.parser.consume_while(|character| match character {
			'a'..='z' | 'A'..='Z' | '0'..='9' => true,
			_ => false
		})
	}
}
