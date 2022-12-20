pub struct Parser {
	position: usize,
	input: String,
}

impl Parser {
	pub fn new(position: usize, input: String) -> Parser {
		Parser { position, input }
	}
	
	pub fn consume_whitespace(&mut self) {
		self.consume_while(char::is_whitespace);
	}
	
	pub fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
		let mut result = String::new();
		while !self.ended() && test(self.next_char()) {
				result.push(self.consume_char());
		}
		return result;
	}
	
	pub fn consume_char(&mut self) -> char {
		let mut iterable = self.input[self.position..].char_indices();
		let (_, current_char) = iterable.next().unwrap();
    let (next_position, _) = iterable.next().unwrap_or((1, ' '));
		self.position += next_position;
    return current_char;
	}
	
	pub fn next_chars(&self, n: usize) -> String {
		self.input[self.position .. self.position + n].to_string()
	}
	
	pub fn next_char(&self) -> char {
		self.input[self.position..].chars().next().unwrap()
	}
	
	pub fn starts_with(&self, test_str: &str) -> bool {
		self.input[self.position..].starts_with(test_str)
	}
	
	pub fn ended(&self) -> bool {
		self.position >= self.input.len()
	}
}