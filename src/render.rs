use std::fs;

use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};

use crate::{dom::{Node, NodeType}, style::StyledNode};

#[derive(Debug)]
struct Bounds {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Coordinates {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Coordinates {
    fn move_left(&mut self, increment: f64) {
			if self.x > 0.0 {
				self.x -= increment;
			}
    }

    fn move_right(&mut self, increment: f64, bounds: &Bounds) {
        if self.x < bounds.width.into() {
					self.x += increment;
				}
    }

    fn move_up(&mut self, increment: f64) {
        if self.y > 0.0 {
					self.y -= increment;
				}
    }

    fn move_down(&mut self, increment: f64, bounds: &Bounds) {
        if self.y < bounds.height.into() {
					self.y += increment;
				}
    }
}

#[derive(Debug)]
pub struct Renderer {
    surface: ImageSurface,
    context: Context,
    bounds: Bounds,
    coords: Coordinates,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        let surface = ImageSurface::create(Format::ARgb32, width, height)
            .expect("Could not create a surface");
        let context = Context::new(&surface).expect("Could not create context!");
        Renderer {
            surface,
            context,
            bounds: Bounds { width, height },
            coords: Coordinates { x: 0.0, y: 16.0 },
        }
    }

    pub fn to_image(&mut self, root_node: StyledNode) {
        self.context.set_source_rgb(1.0, 1.0, 1.0);
        self.context.paint().expect("Paint failed!");

        self.context.set_source_rgb(0.0, 0.0, 0.0);

        let font_size = 16.0;
        self.context
            .select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        self.context.set_font_size(font_size);
				
				self.walk_node_tree(&root_node);
				
				let mut file = fs::File::create("output.png")
					.expect("Couldn't create file.");
				self.surface.write_to_png(&mut file)
					.expect("Couldn't write to png");
    }
		
		fn walk_node_tree(&mut self, next_node: &StyledNode) {		
			let should_move = self.render_text(&next_node);
			
			if should_move {
				self.coords.move_down(18.00, &self.bounds);
			}
			
			if next_node.children.len() > 0 {
				for child in next_node.children.iter() {
					self.walk_node_tree(child);					
				}
			}
		}
		
		fn render_text(&self, node: &StyledNode) -> bool {
			match &node.node.node_type {
				NodeType::Text(content) => {
					println!("{:?}", self.coords);
					self.context.move_to(self.coords.x, self.coords.y);
					self.context.show_text(content).expect("Writing text failed");
					return true;
				}
				_ => false
			}
		}
}
