use cairo::Context;

use crate::parse::{
    cssom::{Color, Value},
    style::StyledNode, dom::NodeType,
};

#[derive(Debug)]
pub struct Dimensions {
	pub x: f64,
	pub y: f64,
}

#[derive(Debug)]
struct VisualRules {
    font_size: f64,
    color: Color,
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
				NodeType::Text(_) => Dimensions { x: 0.0, y: self.visuals.font_size },
				_ => Dimensions { x: 0.0, y: 0.0 }
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

impl VisualRules {
    fn new(node: &StyledNode) -> VisualRules {
        let font_size = node.specified_values.get("font-size");
        let color = node.specified_values.get("color");

        VisualRules {
            font_size: match font_size {
                Some(Value::Length(size, _)) => (*size).into(),
                _ => 0.0,
            },
            color: match color {
								Some(Value::ColorValue(color)) => color.clone(),
								_ => Color::default()
						},
        }
    }
		
		fn color_to_rgb(&self) -> (f64, f64, f64) {
			(self.color.r.into(), self.color.g.into(), self.color.b.into())
		}
}
