#![allow(unused)]
use glium::index::{NoIndices, PrimitiveType};
use glium::winit::application::ApplicationHandler;
use glium::winit::event_loop::EventLoop;
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

pub struct App<T: ApplicationContext> {
    display: glium::Display<WindowSurface>,
    window: glium::winit::window::Window,
    lab: T,
}
pub struct Lab4 {
    program: Option<glium::Program>,
}
fn main() { App::run_loop(Lab4::new()); }

impl ApplicationContext for Lab4 {
    const WINDOW_TITLE: &'static str = "Lab 4";

    /// Método que contiene el código para renderizar un frame.
    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0); // Fondo Negro

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
            glium::draw_parameters::DrawParameters { line_width: Some(1.0), ..Default::default() };
        let buff = vec![
            Vertex { position: [0, 0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [2, 200], color: [0.0, 0.0, 0.1] },
            Vertex { position: [30, 5], color: [1.0, 0.6, 0.1] },
        ];
        let rect = VertexBuffer::new(display, &buff).unwrap();
        target.draw(&rect, NoIndices(PrimitiveType::LineLoop), programa, &uniforms, &parámetro_dibujo).unwrap();

        // Finalizamos el dibujo, y flusheamos el buffer para que se vea en pantalla
        target.finish().unwrap(); // `.unwrap()` porque el flushing puede fallar
    }
}

impl Lab4 {
    fn bresenham(p0: Point, p1: Point) -> Vec<Point> {
        let mut points = Vec::new();
        let (x0, y0) = p0;
        let (x1, y1) = p1;

        let dx = x1 - x0;
        let dy = y1 - y0;
        let sx = if dx < 0 { -1 } else { 1 };
        let sy = if dy < 0 { -1 } else { 1 };
        let dx = dx.abs();
        let dy = dy.abs();

        if dx > dy {
            let mut err = dx / 2;
            let mut y = y0;
            for x in (x0..=x1).step_by(sx as usize) {
                points.push((x, y));
                err -= dy;
                if err < 0 {
                    y += sy;
                    err += dx;
                }
            }
        } else {
            let mut err = dy / 2;
            let mut x = x0;
            for y in (y0..=y1).step_by(sy as usize) {
                points.push((x, y));
                err -= dx;
                if err < 0 {
                    x += sx;
                    err += dy;
                }
            }
        }

        points
    }

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

impl Default for Lab4 {
    fn default() -> Self { Self::new() }
}

impl<T: ApplicationContext> ApplicationHandler for App<T> {
    /// Emitted when the application has been resumed.
    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        // Hacer nada
        tracing::debug!("Application resumed!");
    }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            glium::winit::event::WindowEvent::Resized(new_size) => {
                self.display.resize(new_size.into());
            }
            glium::winit::event::WindowEvent::RedrawRequested => {
                self.lab.draw_frame(&self.display);
            }
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
