use crate::parse::{
    cssom::{Declaration, Rule, Selector, SimpleSelector, Value, Unit, Color, StyleSheet},
    parser::Parser,
};

pub fn parse(source: String) -> StyleSheet {
	let mut parser = CSSParser::new(source);
	let rules = parser.parse_rules();
	StyleSheet { rules }
}

struct CSSParser {
    parser: Parser,
}

impl CSSParser {
    fn new(input: String) -> CSSParser {
        CSSParser {
            parser: Parser::new(0, input),
        }
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.parser.consume_whitespace();
            if self.parser.ended() {
                break;
            }
            rules.push(self.parse_rule());
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.parser.consume_whitespace();
            match self.parser.next_char() {
                ',' => {
                    self.parser.consume_char();
                    self.parser.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
				selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.parser.ended() {
            match self.parser.next_char() {
                '#' => {
                    self.parser.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.parser.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.parser.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        return selector;
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.parser.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.parser.consume_whitespace();
            if self.parser.next_char() == '}' {
                self.parser.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.parser.consume_whitespace();
        assert_eq!(self.parser.consume_char(), ':');
        self.parser.consume_whitespace();
        let value = self.parse_value();
        self.parser.consume_whitespace();
        assert_eq!(self.parser.consume_char(), ';');

        Declaration {
            name: property_name,
            value: value,
        }
    }

    fn parse_identifier(&mut self) -> String {
        self.parser.consume_while(valid_identifier_char)
    }

    fn parse_value(&mut self) -> Value {
        match self.parser.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.parser.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        s.parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit"),
        }
    }
		
		fn parse_color(&mut self) -> Value {
			assert_eq!(self.parser.consume_char(), '#');
			Value::ColorValue(Color {
					r: self.parse_hex_pair(),
					g: self.parse_hex_pair(),
					b: self.parse_hex_pair(),
					a: 255 
			})
		}

		fn parse_hex_pair(&mut self) -> u8 {
				let next_pair = self.parser.next_chars(2);
				self.parser.consume_char();
				self.parser.consume_char();
				u8::from_str_radix(&next_pair, 16).unwrap()
		}
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}
