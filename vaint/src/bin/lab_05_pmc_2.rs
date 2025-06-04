#![allow(dead_code)]
use std::mem;

use glium::index::{NoIndices, PrimitiveType};
use glium::winit::application::ApplicationHandler;
use glium::{Surface as _, VertexBuffer, implement_vertex};
use glutin::surface::WindowSurface;
use utils::ApplicationContext;

pub type Point = (i32, i32);

/// Representación de un vértice en el programa.
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [i32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

/// Estructura que abstrae el manejo de I/O común para la aplicación.
pub struct App<T: ApplicationContext> {
    display: glium::Display<WindowSurface>,
    window: glium::winit::window::Window,
    /// Actual implementación principal del laboratorio.
    lab: T,
}
/// Estructura que representa el laboratorio 4.
pub struct Lab5 {
    program: Option<glium::Program>,
}

fn main() {
    tracing_subscriber::fmt().init();
    // Saltar a `impl ApplicationContext for Lab4` para ver el código principal del laboratorio.
    App::run_loop(Lab5::new());
    tracing::info!("Fin del programa. ADIÓS!");
}

fn puntos_a_vertices(puntos: Vec<Point>, color: [f32; 3]) -> Vec<Vertex> {
    puntos.into_iter().map(|(x, y)| Vertex::new([x, y], color)).collect()
}

impl ApplicationContext for Lab5 {
    const WINDOW_TITLE: &'static str = "Laboratorio 5 - Punto medio de un circulo";

    /// Método que contiene el código para renderizar un frame.
    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.941, 0.90196, 0.549019, 1.0); // Fondo Khaki

        let programa = &*self.program.get_or_insert_with(|| Self::programa(display));

        let (w, h) = target.get_dimensions();
        use glium::uniform;
        let uniforms = uniform! {
            screen_dimensions: [w, h],
            matrix : [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        };

        let parámetro_dibujo: glium::DrawParameters<'_> =
            glium::draw_parameters::DrawParameters { point_size: Some(2.0), ..Default::default() };
        const CRIMSON: [f32; 3] = [220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0];
        const PURPURA: [f32; 3] = [56.0 / 255.0, 29.0 / 255.0, 42.0 / 255.0];
        const LAZULI: [f32; 3] = [62.0 / 255.0, 105.0 / 255.0, 144.0 / 255.0];
        const LILA: [f32; 3] = [203.0 / 255.0, 186.0 / 255.0, 237.0 / 255.0];
        let puntos_circulo_a = circulo_punto_medio((150, 150), 100)
            .into_iter()
            .map(|(x, y)| if y >= 150 { Vertex::new([x, y], CRIMSON) } else { Vertex::new([x, y], PURPURA) });
        let puntos_circulo_b = circulo_punto_medio((550, 150), 100)
            .into_iter()
            .map(|(x, y)| if y >= 150 { Vertex::new([x, y], LAZULI) } else { Vertex::new([x, y], LILA) })
            .chain(puntos_circulo_a);
        let puntos_circulo_c = circulo_punto_medio((950, 150), 100)
            .into_iter()
            .map(|(x, y)| if y >= 150 { Vertex::new([x, y], PURPURA) } else { Vertex::new([x, y], LAZULI) })
            .chain(puntos_circulo_b);

        let circulos: Vec<Vertex> = puntos_circulo_c.collect();
        let vertex_buffer = VertexBuffer::new(display, &circulos).unwrap();

        target.draw(&vertex_buffer, NoIndices(PrimitiveType::Points), programa, &uniforms, &parámetro_dibujo).unwrap();

        // Finalizamos el dibujo, y flusheamos el buffer para que se vea en pantalla
        target.finish().unwrap(); // `.unwrap()` porque el flushing puede fallar
        tracing::info!("Fin del dibujo.");
    }
}

pub struct Circulo(Point);
impl Circulo {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self { Self((0, 0)) }

    pub fn x(&self) -> i32 { self.0.0 }

    pub fn y(&self) -> i32 { self.0.1 }

    pub fn set_coords(&mut self, p: Point) { self.0 = p; }

    pub fn x_mut(&mut self) -> &mut i32 { &mut self.0.0 }

