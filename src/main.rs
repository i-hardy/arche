extern crate cairo;
extern crate gtk;

use std::fs;
use cairo::Context;
use gtk::prelude::*;
use gtk::DrawingArea;

use parse::{html, css, style};
use render::renderer::Renderer;

pub mod parse;
pub mod render;

fn build_ui(application: &gtk::Application) {
	drawable(application, 500, 500, |_, cr| {
		let html_string = fs::read_to_string("./examples/test.html").expect("Failed to read file");
    let node_tree = html::parse(html_string);

    let css_string = fs::read_to_string("./examples/test.css").expect("Failed to read file");
    let stylesheet = css::parse(css_string);

    let style_tree = style::style_tree(&node_tree, &stylesheet, None);
		
		let mut renderer = Renderer::new(cr, 500, 500);
		
		renderer.draw(style_tree);

		Inhibit(false)
});
}

fn main() {	
		let application = gtk::Application::new(
			Some("com.github.i-hardy.arche"),
			Default::default(),
		);

		application.connect_activate(build_ui);

		application.run();
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
}
