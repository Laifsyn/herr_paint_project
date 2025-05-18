use crate::geometries::{Circle, Ellipse, Square};

/// Diferentes variantes de formas que pueden ser dibujadas
pub enum DrawableShape {
    Square(Square),
    Rectangle(Square),
    Circle(Circle),
    Ellipse(Ellipse),
}
