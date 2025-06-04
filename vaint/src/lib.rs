pub use geometries::{Shape, ShapeObject, ShapeStyle};
pub use glium_app::{GlShapeList, GlWindow};
pub use opengl::{Color, PixelCoord, Vertex};
use serde::{Deserialize, Serialize};

#[path = "util/algorithms.rs"]
pub mod algorithms;
#[path = "app/egui/mod.rs"]
pub mod egui_app;
pub mod geometries;
#[path = "app/glium/mod.rs"]
pub mod glium_app;

mod opengl;
#[path = "util/tracing.rs"]
pub mod tracing;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub color: [u8; 3],
    pub background_color: [u8; 3],
    pub figuras: Vec<String>,
    pub grosor: f32,
    pub cuadrado: u32,
    pub centro_cuadrado: (i32, i32),
    pub largo_rectangulo: u32,
    pub ancho_rectangulo: u32,
    pub centro_rectangulo: (i32, i32),
    pub radio1_elipse: u32,
    pub radio2_elipse: u32,
    pub centro_elipse: (i32, i32),
    pub radio_circulo: u32,
    pub centro_circulo: (i32, i32),
}
