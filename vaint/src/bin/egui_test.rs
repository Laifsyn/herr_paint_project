use eframe::{egui, App, Frame};
#[derive(PartialEq)]
enum ColorObjetivo {
    Borde,
    Relleno,
    Fondo,
}

#[derive(Debug, PartialEq, Clone)]
enum Figura {
    Circulo,
    Rectangulo,
     Triangulo,
    Elipse,
    Linea,
}
struct MiApp {
    color: [f32; 3],
    grosor: f32,
    figura:Figura,
    objetivo_color:ColorObjetivo,
    transparente: bool,

    
}

#[derive(Default)]
struct Ventana {}

impl Default for MiApp {
    fn default() -> Self {
        Self { color: [0.0, 0.0, 0.0],
            grosor: 5.0,
            figura: Figura::Circulo,
            objetivo_color: ColorObjetivo::Borde,
            transparente: false,
        }

    }
}

impl App for MiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("R:");
                ui.add(egui::Slider::new(&mut self.color[0], 0.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("G:");
                ui.add(egui::Slider::new(&mut self.color[1], 0.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("B:");
                ui.add(egui::Slider::new(&mut self.color[2], 0.0..=1.0));
            });
            let color = egui::Color32::from_rgb(
                (self.color[0] * 255.0) as u8,
                (self.color[1] * 255.0) as u8,
                (self.color[2] * 255.0) as u8,
            );

             if ui.add(egui::Button::new("                   ").fill(color)).clicked() {
            }

            ui.horizontal(|ui| {
    ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Borde, "Borde");
    ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Relleno, "Relleno");
    ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Fondo, "Fondo de Pantalla");
    });

    // Checkbox de transparencia
    ui.checkbox(&mut self.transparente, "¿Transparente?");

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Grosor:");
                ui.add(egui::Slider::new(&mut self.grosor, 1.0..=50.0));
            });
            ui.group(|ui| {
    ui.label("🟦 Figura seleccionada:");

    ui.horizontal(|ui| {
        if ui.button("🔵 Círculo").clicked() {
            self.figura = Figura::Circulo;
        }
        if ui.button("▭ Rectángulo").clicked() {
            self.figura = Figura::Rectangulo;
        }
        if ui.button("➖ Línea").clicked() {
            self.figura = Figura::Linea;
        }
        if ui.button("🔺 Triángulo").clicked() {
            self.figura = Figura::Triangulo;
        }
        if ui.button("🟡 Elipse").clicked() {
            self.figura = Figura::Elipse;
        }
    });

    ui.label(format!("Figura actual: {:?}", self.figura));
});




    if ui.button("🗑️ Borrar tablero").clicked() {
    // acción temporal (aún no implementada)
println!("Se presionó el botón de borrar.");


let color = if self.transparente {
    egui::Color32::from_rgba_unmultiplied(
        (self.color[0] * 255.0) as u8,
        (self.color[1] * 255.0) as u8,
        (self.color[2] * 255.0) as u8,
        0, // completamente transparente
    )
} else {
    egui::Color32::from_rgb(
        (self.color[0] * 255.0) as u8,
        (self.color[1] * 255.0) as u8,
        (self.color[2] * 255.0) as u8,
    )
};


}
            
           
            
        });
    }
}

impl Ventana{
    fn opciones(_cc: &eframe::CreationContext<'_>) -> Self{
        Self::default()
    }
}

impl eframe::App for Ventana {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 100.0]),
        ..Default::default()
    };
    let _: Result<(), eframe::Error> =eframe::run_native(
        "Selector de Color",
        options,
        Box::new(|_cc| Ok(Box::new(MiApp::default()))),
    );
} 