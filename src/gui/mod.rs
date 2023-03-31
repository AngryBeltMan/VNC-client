use eframe::egui;
use std::sync::{Arc,Mutex};
use std::time::Duration;
pub struct MyApp {
    code:String
}
impl MyApp {
    pub fn new(code:impl Into<String>) -> Self {
        Self {
            code:code.into()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Epic VNC");
            ui.horizontal(|ui| {
                ui.label(&format!("Join code: {}",&self.code));
            });
        });
    }
}
