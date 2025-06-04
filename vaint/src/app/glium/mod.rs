use std::fs;

pub use gl_window::GlWindow;

use crate::{Figura, ShapeObject};

mod gl_window;

/// Alias de: Lista de objetos a dibujar
pub type GlShapeList = Vec<ShapeObject>;

pub fn run_loop_standalone() {
    let event_loop = glium::winit::event_loop::EventLoop::new().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(GlWindow::WINDOW_TITLE)
        .with_inner_size(800, 600)
        .build(&event_loop);

    let config: crate::Config = serde_json::from_str(&fs::read_to_string("config.json").expect("No se pudo leer config.json"))
        .expect("No se pudo deserializar config.json");

    let stroke_color = config.stroke_color;
    let background = config.shape_background_color;
    let mut shapes = GlShapeList::new();

    for figura in &config.figuras {
        let mut shape = match figura {
            Figura::Circulo => ShapeObject::new_circle(config.radio_circulo, config.centro_circulo),
            Figura::Cuadrado => ShapeObject::new_square(config.cuadrado, config.centro_cuadrado),
            Figura::Rectangulo => {
                ShapeObject::new_rectangle(config.ancho_rectangulo, config.largo_rectangulo, config.centro_rectangulo)
            }
            Figura::Elipse => ShapeObject::new_ellipse(config.radio1_elipse, config.radio2_elipse, config.centro_elipse),
        };
        *shape.style_mut() = shape.style_mut().stroke_color(stroke_color).fill_color(background).stroke_width(config.grosor);
        shapes.push(shape);
    }
    // Editar configuracion de estilo

    let shapes_list = shapes;

    let mut this = GlWindow { program: None, display, window, shapes_list, background_color: config.background_color };
    event_loop.run_app(&mut this).unwrap();
}
