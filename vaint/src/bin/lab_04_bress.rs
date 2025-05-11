use glium::winit::event::Event;
use glium::winit::event_loop::ActiveEventLoop;
use glium::{implement_vertex, program};
use glutin::surface::WindowSurface;

const W: f32 = 1000_f32;
const H: f32 = 600_f32;
/// Punto de entrada de la aplicación
fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().unwrap();

    // 2. Create a glutin context and glium Display
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(W as u32, H as u32)
        .with_title("Lab3 Digital Differential Analyzer")
        .build(&event_loop);

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
impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self { Vertex { position, color } }

    pub fn new_red(position: [f32; 2]) -> Self { Vertex::new(position, [1.0, 0.0, 0.0]) }

    pub fn new_green(position: [f32; 2]) -> Self { Vertex::new(position, [0.0, 1.0, 0.0]) }

    pub fn new_blue(position: [f32; 2]) -> Self { Vertex::new(position, [0.0, 0.0, 1.0]) }
}
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
pub fn transformar_coordenadas(coordenadas: (i32, i32)) -> [f32; 2] {
    let (x, y) = coordenadas;
    let x = x as f32 / W * 2.0 - 1.0;
    let y = y as f32 / H * -2.0 + 1.0; // Invertir el eje Y
    [x, y]
}

/// Iterador que devuelve objetos tipo [`Vertex`] que es usado para ubicar puntos en el canvas
pub fn colorear_coordenadas(coordenadas: &[[f32; 2]], color: [f32; 3]) -> impl Iterator<Item = Vertex> + '_ {
    coordenadas.iter().map(move |&pos| Vertex { position: pos, color })
}

/// Algoritmo DDA (Digital Differential Analyzer) para dibujar líneas en un espacio 2D usando puntos
/// discretos.
pub fn dda((x_0, y_0): (i32, i32), (x, y): (i32, i32)) -> Vec<[i32; 2]> {
    let delta_x = x - x_0;
    let delta_y = y - y_0;

    let steps = i32::max(delta_x.abs(), delta_y.abs());

    let dx = (delta_x as f32) / steps as f32;
    let dy = (delta_y as f32) / steps as f32;

    let steps = steps + 1; // Se agrega un paso adicional para incluir el último punto
    let mut puntos = Vec::with_capacity(steps as usize);
    for k in 0..steps {
        let k: f32 = k as f32;
        let x = x_0 + (dx * k) as i32; // Type-Casting a entero trunca los decimales.
        let y = y_0 + (dy * k) as i32;
        puntos.push([x, y]);
    }
    debug_assert_eq!(
        puntos.last().unwrap(),
        &[x, y],
        "Se esperaba que el último punto fuera ({}, {}), pero se obtuvo {:?}",
        x,
        y,
        puntos.last().unwrap()
    );
    puntos
}
/// Contiene la logica sobre el manejo de los eventos a la ventana y la aplicacion durante ejecución
mod ventana {
    use glium::index::{NoIndices, PrimitiveType};
    use glium::winit::event::{ElementState, KeyEvent, WindowEvent};
    use glium::winit::event_loop::ActiveEventLoop;
    use glium::winit::keyboard::NamedKey;
    use glium::winit::window::WindowId;
    use glium::{Surface, VertexBuffer, uniform};
    use glutin::surface::WindowSurface;

    use crate::{Vertex, dda, transformar_coordenadas};

    /// Funcion Auxiliar que dibuja una línea entre dos puntos y aparte dibuja una serie de puntos
    /// paralela a la línea desplazado según `points_shift`
    fn dibujar_linea(
        display: &glium::Display<WindowSurface>,
        target: &mut glium::Frame,
        p0: (i32, i32),
        p: (i32, i32),
        points_shift: (i32, i32), // Desplazamiento de la línea puntual
        programa: &glium::Program,
    ) {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };
        let draw_params = glium::DrawParameters { ..Default::default() };
        let vec_shift = |(vec_x, vec_y), pos: [i32; 2]| (pos[0] + vec_x, pos[1] + vec_y);

        // Puntos individuales de la línea
        let puntos = dda(p0, p) // Realizamos el algoritmo DDA
            .into_iter()
            .map(|v| vec_shift(points_shift, v)) // Desplazamos la serie de puntos
            .map(transformar_coordenadas) // Normalizamos al espacio de coordenadas de OpenGL
            .map(Vertex::new_blue) // Coloreamos los puntos
            .collect::<Vec<_>>(); // Recogemos los puntos a un vector
        let puntos = VertexBuffer::new(display, &puntos).unwrap();
        target.draw(&puntos, NoIndices(PrimitiveType::Points), programa, &uniforms, &draw_params).ok();
        let vertexes: [Vertex; 2] = [Vertex::new_red(transformar_coordenadas(p0)), Vertex::new_red(transformar_coordenadas(p))];
        target // Dibujar la linea con la implementación de OpenGL
            .draw(
                &VertexBuffer::new(display, &vertexes).unwrap(),
                NoIndices(PrimitiveType::LineStrip),
                programa,
                &uniforms,
                &draw_params,
            )
            .ok();
    }

    /// Codigo para controlar lo que se dibuja en la ventana
    fn draw_frame(display: &glium::Display<WindowSurface>) {
        // Obtener el objeto dibujable a la pantalla
        let mut target: glium::Frame = display.draw();

        // Limpiar la pantalla
        target.clear_color(0.2, 0.7, 0.2, 1.0); // Blanco

        let programa = crate::programa(display);

        // Lineas azules: punteadas.
        // Lineas rojas: continuas.
        dibujar_linea(display, &mut target, (86, 36), (86, 279), (-50, 0), &programa);
        dibujar_linea(display, &mut target, (188, 93), (432, 93), (0, 50), &programa);
        dibujar_linea(display, &mut target, (630, 58), (458, 230), (26, 26), &programa);
        dibujar_linea(display, &mut target, (664, 50), (836, 221), (26, -26), &programa);

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
