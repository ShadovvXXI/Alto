#![allow(unused_imports)]
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, RawKeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::windows::IconExtWindows;
use winit::window::{Fullscreen, Icon, Window, WindowAttributes, WindowId};
use winit::keyboard::{self, NamedKey, PhysicalKey};
use winit::raw_window_handle::HasDisplayHandle;
use egui::{*};
use egui_winit::{*};

#[derive(Default)]
struct App {
    window: Option<Window>,
    egui_context : Context,
    egui_winit : Option<egui_winit::State>,
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

        self.egui_context = Context::default();
        
        self.egui_winit = Some(egui_winit::State::new(
            self.egui_context.clone(), 
            self.egui_context.viewport_id(), 
            &self.window.as_ref().unwrap().display_handle().unwrap(), 
            Some(1.0), 
            None, 
            Some(1024),
        ));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window_ref = self.window.as_ref().unwrap();
        let egui_winit = self.egui_winit.as_mut().unwrap();
        
        let event_response = egui_winit.on_window_event(&window_ref, &event);
        
        if event_response.repaint {
            window_ref.request_redraw();
        }
        
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let raw_input = egui_winit.take_egui_input(window_ref);
                
                let full_output = self.egui_context.run(raw_input, |ctx| {
                    CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello, world!");
                        if ui.button("Click me").clicked() {
                            println!("Button clicked!");
                        }
                    });
                });
                
                #[allow(unused_variables)]
                let clipped_primitives = self.egui_context.tessellate(full_output.shapes, 1.0);

                egui_winit.handle_platform_output(
                    window_ref,
                    full_output.platform_output
                );

                window_ref.request_redraw();
            }
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                match (event.physical_key, event.state) {
                    (PhysicalKey::Code(keyboard::KeyCode::F11), ElementState::Pressed) => { 
                        match window_ref.fullscreen() {
                            None => { 
                                window_ref.set_fullscreen(Some(Fullscreen::Borderless(window_ref.current_monitor())));
                            }
                            Some(Fullscreen::Borderless(_monitor_handle)) => {
                                window_ref.set_fullscreen(None);
                            }
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

    #[allow(unused_must_use)]
    event_loop.run_app(&mut app);
}