use eframe::egui::{self, Color32};
use serde::{Deserialize, Serialize};

/// Representación de un color RGB-8bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Color(pub [u8; 3]);

impl Color {
    pub const BLACK: Color = Color::from_u32_rgb(0x333333);

    /// Convierte un entero de 32 bits a un color RGB-8bits.
    ///
    /// # Nota
    ///
    /// Implementación ignora los primeros 8 bits (i.e. `0x FF000000`).
    /// Ejemplo:
    /// ```
    /// 0xFF_0F_FF_FF
    ///   ^^
    ///   ||
    /// Truncados
    /// ```
    ///
    /// # Pánico
    ///
    /// Cuando las aserciones de depuración están habilitadas: entra en pánico si el valor no está
    /// en el formato 0RGB de 8 bits.
    pub const fn from_u32_rgb(value: u32) -> Self {
        debug_assert!((value >> 24) == 0, "Expects a 0RGB-8bits format");
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        Color([r, g, b])
    }

    /// Convierte un set de valores RGB a un color RGB-8bits.
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self { Color([r, g, b]) }

    /// Convierte los valores a un formato RGB basado en flotante.
    pub const fn to_vec(&self) -> [f32; 3] {
        let Color([r, g, b]) = *self;
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        [r, g, b]
    }

    pub fn update_red(&mut self, value: u8) { self.0[0] = value; }

    pub fn update_green(&mut self, value: u8) { self.0[1] = value; }

    pub fn update_blue(&mut self, value: u8) { self.0[2] = value; }

    pub fn as_mut_slice(&mut self) -> &mut [u8; 3] { &mut self.0 }

    pub fn as_slice(&self) -> &[u8; 3] { &self.0 }
}

impl From<Color> for egui::Color32 {
    fn from(value: Color) -> Self {
        let [r, g, b] = value.0;
        Color32::from_rgb(r, g, b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Color([r, g, b]) }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self { Color(value) }
}

impl From<u32> for Color {
    /// Utiliza un esquema de color RGB-8bits. El Alpha es ignorado.
    fn from(value: u32) -> Self { Self::from_u32_rgb(value) }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        let r = (value[0] * 255.0) as u8;
        let g = (value[1] * 255.0) as u8;
        let b = (value[2] * 255.0) as u8;
        Color([r, g, b])
    }
}

#[cfg(test)]
mod test {
    use super::Color;

    #[test]
    #[should_panic(expected = "Expects a 0RGB-8bits format")]
    fn panics_on_rgba() {
        const NUM: u32 = 0xFF_FF_FF_F0;
        Color::from_u32_rgb(NUM);
    }

    #[test]
    fn converts_u32_to_rgb() {
        const NUM: u32 = 0xFF007F;
        let color = Color::from_u32_rgb(NUM);
        assert_eq!(color, Color([255, 0, 127]));
    }
}
