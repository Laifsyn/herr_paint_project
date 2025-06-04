use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    color: [u8; 3],
    background_color: [u8; 3],
    figuras: Vec<String>,
    grosor: f32,
    transparente: bool,
    cuadrado: u32,
    centro_cuadrado: (i32, i32),
    largoRectangulo: u32,
    anchoRectangulo: u32,
    centro_rectangulo: (i32, i32),
    radio1Elipse: u32,
    radio2Elipse: u32,
    centro_elipse: (i32, i32),
    radioCirculo: u32,
    centro_circulo: (i32, i32),
}

pub use gl_window::GlWindow;

use crate::{Color, ShapeObject};

mod gl_window;

/// Alias de: Lista de objetos a dibujar
pub type GlShapeList = Vec<ShapeObject>;

pub fn run_loop_standalone() {
    let event_loop = glium::winit::event_loop::EventLoop::new().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title(GlWindow::WINDOW_TITLE)
        .with_inner_size(800, 600)
        .build(&event_loop);

    let config: Config = serde_json::from_str(&fs::read_to_string("config.json").expect("No se pudo leer config.json"))
        .expect("No se pudo deserializar config.json");

    let r = (config.color[0]) as u8;
    let g = (config.color[1]) as u8;
    let b = (config.color[2]) as u8;
    let borde_color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

    let [r, g, b] = config.background_color;
    let background = Color::from_rgb(r, g, b);
    let mut shapes = GlShapeList::new();

    for figura in &config.figuras {
        match figura.as_str() {
            "Circulo" => {
                let mut circle = ShapeObject::new_circle(config.radioCirculo, config.centro_circulo);
                *circle.style_mut() = circle
                    .style_mut()
                    .stroke_color(Color::from_u32_rgb(borde_color))
                    .stroke_width(config.grosor as f32)
                    .fill_color(background);
                shapes.push(circle);
            }
            "Cuadrado" => {
                let mut square = ShapeObject::new_square(config.cuadrado, config.centro_cuadrado);
                *square.style_mut() = square
                    .style_mut()
                    .stroke_color(Color::from_u32_rgb(borde_color))
                    .stroke_width(config.grosor as f32)
                    .fill_color(background);
                shapes.push(square);
            }
            "Rectangulo" => {
                let mut rectangle =
                    ShapeObject::new_rectangle(config.anchoRectangulo, config.largoRectangulo, config.centro_rectangulo);
                *rectangle.style_mut() = rectangle
                    .style_mut()
                    .stroke_color(Color::from_u32_rgb(borde_color))
                    .stroke_width(config.grosor as f32)
                    .fill_color(background);
                shapes.push(rectangle);
            }
            "Elipse" => {
                let mut ellipse = ShapeObject::new_ellipse(config.radio1Elipse, config.radio2Elipse, config.centro_elipse);
                *ellipse.style_mut() = ellipse
                    .style_mut()
                    .stroke_color(Color::from_u32_rgb(borde_color))
                    .stroke_width(config.grosor as f32)
                    .fill_color(background);
                shapes.push(ellipse);
            }
            _ => {}
        }
    }
    // Editar configuracion de estilo

    let shapes_list = Rc::new(RefCell::new(shapes));
    let mut this = GlWindow { program: None, display, window, shapes_list, background_color: Color::from_u32_rgb(0xffffff) };
    event_loop.run_app(&mut this).unwrap();
}
