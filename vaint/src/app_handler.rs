use glium::winit::application::ApplicationHandler;

pub struct AppHandler {
    request_redraw: bool,
    wait_cancelled: bool,
    close_requested: bool,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) { todo!() }

    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        todo!()
    }

    fn about_to_wait(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        if self.request_redraw && !self.wait_cancelled && !self.close_requested {
            // self.window.as_ref().unwrap().request_redraw();
            todo!()
        }
    }
}
