use std::cell::RefCell;
use std::rc::Rc;

pub use gl_window::GlWindow;

use crate::{Color, ShapeObject};

mod gl_window;

/// Alias de: Lista de objetos a dibujar
pub type GlShapeList = Vec<ShapeObject>;

pub fn run_loop_standalone() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    let event_loop = glium::winit::event_loop::EventLoop::new().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(GlWindow::WINDOW_TITLE)
        .with_inner_size(800, 600)
        .build(&event_loop);
    let mut shapes = GlShapeList::new();
    let center = (65, 60);

    shapes.push(ShapeObject::new_circle(40, center));
    shapes.push(ShapeObject::new_square(50, center));
    shapes.push(ShapeObject::new_rectangle(60, 30, center));
    // Editar configuracion de estilo
    let mut ellipse = ShapeObject::new_ellipse(33, 55, center);
    *ellipse.style_mut() = ellipse.style_mut().stroke_color(Color::from_u32_rgb(0x00ff00)).stroke_width(2.0);
    shapes.push(ellipse);

    let shapes_list = Rc::new(RefCell::new(shapes));
    let mut this = GlWindow { program: None, display, window, shapes_list, background_color: Color::from_u32_rgb(0x3367af) };
    event_loop.run_app(&mut this).unwrap();
}
