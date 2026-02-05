use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Browser Test",
        options,
        Box::new(|cc| Ok(Box::new(BrowserTestApp::new(cc)))),
    )
}

struct BrowserTestApp {
    url: String,
    dark_mode: bool,
}

impl BrowserTestApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dark_mode = cc.egui_ctx.style().visuals.dark_mode;
        Self {
            url: String::new(),
            dark_mode,
        }
    }
}

impl eframe::App for BrowserTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Browser Test");
            if ui
                .checkbox(&mut self.dark_mode, "Dark mode")
                .changed()
            {
                let visuals = if self.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                ctx.set_visuals(visuals);
            }
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
