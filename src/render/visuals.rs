use cairo::Context;

use crate::parse::{
    cssom::{Color, Value},
    style::StyledNode, dom::{NodeType, Node},
};

#[derive(Debug)]
pub struct Dimensions {
	pub x: f64,
	pub y: f64,
	pub top_y: f64,
	pub bottom_y: f64,
}

#[derive(Debug)]
struct Padding {
    top: f64,
		bottom: f64,
		left: f64,
		right: f64,
}

#[derive(Debug)]
struct VisualRules {
    font_size: f64,
    color: Color,
		padding: Padding,
}

#[derive(Debug)]
pub struct Block<'a> {
    node: &'a StyledNode<'a>,
    visuals: VisualRules,
}

impl Block<'_> {
    pub fn new<'a>(node: &'a StyledNode) -> Block<'a> {
			Block {
				node,
				visuals: VisualRules::new(node)
			}
		}

    pub fn paint(&self, context: &Context) {
			let (r, g, b) = self.visuals.color_to_rgb();
			context.set_source_rgb(r, g, b);
			context.set_font_size(self.visuals.font_size);
			self.render_text(context);
    }
		
		pub fn dimensions(&self) -> Dimensions {
			match self.node.node.node_type {
				NodeType::Text(_) => Dimensions { x: 0.0, y: self.visuals.font_size, top_y: 0.0, bottom_y: 0.0 },
				NodeType::Element(_) => Dimensions { x: 0.0, y: 0.0, top_y: self.visuals.padding.top, bottom_y: self.visuals.padding.bottom },
				_ => Dimensions { x: 0.0, y: 0.0, top_y: 0.0, bottom_y: 0.0 }
			}
		}
		
		fn render_text(&self, context: &Context) -> bool {
			let node = self.node;
			match &node.node.node_type {
					NodeType::Text(content) => {
							context
									.show_text(content)
									.expect("Writing text failed");
							true
					}
					_ => false,
			}
	}
}

fn get_length_or_default(maybe_length: &Option<&Value>) -> f64 {
	match maybe_length {
		Some(Value::Length(size, _)) => (*size).into(),
		_ => 0.0,
	}
}

impl VisualRules {
    fn new(node: &StyledNode) -> VisualRules {
        let font_size = node.specified_values.get("font-size");
        let color = node.specified_values.get("color");
				

        VisualRules {
            font_size: get_length_or_default(&font_size),
            color: match color {
								Some(Value::ColorValue(color)) => color.clone(),
								_ => Color::default()
						},
						padding: Padding::new(&node),
        }
    }
		
		fn color_to_rgb(&self) -> (f64, f64, f64) {
			(self.color.r.into(), self.color.g.into(), self.color.b.into())
		}
}

impl Padding {
		fn new(node: &StyledNode) -> Padding {
				let top = node.specified_values.get("padding-top");
				let bottom = node.specified_values.get("padding-bottom");
				let left = node.specified_values.get("padding-left");
				let right = node.specified_values.get("padding-right");
				
				Padding {
					top: get_length_or_default(&top),
					bottom: get_length_or_default(&bottom),
					left: get_length_or_default(&left),
					right: get_length_or_default(&right),
				}
		}
}

