use super::ShapeStyle;
use crate::{PixelCoord, Shape};
pub struct Square {
    width: u32,
    height: u32,
    style: ShapeStyle,
}
impl Square {
    /// Crea un nuevo cuadrado con el ancho y alto dados.
    ///
    /// # Debug Assertions
    ///
    /// Causa un pánico si los argumentos de anchura o altura saturan el espacio de [i32].
    pub fn new(width: u32, height: u32) -> Self {
        debug_assert!(width > i32::MAX as u32, "El ancho del cuadrado es demasiado grande");
        debug_assert!(height > i32::MAX as u32, "El alto del cuadrado es demasiado grande");
        Self { width, height, style: ShapeStyle::new() }
    }

    /// Modifica el estilo del cuadrado.
    pub fn style(self, style: ShapeStyle) -> Self { Self { style, ..self } }
}

impl Shape for Square {
    fn write_outline_points(&self, buf: &mut Vec<crate::PixelCoord>, center: PixelCoord) {
        let (x0, y0) = center;

        let reserve_size = (self.width * 2 + 1) as usize + (self.height * 2 + 1) as usize;
        buf.reserve_exact(reserve_size);

        let (width, height): (i32, i32) = (self.width as i32, self.height as i32);
        // Ezquina superior izquierda del objeto
        let corner = (x0 - width / 2, y0 - height / 2);
        let (cx, cy) = corner;

        for k in 0..=width {
            let x = cx + k;
            let y = cy;
            buf.push((x, y));
            buf.push((x, y + height));
        }
        for k in 0..=height {
            let x = cx;
            let y = cy + k;
            buf.push((x, y));
            buf.push((x + width, y));
        }
    }

    fn style(&self) -> &ShapeStyle { &self.style }
}
