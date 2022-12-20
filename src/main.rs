extern crate cairo;

use std::fs;

pub mod css;
pub mod cssom;
pub mod dom;
pub mod html;
pub mod parser;
pub mod render;
pub mod style;

fn main() {
    let html_string = fs::read_to_string("./examples/test.html").expect("Failed to read file");
    let node_tree = html::parse(html_string);

    let css_string = fs::read_to_string("./examples/test.css").expect("Failed to read file");
    let stylesheet = css::parse(css_string);

    let style_tree = style::style_tree(&node_tree, &stylesheet);

    let mut renderer = render::Renderer::new(600, 600);

    renderer.to_image(style_tree)
}
