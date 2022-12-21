use cairo::{Context, FontSlant, FontWeight};

use crate::parse::{style::StyledNode};

use super::visuals::Block;

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
pub struct Renderer<'a> {
    context: &'a Context,
    bounds: Bounds,
    coords: Coordinates,
}

impl Renderer<'_> {
    pub fn new(context: &Context, width: i32, height: i32) -> Renderer {
        Renderer {
            context,
            bounds: Bounds { width, height },
            coords: Coordinates { x: 0.0, y: 0.0 },
        }
    }

    pub fn draw(&mut self, root_node: StyledNode) {
        self.context.set_source_rgb(1.0, 1.0, 1.0);
        self.context.paint().expect("Paint failed!");

        self.context
            .select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);

        self.walk_node_tree(&root_node);
    }

    fn walk_node_tree(&mut self, next_node: &StyledNode) {
				let painting_block = Block::new(next_node);
				let block_dimensions = painting_block.dimensions();
								
				self.coords.move_down(block_dimensions.y, &self.bounds);
				self.context.move_to(self.coords.x, self.coords.y);
				
				println!("{:?}", self.coords);
				
				painting_block.paint(self.context);
				
        if !next_node.children.is_empty() {
            for child in next_node.children.iter() {
                self.walk_node_tree(child);
            }
        }
    }
}
