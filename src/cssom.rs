pub struct StyleSheet {
	pub rules: Vec<Rule>,
}

pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declarations: Vec<Declaration>,
}

pub enum Selector {
	Simple(SimpleSelector),
}

pub struct SimpleSelector {
	pub tag_name: Option<String>,
	pub id: Option<String>,
	pub class: Vec<String>,
}

pub struct Declaration {
	pub name: String,
	pub value: Value,
}

pub enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
}

pub enum Unit {
	Px,
}

pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

pub fn rule(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
	Rule { selectors, declarations }
}