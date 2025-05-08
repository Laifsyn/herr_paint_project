use glium::winit::event::Event;
use glium::winit::event_loop::ActiveEventLoop;
use glium::{implement_vertex, program};
use glutin::surface::WindowSurface;

/// Punto de entrada de la aplicación
fn main() {
    // Inicializamos la instancia del EventLoop que se encarga de procesar los eventos en el programa.
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().unwrap();

    // Creanos una ventana de 200x150 pixeles
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(400, 300)
        .with_title("Programa OpenGl Ejemplo")
        .build(&event_loop);

    #[allow(deprecated)]
    event_loop
        // .run(....) equivalente a glutMainLoop()
        .run(move |evento: Event<()>, r#loop: &ActiveEventLoop| {
            match evento {
                Event::WindowEvent { window_id, event: win_event } => {
                    // Delegamos el manejo de eventos a la función procesar_eventos
                    //
                    // Podemos ir a la función `draw_frame` para ver el siguiente paso sobre dibujo en la ventana
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
pub fn transformar_coordenadas(coordenadas: (u32, u32)) -> [f32; 2] {
    let (x, y) = coordenadas;
    let x = x as f32 / 400_f32 * 2.0 - 1.0;
    let y = y as f32 / 300_f32 * -2.0 + 1.0; // Invertir el eje Y
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
        let pos = |x: u32, y: u32| transformar_coordenadas((x, y));

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
    use glium::{IndexBuffer, Surface, VertexBuffer, uniform};
    use glutin::surface::WindowSurface;

    use super::{colorear_coordenadas, transformar_coordenadas};
    use crate::Vertex;

    /// Codigo para controlar lo que se dibuja en la ventana
    fn draw_frame(display: &glium::Display<glutin::surface::WindowSurface>) {
        // Obtener el objeto dibujable a la pantalla
        let mut target: glium::Frame = display.draw();

        // Limpiar la pantalla a un color blanco
        // C: `glClearColor (1.0, 1.0, 1.0, 0.0);`
        target.clear_color(1.0, 0.8, 1.0, 1.0); // "lila rosadito"

        // # Inicializacion
        // Uniforms que el openGL va a usar.
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };
        // Obtenemos el script que OpenGL utiliza para dibujar los vertices
        let programa = crate::programa(display);

        let mut figura_vertices = Vec::new();

        // # Dibujo
        // Linea - Creamos los objetos para dibujar una linea roja que va desde (180, 45) a (10, 15)
        let (rect, index_buffer): (VertexBuffer<Vertex>, IndexBuffer<u16>) =
            crear_figura!(&mut figura_vertices, display, [1.0, 0.0, 0.0], (180, 120), (10, 15));
        // Parámetros de dibujo personalizados (rellenado con parámetros defecto)
        let parámetro_dibujo: glium::DrawParameters<'_> =
            glium::draw_parameters::DrawParameters { line_width: Some(5.0), ..Default::default() };
        // Dibujamos la figura
        target.draw(&rect, &index_buffer, &programa, &uniforms, &parámetro_dibujo).unwrap();

        // Finalizamos el dibujo, y flusheamos el buffer para que se vea en pantalla
        target.finish().unwrap(); // `.unwrap()` porque el flushing puede fallar
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
