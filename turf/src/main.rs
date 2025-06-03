use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Turf", options, Box::new(|_cc| Box::<MyApp>::default()))
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Turf");
            ui.label("Search memory, list processes, etc.");
        });
    }
}
