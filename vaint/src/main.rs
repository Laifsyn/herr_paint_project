// use eframe::Result;
// use vaint::egui_app;
use eframe::{egui, App, Frame};
mod opengl_app;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    color: [f32; 3],
    figuras: Vec<String>,
    grosor: f32,
    transparente: bool,
    cuadrado: u32,
    centro_cuadrado: (i32, i32),
    largoRectangulo: u32,
    anchoRectangulo: u32,
    centro_rectangulo: (i32, i32),
    radio1Elipse: u32,
    radio2Elipse: u32,
    centro_elipse: (i32, i32),
    radioCirculo: u32,
    centro_circulo: (i32, i32),
}
/*fn main() {
    vaint::tracing::init();
    vaint::glium_app::run_loop_standalone();
}*/

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
    Cuadrado,
}

struct MiApp {
    color: [f32; 3],
    grosor: f32,
    figura:Figura,
    objetivo_color:ColorObjetivo,
    transparente: bool,
    Cuadrado: u32,
    centro_cuadrado: (i32, i32),
    largoRectangulo: u32,
    anchoRectangulo: u32,
    centro_rectangulo: (i32, i32),
    radio1Elipse: u32,
    radio2Elipse: u32,
    centro_elipse: (i32, i32),
    radioCirculo: u32,
    centro_circulo: (i32, i32),
    figuras_seleccionadas: Vec<Figura>,
}

impl Default for MiApp {
    fn default() -> Self {
        Self { color: [0.0, 0.0, 0.0],
            grosor: 5.0,
            figura: Figura::Circulo,
            objetivo_color: ColorObjetivo::Borde,
            transparente: false,
            Cuadrado: 50,
            centro_cuadrado: (300, 300),
            largoRectangulo: 60,
            anchoRectangulo: 30,
            centro_rectangulo: (300, 300),
            radio1Elipse: 20,
            radio2Elipse: 30,
            centro_elipse: (300, 300),
            radioCirculo: 50,
            centro_circulo: (300, 300),
            figuras_seleccionadas: vec![],
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
                for figura in &self.figuras_seleccionadas {
                    match figura {
                        Figura::Circulo => {
                            ui.horizontal(|ui| {
                                ui.label("Radio (Círculo):");
                                ui.add(egui::DragValue::new(&mut self.radioCirculo));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_circulo.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_circulo.1));
                            });
                        }
                        Figura::Cuadrado => {
                            ui.horizontal(|ui| {
                                ui.label("Lado (Cuadrado):");
                                ui.add(egui::DragValue::new(&mut self.Cuadrado));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_cuadrado.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_cuadrado.1));
                            });
                        }
                        Figura::Rectangulo => {
                            ui.horizontal(|ui| {
                                ui.label("Largo (Rectángulo):");
                                ui.add(egui::DragValue::new(&mut self.largoRectangulo));
                                ui.label("Ancho:");
                                ui.add(egui::DragValue::new(&mut self.anchoRectangulo));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_rectangulo.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_rectangulo.1));
                            });
                        }
                        Figura::Elipse => {
                            ui.horizontal(|ui| {
                                ui.label("Radio 1 (Elipse):");
                                ui.add(egui::DragValue::new(&mut self.radio1Elipse));
                                ui.label("Radio 2:");
                                ui.add(egui::DragValue::new(&mut self.radio2Elipse));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_elipse.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_elipse.1));
                            });
                        }
                        _ => {}
                    }
                }
            });

            ui.group(|ui| {
                ui.label(" Figuras seleccionadas:");

                for figura in [
                    (Figura::Circulo, "🔵 Círculo"),
                    (Figura::Rectangulo, "▭ Rectángulo"),
                    (Figura::Cuadrado, "➖ Cuadrado"),
                    (Figura::Elipse, "🟡 Elipse"),
                ] {
                    let mut selected = self.figuras_seleccionadas.contains(&figura.0);
                    if ui.checkbox(&mut selected, figura.1).changed() {
                        if selected {
                            self.figuras_seleccionadas.push(figura.0.clone());
                        } else {
                            self.figuras_seleccionadas.retain(|f| f != &figura.0);
                        }
                    }
                }

                ui.label(format!("Figuras actuales: {:?}", self.figuras_seleccionadas));
            });

            ui. horizontal(|ui| {
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
                
                if ui.button("🗑️ Iniciar Tablero").clicked() {
                    // Guarda la configuración actual
                    let config = Config {
                        color: self.color,
                        figuras: self.figuras_seleccionadas.iter().map(|f| format!("{:?}", f)).collect(),
                        grosor: self.grosor,
                        transparente: self.transparente,
                        cuadrado: self.Cuadrado as u32,
                        centro_cuadrado: self.centro_cuadrado,
                        largoRectangulo: self.largoRectangulo as u32,
                        anchoRectangulo: self.anchoRectangulo as u32,
                        centro_rectangulo: self.centro_rectangulo,
                        radio1Elipse: self.radio1Elipse as u32,
                        radio2Elipse: self.radio2Elipse as u32,
                        centro_elipse: self.centro_elipse,
                        radioCirculo: self.radioCirculo as u32,
                        centro_circulo: self.centro_circulo,
                    };
                    fs::write("config.json", serde_json::to_string(&config).unwrap()).unwrap();

                    println!("Lanzando ventana OpenGL como proceso externo...");
                    let _ = std::process::Command::new("opengl_app.exe").spawn();
                }
            });
        });
    }
}


fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([495.0, 405.0])
            .with_position(egui::pos2(700.0, 350.0)),
        ..Default::default()
    };
    let _: Result<(), eframe::Error> =eframe::run_native(
        "Selector de Color",
        options,
        Box::new(|_cc| Ok(Box::new(MiApp::default()))),
    );
} 
