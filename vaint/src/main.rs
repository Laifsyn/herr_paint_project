#[macro_use]
extern crate glium;
mod support;

use glium::{Display, Surface, index::PrimitiveType};
use glutin::surface::WindowSurface;
use support::{ApplicationContext, State};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct Application {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub program: glium::Program,
}

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];
impl ApplicationContext for Application {
    const WINDOW_TITLE: &'static str = "Glium triangle example";

    fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = {
            glium::VertexBuffer::new(display, &[
                Vertex { position: [-0.5, -0.5], color: BLACK },
                Vertex { position: [0.0, 0.5], color: [0.25, 0.68, 0.24] },
                Vertex { position: [0.5, -0.5], color: BLACK },
                Vertex { position: [-0.5, -0.5], color: [1.0, 1.0, 1.0] },
                Vertex { position: [0.0, 0.5], color: [0.5, 0.5, 0.5] },
                Vertex { position: [-1.0, 0.0], color: BLACK },
                // Vertex { position: [0.0, 0.5], color: [0.5, 0.5, 0.24] },
            ])
            .unwrap()
        };

        // building the index buffer
        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::LineStrip, &[0u16, 1, 2]).unwrap();

        // compiling shaders and linking them together
        let program = program!(display,
            100 => {
                vertex: "
                    #version 100

                    uniform lowp mat4 matrix;

                    attribute lowp vec2 position;
                    attribute lowp vec3 color;

                    varying lowp vec3 vColor;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 100
                    varying lowp vec3 vColor;

                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },
        )
        .unwrap();

        Self { vertex_buffer, index_buffer, program }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        // Obtener el objeto para dibujar a la pantalla
        let mut frame: glium::Frame = display.draw();

        // For this example a simple identity matrix suffices
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };
        // let uniforms = EmptyUniforms;

        // Now we can draw the triangle
        frame.clear_color(1.0, 1.0, 1.0, 0.0); // Fondo Blanco
        frame.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();
    }
}

fn main() { State::<Application>::run_loop(); }
