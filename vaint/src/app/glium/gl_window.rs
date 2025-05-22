use std::cell::RefCell;
use std::rc::Rc;

use glium::index::{NoIndices, PrimitiveType};
use glium::winit::application::ApplicationHandler;
use glium::{Display, DrawParameters, Surface, uniform};
use glutin::surface::WindowSurface;

use super::GlShapeList;
use crate::algorithms::flood_fill;
use crate::{Color, PixelCoord, Shape, ShapeObject, ShapeStyle, Vertex};

pub struct GlWindow {
    pub program: Option<glium::Program>,
    pub display: glium::Display<WindowSurface>,
    pub window: glium::winit::window::Window,
    /// Lista de Objetos a dibujar
    pub shapes_list: Rc<RefCell<GlShapeList>>,
    pub background_color: Color,
}

impl GlWindow {
    pub const WINDOW_TITLE: &'static str = "Vaint - OpenGL Windows";

    fn draw_frame(&mut self) {
        let display = &mut self.display;
        let program = self.program.get_or_insert_with(|| programa(display));

        let mut target = display.draw();
        // Colorear el fondo de la ventana
        let [red, green, blue] = self.background_color.to_vec();
        target.clear_color(red, green, blue, 1.0);

        let (screen_width, screen_height) = target.get_dimensions();
        let uniforms = uniform! {
            screen_dimensions: [screen_width, screen_height],
            matrix : [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        };

        let outline_points = Self::generate_shapes(&self.shapes_list.borrow(), (screen_width as i32, screen_height as i32));

        // Dibujar el relleno de las figuras
        for (outlines, style) in outline_points.iter().filter(|(_, s)| s.fill_color.is_some()) {
            let Some(color) = style.fill_color else { unreachable!("El color de relleno es Some(_)") };

            let mut fill_points = Vec::new();
            flood_fill(outlines, &mut fill_points);

            if fill_points.is_empty() {
                tracing::warn!("No hay puntos para rellenar la figura");
                continue;
            }

            let drawing_params = DrawParameters { ..Default::default() };
            let fill_points =
                fill_points.iter().map(|(x, y)| Vertex { position: [*x, *y], color: color.to_vec() }).collect::<Vec<_>>();
            let vertex_buffer = glium::VertexBuffer::new(display, &fill_points).unwrap();
            target.draw(&vertex_buffer, NoIndices(PrimitiveType::Points), program, &uniforms, &drawing_params).unwrap();
        }

        // Dibujar el contorno de las figuras
        for (vertices, style) in outline_points.iter() {
            let drawing_params = DrawParameters { point_size: Some(style.stroke_width), ..Default::default() };
            let vertices = vertices
                .iter()
                .map(|(x, y)| Vertex { position: [*x, *y], color: style.stroke_color.unwrap_or(Color::BLACK).to_vec() })
                .collect::<Vec<_>>();
            let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
            target.draw(&vertex_buffer, NoIndices(PrimitiveType::Points), program, &uniforms, &drawing_params).unwrap();
        }
        target.finish().unwrap();
    }

    /// Genera los puntos con color ([`Vertex`]) de cada figura a dibujar, y devuelve el grosor de
    /// cada figura.
    fn generate_shapes(shapes_list: &[ShapeObject], screen_dimensions: PixelCoord) -> Vec<(Vec<PixelCoord>, ShapeStyle)> {
        let (screen_width, screen_height) = screen_dimensions;
        // Filtra los puntos que están fuera de la pantalla.
        let filter_inbounds = |(x, y): &PixelCoord| -> bool { (0..screen_width).contains(x) && (0..screen_height).contains(y) };
        let mut vertices: Vec<(Vec<PixelCoord>, ShapeStyle)> = Vec::with_capacity(shapes_list.len());

        for shape in shapes_list
            .iter()
            // Incluir solo figuras con color de borde definido.
            .filter(|obj| obj.style().stroke_color.is_some())
        {
            let mut points: Vec<PixelCoord> = Vec::new();
            shape.write_outline_points(&mut points);
            let style = shape.style();
            let iter_vertices = points.into_iter().filter(filter_inbounds).collect();
            vertices.push((iter_vertices, *style));
        }
        vertices
    }

    fn handle_window_event(&mut self, _event: &glium::winit::event::WindowEvent) {
        let _window = &self.window;
        tracing::debug!("Do nothing....");
    }
}

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

impl ApplicationHandler for GlWindow {
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
                self.draw_frame();
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
                self.handle_window_event(&e);
            }
        }
    }
}