    pub fn y_mut(&mut self) -> &mut i32 { &mut self.0.1 }
}

/// Obtiene los puntos que forman la circunferencia de un círculo
fn circulo_punto_medio(centro: Point, r: i32) -> Vec<Point> {
    let mut puntos = Vec::new();
    let (cx, cy) = centro;
    let mut c = Circulo::new();
    c.set_coords((0, r));
    let mut d = 1 - r;

    let mut plot_circle = |c: &Circulo| {
        // Se insertan los puntos iniciales para cada octante
        puntos.extend([
            (cx + c.x(), cy + c.y()),
            (cx - c.x(), cy + c.y()),
            (cx + 2 * r + c.x(), cy - c.y()),
            (cx + 2 * r - c.x(), cy - c.y()),
            (cx + c.y(), cy + c.x()),
            (cx - c.y(), cy + c.x()),
            (cx + 2 * r + c.y(), cy - c.x()),
            (cx + 2 * r - c.y(), cy - c.x()),
        ]);
    };

    plot_circle(&c);
    let mut circulo = c;
    while circulo.x() < circulo.y() {
        *circulo.x_mut() += 1;
        if d < 0 {
            d += 2 * circulo.x() + 1;
        } else {
            *circulo.y_mut() -= 1;
            d += 2 * (circulo.x() - circulo.y()) + 1;
        }
        plot_circle(&circulo);
    }

    puntos
}

/// Algoritmo de Bresenham para dibujar líneas sin usar multiplicación de flotantes.
fn bresenham(p0: Point, p1: Point) -> Vec<Point> {
    tracing::trace!("Bresenhan: ({x0}:{y0}) ({x1},{y1})", x0 = p0.0, y0 = p0.1, x1 = p1.0, y1 = p1.1);
    let (delta_x, delta_y) = ((p1.0 - p0.0), p1.1 - p0.1);
    tracing::trace!("Delta: (|{delta_x}| >= |{delta_y}|)", delta_x = delta_x, delta_y = delta_y);
    // Usamos la implementación que nos dará la mayor cantidad de pasos en renderizado
    if (delta_y).abs() <= (delta_x).abs() { h_bressenham(p0, p1) } else { v_bressenham(p1, p0) }
}

fn v_bressenham((mut x0, mut y0): Point, (mut x1, mut y1): Point) -> Vec<Point> {
    if y1 < y0 {
        mem::swap(&mut y0, &mut y1);
        mem::swap(&mut x0, &mut x1);
    }

    let delta_y: i32 = y1 - y0;
    let delta_x: i32 = x1 - x0;

    let (x0, y0, _x1, y1, delta_x, delta_y, x_increment): (i32, i32, i32, i32, i32, i32, i32) =
        (x0, y0, x1, y1, delta_x.abs(), delta_y.abs(), if x0 > x1 { -1 } else { 1 });

    let mut p: i32 = 2 * delta_x - delta_y;
    let two_dx: i32 = 2 * delta_x;
    let two_dx_dy: i32 = 2 * (delta_x - delta_y);

    let mut points: Vec<Point> = Vec::new();

    // Coordenadas inicial de la línea

    // let mut y: i32 = i32::min(y0, y1);
    let mut x = x0;
    let y1: i32 = i32::max(y0, y1);

    points.push((x, y0));
    for y in (y0 + 1)..=y1 {
        if p < 0 {
            p += two_dx;
        } else {
            x += x_increment;
            p += two_dx_dy;
        }
        points.push((x, y));
    }
    points
}

fn h_bressenham((mut x0, mut y0): Point, (mut x1, mut y1): Point) -> Vec<Point> {
    if x1 < x0 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    let delta_x: i32 = x1 - x0;
    let delta_y: i32 = y1 - y0;

    let (x0, y0, x1, _y1, delta_x, delta_y, y_increment) =
        (x0, y0, x1, y1, delta_x.abs(), delta_y.abs(), if y0 < y1 { 1 } else { -1 });

    let mut p: i32 = 2 * delta_y - delta_x;
    let two_dy: i32 = 2 * delta_y;
    let two_dy_dx: i32 = 2 * (delta_y - delta_x);

    let mut points: Vec<Point> = Vec::new();

    // Coordenadas inicial de la línea

    // let mut y: i32 = i32::min(y0, y1);
    let mut y = y0;
    let x1: i32 = i32::max(x0, x1);
    points.push((x0, y));
    for x in (x0 + 1)..=x1 {
        if p < 0 {
            p += two_dy;
        } else {
            y += y_increment;
            p += two_dy_dx;
        }
        points.push((x, y));
    }
    points
}

