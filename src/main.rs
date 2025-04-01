use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::IconExtWindows;
use winit::window::{Window, WindowId, WindowAttributes, Icon};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let icon = Icon::from_path("src/alto.ico", None).ok();

        let attrs = WindowAttributes::default()
        .with_title("Alto Browser")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .with_window_icon(icon)
        .with_resizable(false);

        self.window = Some(event_loop.create_window(attrs).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}