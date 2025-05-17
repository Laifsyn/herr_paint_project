//! Utilidades para el dibujo con OpenGl
mod color;
mod vertex;

/// Alias para coordenadas en pixeles
pub type PixelCoord = (i32, i32);

pub use color::Color;
pub use vertex::Vertex;
