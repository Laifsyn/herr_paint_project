use eframe::egui::{self, Color32, Label, RichText, Slider, SliderClamping};

use super::states::{SelectedItem, SliderEditor};
use crate::{Color, ShapeObject};

/// Interfaz para recibir entrada de usuario.
pub struct VaintUI {
    /// Lista de figuras ([`Shape`](crate::Shape)) dibujados en la ventana.
    pub shapes: Vec<ShapeObject>,
    /// Indica el ítem seleccionado por [VaintUI::shapes]
    pub selected_item: SelectedItem,
    /// Describe el objeto que el slider está editando el color.
    pub slider_selector: SliderEditor,
    /// Color del fondo de la pantalla.
    pub background_color: Color,
    /// Color del slider.
    pub slider_value: Color,
}

impl eframe::App for VaintUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_sliders(ui);
        });
    }
}

/// BLoque de funciones para dibujo de la interfaz de usuario.
impl VaintUI {
    /// Dibuja el recuadro del los sliders.
    fn ui_sliders(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered_justified(|ui| {
            let color = Color32::from(Color::from_u32_rgb(0x333333));
            let color_slider = move |label_text: &'static str, r#ref| {
                move |ui: &mut egui::Ui| {
                    let new_label = |text: &str| {
                        let text = RichText::new(text).color(color);
                        Label::new(text)
                    };
                    let hex_value = format!("#{:02X}", r#ref);
                    let slider = Slider::new(r#ref, 0x00..=0xFF)
                        .clamping(SliderClamping::Always)
                        .trailing_fill(true)
                        .drag_value_speed(0.75);
                    ui.add(new_label(label_text));
                    ui.add(slider);
                    ui.add(new_label(&hex_value));
                }
            };

            let [r, g, b] = &mut self.slider_value.as_mut_slice();
            ui.horizontal(color_slider("R", r));
            ui.horizontal(color_slider("G", g));
            ui.horizontal(color_slider("B", b));
        });
    }
}
