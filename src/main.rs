use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Browser Test",
        options,
        Box::new(|_cc| Ok(Box::new(BrowserTestApp::default()))),
    )
}

#[derive(Default)]
struct BrowserTestApp {
    url: String,
}

impl eframe::App for BrowserTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Browser Test");
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.url);
            });
            ui.separator();
            if ui.button("Open").clicked() {
                // TODO: Open the URL in a webview.
            }
        });
    }
}
