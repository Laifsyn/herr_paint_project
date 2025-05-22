// use eframe::Result;
// use vaint::egui_app;

fn main() {
    vaint::tracing::init();
    vaint::glium_app::run_loop_standalone();
}
