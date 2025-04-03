use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, RawKeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::IconExtWindows;
use winit::window::{Fullscreen, Icon, Window, WindowAttributes, WindowId};
use winit::keyboard::{self, NamedKey, PhysicalKey};

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
        .with_resizable(true);

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
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                let window_ref = self.window.as_ref().unwrap();
                match (event.physical_key, event.state) {
                    (PhysicalKey::Code(keyboard::KeyCode::F11), ElementState::Pressed) => { 
                        match self.window.as_ref().unwrap().fullscreen() {
                            None => { 
                                window_ref.set_fullscreen(Some(Fullscreen::Borderless(window_ref.current_monitor())))
                            }
                            Some(Fullscreen::Borderless(_monitor_handle)) => {window_ref.set_fullscreen(None)}
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // fn device_event(
    //         &mut self,
    //         event_loop: &ActiveEventLoop,
    //         device_id: winit::event::DeviceId,
    //         event: DeviceEvent,
    //     ) {
    //     match event {
    //         DeviceEvent::Key(RawKeyEvent { physical_key, state }) => {
    //             let window_ref = self.window.as_ref().unwrap();
    //             match (physical_key, state) {
    //                 (PhysicalKey::Code(keyboard::KeyCode::F11), ElementState::Pressed) => { 
    //                     match self.window.as_ref().unwrap().fullscreen() {
    //                         None => { 
    //                             window_ref.set_fullscreen(Some(Fullscreen::Borderless(window_ref.current_monitor())))
    //                         }
    //                         Some(Fullscreen::Borderless(_monitor_handle)) => {window_ref.set_fullscreen(None)}
    //                         _ => (),
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         }
    //         _ => (),
    //     }   
    // }

}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}