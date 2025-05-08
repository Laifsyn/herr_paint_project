#[macro_use]
extern crate glium;
use std::borrow::Cow;
use std::sync::mpsc::channel;
use std::time::Duration;

use glium::Surface;
use glium::texture::{ClientFormat, RawImage2d};

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Glium tutorial #6").build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);
    // We've changed our shape to a rectangle so the image isn't distorted.
    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let (tx, rx) = channel();

    let mut t = 0.0_f32;

    let mut offset: u8 = 0;

    std::thread::spawn(move || {
        let mut offset = 0_u8;
        let mut add = 1_i8;
        loop {
            // Simulate some work
            // Send a message to the main thread
            tx.send(offset).unwrap();

            match add {
                1 => {
                    offset = std::ops::Add::add(offset, 1);
                    if offset == 254 {
                        add = -1;
                    }
                }
                -1 => {
                    offset = std::ops::Sub::sub(offset, 1);
                    if offset == 1 {
                        add = 1;
                    }
                }
                _ => {
                    unreachable!()
                }
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    });
    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::winit::event::WindowEvent::CloseRequested => {
                            window_target.exit();
                        }
                        // We now need to render everyting in response to a RedrawRequested event due to the animation
                        glium::winit::event::WindowEvent::RedrawRequested => {
                            // we update `t`
                            t += 0.02;
                            let x = t.sin() * 0.5;

                            let mut target = display.draw();
                            target.clear_color(0.0, 0.0, 1.0, 1.0);

                            let (width, height) = target.get_dimensions();
                            offset = rx.try_recv().unwrap_or(offset);
                            let num = 127_u8.wrapping_add(offset);
                            let data = vec![(num, num, num); width as usize * height as usize];
                            let format = ClientFormat::U8U8U8;
                            let image = RawImage2d { data: Cow::Borrowed(&data), width, height, format };
                            let texture = glium::Texture2d::new(&display, image).unwrap();
                            let uniforms = uniform! {
                                matrix: [
                                    [1.0, 0.0, 0.0, 0.0],
                                    [0.0, 1.0, 0.0, 0.0],
                                    [0.0, 0.0, 1.0, 0.0],
                                    [ x , 0.0, 0.0, 1.0f32],
                                ],
                                tex: &texture,
                            };

                            target.draw(&vertex_buffer, indices, &program, &uniforms, &Default::default()).unwrap();
                            target.finish().unwrap();
                        }
                        // Because glium doesn't know about windows we need to resize the display
                        // when the window's size has changed.
                        glium::winit::event::WindowEvent::Resized(window_size) => {
                            display.resize(window_size.into());
                        }
                        _ => (),
                    }
                }
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
