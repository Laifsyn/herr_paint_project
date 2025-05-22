use crate::geometries::{Circle, Ellipse, Square};
use crate::{PixelCoord, Shape};

/// Estructura que describe un Objeto 2D dibujable. Incluye información sobre us posición.
pub struct ShapeObject {
    shape: DrawableShape,
    pub center: PixelCoord,
}

impl ShapeObject {
    /// Crea un nuevo objeto de forma con la figura y el centro dados.
    ///
    /// # Debug Assertions
    ///
    /// Causa un pánico si el centro del objeto está fuera del rango de [i32].
    fn new<T: Into<DrawableShape>>(shape: T, center: PixelCoord) -> Self {
        debug_assert!(center.0 < i32::MAX, "El centro del objeto está fuera del rango de i32");
        debug_assert!(center.1 < i32::MAX, "El centro del objeto está fuera del rango de i32");
        Self { shape: shape.into(), center }
    }

    /// Construye un objeto cuadrado
    pub fn new_square(side: u32, center: PixelCoord) -> Self {
        let square = Square::new(side, side);
        Self::new(square, center)
    }

    /// Construye un objeto circular
    pub fn new_circle(radius: u32, center: PixelCoord) -> Self {
        let circle = Circle::new(radius);
        Self::new(circle, center)
    }

    /// Construye un objeto elíptico
    pub fn new_ellipse(radius_x: u32, radius_y: u32, center: PixelCoord) -> Self {
        match Ellipse::new(radius_x, radius_y) {
            Ok(ellipse) => Self::new(ellipse, center),
            Err(circle) => Self::new(circle, center),
        }
    }

    /// Construye un objeto rectangular
    pub fn new_rectangle(width: u32, height: u32, center: PixelCoord) -> Self {
        let rectangle = Square::new(width, height);
        Self::new(rectangle, center)
    }

    /// Obtiene una referencia editable al estilo de la figura.
    pub fn style_mut(&mut self) -> &mut crate::ShapeStyle {
        match &mut self.shape {
            DrawableShape::Square(s) => &mut s.style,
            DrawableShape::Circle(s) => &mut s.style,
            DrawableShape::Ellipse(s) => &mut s.style,
            DrawableShape::Rectangle(s) => &mut s.style,
        }
    }
}

impl Shape for ShapeObject {
    fn write_outline_points_at(&self, buf: &mut Vec<PixelCoord>, center: PixelCoord) {
        self.shape.write_outline_points_at(buf, center);
    }

    /// Escribe al buffer dado los puntos que forman el contorno del objeto, centrado en
    /// [`ShapeObject::center`].
    fn write_outline_points(&self, buf: &mut Vec<PixelCoord>) { self.write_outline_points_at(buf, self.center); }

    fn style(&self) -> &crate::ShapeStyle { self.shape.style() }
}

/// Diferentes variantes de formas que pueden ser dibujadas
///
/// # Elemeto Privado
///
/// Revisar [`ShapeObject`](crate::ShapeObject) para más detalles.
enum DrawableShape {
    Square(Square),
    Rectangle(Square),
    Circle(Circle),
    Ellipse(Ellipse),
}
impl Shape for DrawableShape {
    fn write_outline_points_at(&self, buf: &mut Vec<PixelCoord>, center: PixelCoord) {
        match self {
            DrawableShape::Square(s) => s.write_outline_points_at(buf, center),
            DrawableShape::Circle(s) => s.write_outline_points_at(buf, center),
            DrawableShape::Ellipse(s) => s.write_outline_points_at(buf, center),
            DrawableShape::Rectangle(s) => s.write_outline_points_at(buf, center),
        }
    }

    fn style(&self) -> &crate::ShapeStyle {
        match self {
            DrawableShape::Square(s) => s.style(),
            DrawableShape::Circle(s) => s.style(),
            DrawableShape::Ellipse(s) => s.style(),
            DrawableShape::Rectangle(s) => s.style(),
        }
    }
}

impl From<Square> for DrawableShape {
    fn from(square: Square) -> Self {
        let (width, height, _) = square.read_fields();
        if width == height { DrawableShape::Square(square) } else { DrawableShape::Rectangle(square) }
    }
}
impl From<Circle> for DrawableShape {
    fn from(circle: Circle) -> Self { DrawableShape::Circle(circle) }
}
impl From<Ellipse> for DrawableShape {
    fn from(ellipse: Ellipse) -> Self { DrawableShape::Ellipse(ellipse) }
}
