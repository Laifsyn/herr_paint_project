//! Vértices con color en OpenGL

use glium::implement_vertex;

use super::color::Color;

/// Alias para el tipo de color utilizado en el programa de OpenGL.
pub type Rgb = [f32; 3];

/// Representación de un vértice en el programa.
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [i32; 2],
    pub color: Rgb,
}
implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new(position: [i32; 2], color: impl Into<Color>) -> Self { Vertex { position, color: color.into().to_vec() } }
}
