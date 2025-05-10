use glium::winit::event::Event;
use glium::winit::event_loop::ActiveEventLoop;
use glium::{implement_vertex, program};
use glutin::surface::WindowSurface;

/// Punto de entrada de la aplicación
fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().unwrap();

    // 2. Create a glutin context and glium Display
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(1000, 1000).build(&event_loop);

    #[allow(deprecated)]
    event_loop
        .run(move |evento: Event<()>, r#loop: &ActiveEventLoop| {
            match evento {
                Event::WindowEvent { window_id, event: win_event } => {
                    // manejamos eventos a la ventana en la función
                    ventana::procesar_eventos(r#loop, window_id, win_event, &display);
                }
                // otros eventos
                _ignorado => {}
            }
        })
        .ok();
}
/// Representación de un vértice en el programa.
#[derive(Copy, Clone)]
pub struct Vertex {
    // Las coordenadas están delimitadas en el rango [-1.0, 1.0], donde [-1.0, -1.0] es la esquina inferior izquierda y [1.0,
    // 1.0] es la esquina superior derecha.
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

/// Representa un Vector de 2 dimensiones
pub type Vec2 = [f32; 2];

///Programa principal ejecutado por el GPU
///
/// # NOTA:
///
/// No encontré forma de evitar la necesidad de proveer programa.
pub fn programa(display: &glium::Display<WindowSurface>) -> glium::Program {
    program!(display,
        330 => {  // GLSL 330 (modern OpenGL on Windows)
            vertex: "
                    #version 330
        
                    in vec2 position;  // Coordenadas XY representadas con Vec2
                    in vec3 color;    // RGB representado con Vec3 
        
                    out vec3 vColor;
        
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0); // 2D → 4D clip space
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

/// Traduce la posición de coordenadas absolutas a coordenadas de OpenGl -1.0..=1.0.
pub fn transformar_coordenadas<const N: usize>(coordenadas: (u32, u32)) -> [f32; 2] {
    let (x, y) = coordenadas;
    let x = x as f32 / N as f32 * 2.0 - 1.0;
    let y = y as f32 / N as f32 * -2.0 + 1.0; // Invertir el eje Y
    [x, y]
}

/// Macro para definir un set de [`Vertex`]
macro_rules! crear_figura {
    // Capture: vec, color, first point, then one or more additional points
    ($vec:expr, $display:expr, $color:expr,
     ($x0:expr, $y0:expr)
     $(, ($x:expr, $y:expr))+
     $(,)?
    ) => {{
        use self::transformar_coordenadas;
        let pos = |x: u32, y: u32| transformar_coordenadas::<1000>((x, y));

        // Build a fixed-size array of points, repeating the first at the end:
        let shape = [
            // pos(10, 10),
            pos($x0, $y0),
            $(
                pos($x, $y),
            )+
            pos($x0, $y0),
        ];
        use self::colorear_coordenadas as colorear;
        $vec.clear();
        $vec.extend(colorear(&shape, $color));
        let result : glium::VertexBuffer<crate::Vertex> = glium::VertexBuffer::new($display, &$vec).unwrap();
        let index_buffer: glium::IndexBuffer<u16> = {
            let indices: Vec<u16> = (0..$vec.len() as u16).collect();
            glium::IndexBuffer::new($display, glium::index::PrimitiveType::LineStrip, &indices).unwrap()
        };
        (result, index_buffer)

    }};
}

/// Iterador que devuelve objetos tipo [`Vertex`] que es usado para ubicar puntos en el canvas
pub fn colorear_coordenadas(coordenadas: &[[f32; 2]], color: [f32; 3]) -> impl Iterator<Item = Vertex> + '_ {
    coordenadas.iter().map(move |&pos| Vertex { position: pos, color })
}

/// Contiene la logica sobre el manejo de los eventos a la ventana y la aplicacion durante ejecución
mod ventana {
    use glium::winit::event::{ElementState, KeyEvent, WindowEvent};
    use glium::winit::event_loop::ActiveEventLoop;
    use glium::winit::keyboard::NamedKey;
    use glium::winit::window::WindowId;
    use glium::{Surface, uniform};
    use glutin::surface::WindowSurface;

    use super::{colorear_coordenadas, transformar_coordenadas};

    /// Codigo para controlar lo que se dibuja en la ventana
    fn draw_frame(display: &glium::Display<glutin::surface::WindowSurface>) {
        // Obtener el objeto dibujable a la pantalla
        let mut target: glium::Frame = display.draw();

        // Limpiar la pantalla
        target.clear_color(0.0, 0.0, 0.0, 1.0); // Blanco

        // # Inicializacion
        // For this example a simple identity matrix suffices
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };
        let programa = crate::programa(display);
        let mut figura_vertices = Vec::new();

        // Figura 1 - Crear un rectangulo vertical verde
        let (rect, index_buffer) =
            crear_figura!(&mut figura_vertices, display, [0.0, 1.0, 0.0], (33, 60), (233, 60), (233, 180), (33, 180));
        target.draw(&rect, &index_buffer, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 2 - Rectángulo acostado Rojo
        let (fig, idxb) =
            crear_figura!(&mut figura_vertices, display, [1.0, 0.0, 0.0], (260, 30), (380, 30), (380, 210), (260, 210));
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 3 - Rombo
        let (fig, idxb) =
            crear_figura!(&mut figura_vertices, display, [0.0, 0.0, 1.0], (496, 33), (583, 120), (496, 208), (408, 120),);
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 4 - Trapecio
        let (fig, idxb) =
            crear_figura!(&mut figura_vertices, display, [1.0, 1.0, 0.0], (610, 30), (790, 30), (746, 210), (656, 210));
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 5 - Triángulo
        let (fig, idxb) = crear_figura!(&mut figura_vertices, display, [1.0, 0.5, 0.0], (818, 195), (893, 45), (968, 195));
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 6 - Triangulo Rectangulo
        let (fig, idxb) = crear_figura!(&mut figura_vertices, display, [0.0, 1.0, 1.0], (33, 341), (183, 490), (33, 490));
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 7 - Hexágono
        let (fig, idxb) = crear_figura!(
            &mut figura_vertices,
            display,
            [1.0, 0.0, 1.0],
            (213, 416),
            (255, 342),
            (339, 342),
            (382, 416),
            (339, 489),
            (255, 489)
        );
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 8 - Pentágono
        let (fig, idxb) = crear_figura!(
            &mut figura_vertices,
            display,
            [0.0, 0.5, 0.0],
            (411, 398),
            (492, 339),
            (572, 398),
            (541, 492),
            (442, 492),
        );
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 9 - Poligono +
        let (fig, idxb) = crear_figura!(
            &mut figura_vertices,
            display,
            [0.5, 0.8, 1.0],
            (602, 356),
            (642, 356),
            (642, 316),
            (762, 316),
            (762, 356),
            (802, 356),
            (802, 476),
            (762, 476),
            (762, 516),
            (642, 516),
            (642, 476),
            (602, 476),
        );
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        // Figura 10 - Extrella
        let (fig, idxb) = crear_figura!(
            &mut figura_vertices,
            display,
            [0.8, 0.8, 0.8],
            (831, 398),
            (892, 398),
            (912, 339),
            (930, 398),
            (992, 398),
            (942, 434),
            (961, 492),
            (912, 456),
            (862, 492),
            (881, 434),
        );
        target.draw(&fig, &idxb, &programa, &uniforms, &Default::default()).unwrap();

        target.finish().unwrap();
    }

    /// Eventos principales para el manejo de la ventana. Hace uso de [`draw_frame`] cuando se
    /// requiere re-renderizar la ventana.
    pub fn procesar_eventos(
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
        display: &glium::Display<WindowSurface>,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                display.resize(new_size.into());
            }
            WindowEvent::RedrawRequested => {
                draw_frame(display);
            }
            // Cierra el programa cuando se presiona la tecla Escape o se cierra la ventana
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed, logical_key: glium::winit::keyboard::Key::Named(NamedKey::Escape), ..
                    },
                ..
            } => event_loop.exit(),
            // Every other event
            _ignored_event => {}
        }
    }
}
