// Use the re-exported winit dependency to avoid version mismatches.
// Requires the `simple_window_builder` feature.
use glium::{
    Surface, implement_vertex, uniform,
    winit::{
        self,
        application::ApplicationHandler,
        event::{Event, WindowEvent},
        window::Window,
    },
};
use impls::impls;

fn main() {
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Hello world").build(&event_loop);
    const {
        if impls!(Window: ApplicationHandler ) {
            panic!();
        }
    }
    let vertex_shader_src = r#"

    in vec2 position;

    uniform float x;

    void main() {
        vec2 pos = position;
        pos.x += x;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;
    let fragment_shader_src = r#"
out vec4 color;

void main() {
    color = vec4(1.0, 0.2, 0.0, 0.5);
}
"#;

    let (tx, rx) = std::sync::mpsc::channel();
    let _handle = std::thread::spawn(move || {
        loop {
            std::thread::park_timeout(std::time::Duration::from_millis(1000 / 30));
            println!("Msg sent");
            tx.send(()).ok();
        }
    });
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t: f32 = 0.0;
    #[allow(clippy::single_match, deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => {
                        // We update `t`
                        t += 0.01;
                        // We use the sine of t as an offset, this way we get a
                        // nice smooth animation
                        let x_off = t.sin() * 0.5;

                        let shape =
                            vec![Vertex { position: [-0.5 + x_off, -0.5] }, Vertex { position: [0.0 + x_off, 0.5] }, Vertex {
                                position: [0.5 + x_off, -0.25],
                            }];

                        let mut target = display.draw();
                        target.clear_color(0.5, 0.0, 0.3, 1.0);

                        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

                        target.draw(&vertex_buffer, indices, &program, &uniform! { x: x_off }, &Default::default()).unwrap();
                        target.finish().unwrap();
                    }
                    WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                }
            }
            Event::AboutToWait => {
                if rx.try_recv().is_ok() {
                    // We can use this to trigger a redraw
                    window.request_redraw();
                    match rx.try_iter().count() {
                        0 => (),
                        n => {
                            println!("Received {} messages", n);
                        }
                    };
                }
            }
            _ => (),
        }
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
