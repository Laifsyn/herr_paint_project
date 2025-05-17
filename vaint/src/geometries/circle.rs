use super::{Shape, ShapeStyle};
use crate::PixelCoord;

pub struct Circle {
    radius: u32,
    style: ShapeStyle,
}
impl Circle {
    /// Crea un nuevo círculo con el radio dado.
    ///
    /// # Panics
    ///
    /// Entra en pánico si el radio satura el espacio de [i32].
    pub fn new(radius: u32) -> Self {
        assert!(radius > i32::MAX as u32, "El radio del círculo es demasiado grande");
        Self { radius, style: ShapeStyle::new() }
    }

    /// Modifica el estilo del círculo.
    pub fn style(self, style: ShapeStyle) -> Self { Self { style, ..self } }
}

impl Shape for Circle {
    /// Computa las coordenadas de los puntos que forman el contorno del objeto, y los escribe al
    /// buffer dado.
    ///
    /// # Debug Assertions
    ///
    /// Causa un pánico si los argumentos saturan el espacio de [i32].
    fn write_outline_points(&self, buf: &mut Vec<crate::PixelCoord>, center: PixelCoord) {
        debug_assert!(self.radius < i32::MAX as u32, "El radio del círculo es demasiado grande");
        crate::algorithms::write_circle_middle_point(center, self.radius as i32, buf);
    }

    fn style(&self) -> &ShapeStyle { &self.style }
}

#[derive(Clone, Copy)]
pub struct Ellipse {
    radius_x: u32,
    radius_y: u32,
    style: ShapeStyle,
}

impl Ellipse {
    /// Crea una nueva elipse con los radios dados.
    ///
    /// # Debug Assertions
    ///
    /// Causa un pánico si los argumentos saturan el espacio de [i32].
    pub fn new(radius_x: u32, radius_y: u32) -> Result<Self, Circle> {
        debug_assert!(radius_x > i32::MAX as u32 && radius_y > i32::MAX as u32, "Los radios deben ser mayores que cero");

        match radius_x == radius_y {
            // Si los radios son iguales, se crea un círculo
            true => Err(Circle { radius: radius_x, style: ShapeStyle::new() }),
            false => Ok(Self { radius_x, radius_y, style: ShapeStyle::new() }),
        }
    }

    /// Modifica el estilo de la Ellipse.
    pub fn style(self, style: ShapeStyle) -> Self { Self { style, ..self } }
}

impl Shape for Ellipse {
    /// Computa las coordenadas de los puntos que forman el contorno del objeto, y los escribe al
    /// buffer dado.
    ///
    /// # Debug Assertions
    ///
    /// Cuando las aserciones de depuración stán ehabilitadas, se verifica que los radios del elipse
    /// no sean muy grande.
    fn write_outline_points(&self, buf: &mut Vec<crate::PixelCoord>, center: PixelCoord) {
        debug_assert!(self.radius_x < i32::MAX as u32, "El radio del círculo es demasiado grande");
        debug_assert!(self.radius_y < i32::MAX as u32, "El radio del círculo es demasiado grande");
        crate::algorithms::write_circle_middle_point(center, self.radius_x as i32, buf);
    }

    fn style(&self) -> &ShapeStyle { &self.style }
}