pub fn dda((x_0, y_0): (i32, i32), (x, y): (i32, i32)) -> Vec<Point> {
    let delta_x = x - x_0;
    let delta_y = y - y_0;

    let steps = i32::max(delta_x.abs(), delta_y.abs());

    let dx = (delta_x as f32) / steps as f32;
    let dy = (delta_y as f32) / steps as f32;

    let steps = steps + 1; // Se agrega un paso adicional para incluir el último punto
    let mut puntos = Vec::with_capacity(steps as usize);
    for k in 0..steps {
        let k: f32 = k as f32;
        let x = x_0 + (dx * k).round() as i32; // Type-Casting a entero trunca los decimales.
        let y = y_0 + (dy * k).round() as i32;
        puntos.push((x, y));
    }
    puntos
}

impl Lab5 {
    pub fn new() -> Self { Self { program: None } }
}

impl<T> App<T>
where
    T: ApplicationContext,
{
    pub fn run_loop(lab: T) {
        let event_loop = glium::winit::event_loop::EventLoop::new().unwrap();

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(T::WINDOW_TITLE)
            .with_inner_size(800, 600)
            .build(&event_loop);
        let mut this = Self { display, window, lab };
        event_loop.run_app(&mut this).unwrap();
    }
}

mod utils {
    use glium::Display;
    use glutin::surface::WindowSurface;

    pub trait ApplicationContext {
        const WINDOW_TITLE: &'static str;
        /// Método que contiene el código para renderizar un frame.
        fn draw_frame(&mut self, _display: &Display<WindowSurface>) {}
        fn handle_window_event(&mut self, _event: &glium::winit::event::WindowEvent, _window: &glium::winit::window::Window) {}
        fn programa(display: &Display<WindowSurface>) -> glium::Program {
            use glium::program;
            program!(display,
                330 => {  // GLSL 330 (modern OpenGL on Windows)
                    vertex: "
                    #version 330
        
                    in vec2 position;  // Coordenadas XY representadas con Vec2
                    in vec3 color;     // RGB representado con Vec3 
                    uniform uvec2 screen_dimensions;
        
                    out vec3 vColor;
        
                    void main() {
                        vec2 flipped_position = vec2(position.x, screen_dimensions.y - position.y);
                        vec2 normalized_device_coords = (flipped_position / vec2(screen_dimensions)) * 2.0 - 1.0;
                        gl_Position = vec4(normalized_device_coords, 0.0, 1.0); // 2D → 4D clip space
                        vColor = color;
                    }
                ",

                    fragment: "
                    #version 330
                    in vec3 vColor; // RGB representado con Vec3 
                    out vec4 frag_color;
        
                    void main() {
                        frag_color = vec4(vColor, 1.0); // Alpha = 1.0 (Sin transparencia)
                    }
                ",
                },
            )
            .unwrap()
        }
    }
}

impl Default for Lab5 {
    fn default() -> Self { Self::new() }
}

impl<T: ApplicationContext> ApplicationHandler for App<T> {
    /// Emitted when the application has been resumed.
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        // Hacer nada
        tracing::debug!("Application resumed!");
    }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            glium::winit::event::WindowEvent::Resized(new_size) => {
                self.display.resize(new_size.into());
            }
            glium::winit::event::WindowEvent::RedrawRequested => {
                self.lab.draw_frame(&self.display);
            }
            // Manejar eventos de cierre de ventana
            glium::winit::event::WindowEvent::CloseRequested
            | glium::winit::event::WindowEvent::KeyboardInput {
                event:
                    glium::winit::event::KeyEvent {
                        state: glium::winit::event::ElementState::Pressed,
                        logical_key: glium::winit::keyboard::Key::Named(glium::winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            e => {
                self.lab.handle_window_event(&e, &self.window);
            }
        }
    }
}

