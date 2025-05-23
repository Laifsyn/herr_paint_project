//! Máquinas de Estados del programa

use eframe::egui::Color32;

use crate::Color;

/// Indica qué objeto interno de la interfaz está seleccionado
pub enum SelectedItem {
    /// No hay figura seleccionada.
    None,
    /// Índice de la figura seleccionada.
    ShapeId(usize),
}

impl SelectedItem {
    pub fn as_mut_usize(&mut self) -> Option<&mut usize> {
        match self {
            SelectedItem::None => None,
            SelectedItem::ShapeId(id) => Some(id),
        }
    }
}

pub enum SliderEditor {
    /// Editando el color de fondo de la ventana.
    Background,
    /// Editando el color del borde de una figura.
    Stroke,
    /// Editando el color de relleno de una figura.
    Fill,
    /// No hay objeto seleccionado para editar el color.
    None,
}

#[derive(Clone, Copy)]
/// Coloramiento para la figura. Indica si el color es transparente u opaco.
pub enum AlphaColoring {
    /// Invisible.
    Transparent(Color),
    /// Color opaco.
    Opaque(Color),
}

impl AlphaColoring {
    pub fn is_transparent(&self) -> bool { matches!(self, AlphaColoring::Transparent(_)) }

    pub fn is_opaque(&self) -> bool { matches!(self, AlphaColoring::Opaque(_)) }
}

impl From<AlphaColoring> for Color32 {
    fn from(value: AlphaColoring) -> Self {
        let [r, g, b] = value.as_inner().0;
        match value {
            AlphaColoring::Transparent(_) => Color32::from_rgba_premultiplied(r, g, b, 0),
            AlphaColoring::Opaque(_) => Color32::from_rgba_premultiplied(r, g, b, 255),
        }
    }
}

impl AlphaColoring {
    pub fn as_inner(self) -> Color {
        match self {
            AlphaColoring::Transparent(color) => color,
            AlphaColoring::Opaque(color) => color,
        }
    }

    pub fn as_inner_ref(&self) -> &Color {
        match self {
            AlphaColoring::Transparent(color) => color,
            AlphaColoring::Opaque(color) => color,
        }
    }

    pub fn as_inner_ref_mut(&mut self) -> &mut Color {
        match self {
            AlphaColoring::Transparent(color) => color,
            AlphaColoring::Opaque(color) => color,
        }
    }

    pub fn as_slice(&self) -> &[u8; 3] { self.as_inner_ref().as_slice() }
}

impl From<Color> for AlphaColoring {
    /// Siempre devuelve [`AlphaColoring::Opaque`]
    fn from(value: Color) -> Self { AlphaColoring::Opaque(value) }
}
