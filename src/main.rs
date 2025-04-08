use egui::{*};
use eframe::{*};
use image::open;

struct App{
    url : String,
    tabs : Vec<String>,
    current_tab : usize
}

impl Default for App{
    fn default() -> App{
        App {
            url : String::from("https://example.com"),
            tabs : ["start tab".to_string()].to_vec(),
            current_tab : 0,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("SupremeLL-Medium.otf")).into(),
    );

    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        setup_custom_fonts(ctx);
        CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui|{
                for tab_i in 0..self.tabs.len(){
                    if self.tabs.len() > tab_i{
                        if self.tabs.len()!=1{
                            ui.spacing_mut().item_spacing = egui::vec2(-14.0, 0.0);
                        } 
                        if ui.selectable_label(
                            if self.current_tab == tab_i {true} else {false},
                            self.tabs[tab_i].clone() + {if self.tabs.len()!=1 {"    "} else {""}})
                        .clicked() {
                            self.current_tab = tab_i;
                        };

                        ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);
                        if self.tabs.len()!=1{
                            if ui.button("x").clicked() {
                                self.tabs.remove(tab_i);
                                if tab_i < self.current_tab {
                                    self.current_tab -= 1
                                }
                                else if tab_i == self.current_tab {
                                    self.current_tab = 
                                    if tab_i == self.tabs.len() {
                                        tab_i-1
                                    } else {
                                        tab_i
                                    }
                                }
                            }
                        } 
                    }
                };
                if ui.button("+").clicked() {
                    self.tabs.push("temp_name".to_string());
                }
            });
            ui.horizontal( |ui|{
                if ui.button("←").clicked(){
                    println!("Loading previous page");
                }
                if ui.button("↺").clicked(){
                    println!("Update current page");
                }
                if ui.button("→").clicked(){
                    println!("Loading next page");
                }
                
                ui.add (TextEdit::singleline(&mut self.url));
                if ui.button("Перейти").clicked(){
                    println!("Loading: {}", self.url);
                }   
            });
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