impl Vertex {
    pub fn new(position: [i32; 2], color: [f32; 3]) -> Self { Vertex { position, color } }

    pub fn new_red(position: [i32; 2]) -> Self { Vertex::new(position, [1.0, 0.0, 0.0]) }

    pub fn new_green(position: [i32; 2]) -> Self { Vertex::new(position, [0.0, 1.0, 0.0]) }

    pub fn new_blue(position: [i32; 2]) -> Self { Vertex::new(position, [0.0, 0.0, 1.0]) }
}

/// Codigo de Prueba para el test de bressenham
#[cfg(test)]
mod test {
    use core::panic;
    use std::sync::LazyLock;

    use crate::{Point, bresenham, dda};

    static TRACING: LazyLock<()> = LazyLock::new(|| {
        let _ = tracing_subscriber::fmt().without_time().with_file(true).with_line_number(true).try_init().ok();
    });

    #[test]
    fn bress_360deg() {
        *TRACING;
        let p0 = (10, 0);
        let p1 = (219, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_15deg() {
        *TRACING;
        let p0 = (10, 3);
        let p1 = (208, 56);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_45deg() {
        *TRACING;
        let p0 = (10, 0);
        let p1 = (210, 200);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_60deg() {
        *TRACING;
        let p0 = (0, -173);
        let p1 = (100, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_90deg() {
        *TRACING;
        let p0 = (0, 0);
        let p1 = (0, -200);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_120deg() {
        *TRACING;
        let p0 = (100, 0);
        let p1 = (0, -173);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_165deg() {
        *TRACING;
        let p0 = (208, 3);
        let p1 = (10, 56);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_180deg() {
        *TRACING;
        let p0 = (219, 555);
        let p1 = (10, 555);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_195deg() {
        *TRACING;
        let p0 = (208, 3);
        let p1 = (10, 56);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_225deg() {
        *TRACING;
        let p0 = (210, 200);
        let p1 = (10, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_240deg() {
        *TRACING;
        let p0 = (100, 173);
        let p1 = (0, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_270deg() {
        *TRACING;
        let p0 = (0, 200);
        let p1 = (0, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_300deg() {
        *TRACING;
        let p0 = (0, 173);
        let p1 = (100, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_315deg() {
        *TRACING;
        let p0 = (10, 200);
        let p1 = (210, 0);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }

    #[test]
    fn bress_345deg() {
        *TRACING;
        let p0 = (10, 56);
        let p1 = (208, 3);
        let b_points = bresenham(p0, p1);
        let d_points = dda(p0, p1);
        compare(b_points, d_points);
    }
    #[track_caller]
    fn compare(mut a_points: Vec<Point>, mut b_points: Vec<Point>) {
        let (b, a) = (b_points.last().unwrap(), b_points.first().unwrap());
        let (x, y) = (b.0 - a.0, b.1 - a.1);
        tracing::debug!(
            "Componentes: x:{x:?} y:{y:?}. Angle: {alpha:.2} deg",
            alpha = f64::atan(y as f64 / x as f64).to_degrees()
        );
        a_points.sort();
        b_points.sort();

        // a_points.sort_by_key(|&(a, b)| (a));
        // b_points.sort_by_key(|&(a, b)| (a));
        assert_eq!(a_points.len(), b_points.len(), "Los dos arreglos no tienen la misma longitud");
        let longest = a_points.len().max(b_points.len());
        let mut errors = 0;
        let mut report = Vec::new();
        for idx in 0..longest {
            let i = a_points.get(idx);
            let j = b_points.get(idx);
            if i != j {
                report.push(Err(format!("Se esperaba que los puntos fueran iguales: {:?} != {:?}", i, j)));
                errors += 1;
            } else {
                report.push(Ok(format!("Los puntos son iguales: {:?} ", j)));
            }
        }
        if errors > longest * 5 / 100 {
            for r in report.into_iter() {
                match r {
                    Ok(ok) => tracing::info!("{}", ok),
                    Err(err) => tracing::error!("{}", err),
                }
            }
            panic!("Los puntos no son iguales");
        }
    }
}
