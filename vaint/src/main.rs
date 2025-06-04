use std::fs;

use eframe::{App, Frame, egui};
use vaint::{Color, Figura};

#[derive(PartialEq)]
enum ColorObjetivo {
    Borde,
    Relleno,
    Fondo,
}

struct MiApp {
    stroke_color: Color,
    shape_background: Color,
    /// Fondo de la pantalla de glium.
    screen_background: Color,
    grosor: f32,
    objetivo_color: ColorObjetivo,
    cuadrado: u32,
    centro_cuadrado: (i32, i32),
    largo_rectangulo: u32,
    ancho_rectangulo: u32,
    centro_rectangulo: (i32, i32),
    radio1_elipse: u32,
    radio2_elipse: u32,
    centro_elipse: (i32, i32),
    radio_circulo: u32,
    centro_circulo: (i32, i32),
    figuras_seleccionadas: Vec<Figura>,
}

impl Default for MiApp {
    fn default() -> Self {
        Self {
            stroke_color: Color::from_u32_rgb(0x333333),
            grosor: 5.0,
            objetivo_color: ColorObjetivo::Borde,
            cuadrado: 50,
            centro_cuadrado: (300, 300),
            largo_rectangulo: 60,
            ancho_rectangulo: 30,
            centro_rectangulo: (300, 300),
            radio1_elipse: 20,
            radio2_elipse: 30,
            centro_elipse: (300, 300),
            radio_circulo: 50,
            centro_circulo: (300, 300),
            figuras_seleccionadas: vec![],
            shape_background: Color::from_u32_rgb(0xffffff),
            screen_background: Color::from_u32_rgb(0xffffff),
        }
    }
}

impl App for MiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let [r, g, b] = match &self.objetivo_color {
                ColorObjetivo::Borde => &mut self.stroke_color,
                ColorObjetivo::Relleno => &mut self.shape_background,
                ColorObjetivo::Fondo => &mut self.screen_background,
            }
            .as_mut_slice();
            ui.horizontal(|ui| {
                ui.label("R:");
                ui.add(egui::Slider::new(r, 0..=u8::MAX));
            });
            ui.horizontal(|ui| {
                ui.label("G:");
                ui.add(egui::Slider::new(g, 0..=u8::MAX));
            });
            ui.horizontal(|ui| {
                ui.label("B:");
                ui.add(egui::Slider::new(b, 0..=u8::MAX));
            });
            let color = egui::Color32::from_rgb(*r, *g, *b);

            ui.add(egui::Button::new("                   ").fill(color));

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Borde, "Borde");
                ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Relleno, "Relleno");
                ui.radio_value(&mut self.objetivo_color, ColorObjetivo::Fondo, "Fondo de Pantalla");
            });

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
                                ui.add(egui::DragValue::new(&mut self.radio_circulo));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_circulo.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_circulo.1));
                            });
                        }
                        Figura::Cuadrado => {
                            ui.horizontal(|ui| {
                                ui.label("Lado (Cuadrado):");
                                ui.add(egui::DragValue::new(&mut self.cuadrado));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_cuadrado.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_cuadrado.1));
                            });
                        }
                        Figura::Rectangulo => {
                            ui.horizontal(|ui| {
                                ui.label("Largo (Rectángulo):");
                                ui.add(egui::DragValue::new(&mut self.largo_rectangulo));
                                ui.label("Ancho:");
                                ui.add(egui::DragValue::new(&mut self.ancho_rectangulo));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_rectangulo.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_rectangulo.1));
                            });
                        }
                        Figura::Elipse => {
                            ui.horizontal(|ui| {
                                ui.label("Radio 1 (Elipse):");
                                ui.add(egui::DragValue::new(&mut self.radio1_elipse));
                                ui.label("Radio 2:");
                                ui.add(egui::DragValue::new(&mut self.radio2_elipse));
                                ui.label("Centro X:");
                                ui.add(egui::DragValue::new(&mut self.centro_elipse.0));
                                ui.label("Centro Y:");
                                ui.add(egui::DragValue::new(&mut self.centro_elipse.1));
                            });
                        }
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

            if ui.button("🗑️ Iniciar Tablero").clicked() {
                // Guarda la configuración actual
                let config = vaint::Config {
                    stroke_color: self.stroke_color,
                    figuras: self.figuras_seleccionadas.clone(),
                    grosor: self.grosor,
                    cuadrado: self.cuadrado,
                    centro_cuadrado: self.centro_cuadrado,
                    largo_rectangulo: self.largo_rectangulo,
                    ancho_rectangulo: self.ancho_rectangulo,
                    centro_rectangulo: self.centro_rectangulo,
                    radio1_elipse: self.radio1_elipse,
                    radio2_elipse: self.radio2_elipse,
                    centro_elipse: self.centro_elipse,
                    radio_circulo: self.radio_circulo,
                    centro_circulo: self.centro_circulo,
                    shape_background_color: self.shape_background,
                    background_color: self.screen_background,
                };
                fs::write("config.json", serde_json::to_string(&config).unwrap()).unwrap();

                println!("Lanzando ventana OpenGL como proceso externo...");
                // la aplicación de OpenGL y Eframe ambos hacen uso de winit::event_loop::EventLoop, por lo que no
                // es posible ejecutar la ventana de OpenGL en otro hilo.
                //
                // https://docs.rs/winit/latest/winit/event_loop/struct.EventLoopBuilder.html#method.build
                let _ = std::process::Command::new("opengl_app.exe").spawn();
            }
        });
    }
}

fn main() {
    vaint::tracing::init();
    tracing::info!("Vaint OpenGL App is running...");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([495.0, 405.0]).with_position(egui::pos2(700.0, 350.0)),
        ..Default::default()
    };
    let _: Result<(), eframe::Error> =
        eframe::run_native("Selector de Color", options, Box::new(|_cc| Ok(Box::new(MiApp::default()))));
}
