pub use vaint_ui::VaintUI;

mod shape;
mod states;
mod vaint_ui;

pub fn start_app() -> Result<(), eframe::Error> {
    use crate::Color;
    let app = VaintUI {
        shapes: Vec::new(),
        selected_item: states::SelectedItem::None,
        background_color: Color::from_rgb(125, 124, 124),
        slider_selector: states::SliderEditor::Background,
        slider_value: Color::from_u32_rgb(0xC3B091),
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Vaint User Command Interface", native_options, Box::new(|_cc| Ok(Box::new(app))))
}
