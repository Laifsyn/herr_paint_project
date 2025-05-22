//! Figuras geométricas 2D
mod circle;
mod shape;
mod square;
pub use circle::{Circle, Ellipse};
pub use shape::ShapeObject;
pub use square::Square;

use crate::{Color, PixelCoord, Vertex};

/// Estilo de una figura geométrica.
#[derive(Clone, Copy)]
pub struct ShapeStyle {
    /// Color del borde de la figura
    pub stroke_color: Option<Color>,
    /// Color de relleno de la figura
    pub fill_color: Option<Color>,
    /// Describe el grosor del bordea
    pub stroke_width: f32,
}

impl ShapeStyle {
    fn new() -> Self { Self { stroke_color: Some(Color::BLACK), fill_color: None, stroke_width: 1.0 } }

    /// Cambia el grosor del borde del estilo.
    pub fn stroke_width(self, width: impl Into<f32>) -> Self { Self { stroke_width: width.into(), ..self } }

    /// Cambia el color del borde del estilo.
    pub fn stroke_color(self, color: impl Into<Color>) -> Self { Self { stroke_color: Some(color.into()), ..self } }

    /// Cambia el color de relleno del estilo.
    pub fn fill_color(self, color: impl Into<Color>) -> Self { Self { fill_color: Some(color.into()), ..self } }

    /// Revisa si el estilo puede implicar una figura transparente.
    pub fn is_transparent(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self { stroke_color: None, fill_color: None, .. } => true,
            Self { stroke_color: Some(_), fill_color: None, stroke_width: 0.0 } => true,
            _ => false,
        }
    }
}

impl Default for ShapeStyle {
    fn default() -> Self { Self::new() }
}

/// Define como una figura geométrica es representable en un espacio 2D.
pub trait Shape {
    /// Escribe al buffer dado los puntos que forman el contorno del objeto.
    fn write_outline_points_at(&self, buf: &mut Vec<PixelCoord>, center: PixelCoord);

    /// Computa las coordenadas de los puntos que forman el contorno del objeto, y los devuelve como
    /// un vector.
    fn to_outline_points(&self, center: PixelCoord) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        self.write_outline_points_at(&mut points, center);
        points
    }

    /// Escribe al buffer dado los puntos que forman el contorno del objeto, centrado en el origen.
    ///
    /// # Nota
    ///
    /// Por defecto utiliza [`Shape::write_outline_points_at`] con el centro en (0, 0).
    fn write_outline_points(&self, buf: &mut Vec<PixelCoord>) {
        let center = (0, 0);
        self.write_outline_points_at(buf, center);
    }

    /// Devuelve el estilo de la figura.
    fn style(&self) -> &ShapeStyle;

    #[inline]
    /// Escribe al buffer dado los Vértices coloreados según el estilo definido por
    /// [`Shape::style`].
    fn points_to_vertex(&self, points: &[PixelCoord], buf: &mut Vec<Vertex>) -> usize {
        if self.style().is_transparent() {
            return 0;
        }
        let Some(color) = self.style().stroke_color else {
            unreachable!("We already checked for transparency");
        };

        let mut writes = 0;
        let iter = points.iter().map(|&(x, y)| {
            writes += 1;
            Vertex::new([x, y], color)
        });
        buf.extend(iter);

        writes
    }

    /// Escribe al buffer dado los puntos que forman el relleno del objeto.
    fn flood_fill(&self, outline_points: &[PixelCoord], buf: &mut Vec<PixelCoord>) {
        let _ = (outline_points, buf); // Silenciar advertencias de variables no utilizadas
        unimplemented!("Flood fill not implemented for this shape yet");
    }
}
