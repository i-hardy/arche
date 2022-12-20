#[derive(Debug)]
pub struct StyleSheet {
	pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
	Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
	pub tag_name: Option<String>,
	pub id: Option<String>,
	pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
	pub name: String,
	pub value: Value,
}

#[derive(Debug)]
pub enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
}

#[derive(Debug)]
pub enum Unit {
	Px,
}

#[derive(Debug)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

pub fn rule(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
	Rule { selectors, declarations }
}