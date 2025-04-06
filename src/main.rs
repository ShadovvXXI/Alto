use egui::{*};
use eframe::{*};
use image::open;

struct App{
    url : String
}

impl Default for App{
    fn default() -> App{
        App {
            url : String::from("https://example.com")
        }
    }
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{
            ui.horizontal( |ui|{
                ui.text_edit_singleline(&mut "Some");
                if ui.button("Перейти").clicked(){
                    println!("Loading: {}", self.url);
                }
            })
        });
    }
}

fn load_icon(path: &str) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport : egui::ViewportBuilder{
            title : Some("Alto Browser".to_string()),
            icon : Some(load_icon("src/alto.ico").into()),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = run_native(
        "Alto Browser", 
        options, 
        Box::new(|shanat| Ok(Box::new(App::default())))
    );

    Ok(())
}