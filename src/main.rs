use eframe::egui;

/// Launches the eframe application window titled "Browser Test" and runs the BrowserTestApp.

///

/// # Examples

///

/// ```no_run

/// fn run_app() {

///     // Starts the GUI; do not run in tests.

///     let _ = crate::main();

/// }

/// ```
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
    /// Creates a new BrowserTestApp using the provided creation context.
    ///
    /// The returned app has an empty `url` and `dark_mode` initialized from the `egui` visuals
    /// available in `cc` (true if the current visuals use dark mode).
    ///
    /// # Examples
    ///
    /// ```
    /// # use eframe::CreationContext;
    /// fn build_app(cc: &CreationContext) {
    ///     let app = BrowserTestApp::new(cc);
    ///     assert!(app.url.is_empty());
    /// }
    /// ```
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dark_mode = cc.egui_ctx.style().visuals.dark_mode;
        Self {
            url: String::new(),
            dark_mode,
        }
    }
}

impl eframe::App for BrowserTestApp {
    /// Updates the application's UI for the current frame, drawing the central panel with the title,
    /// a dark mode toggle, a URL input, and an "Open" button.
    ///
    /// Toggling "Dark mode" applies the corresponding egui visuals to the provided context.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // The eframe runtime provides `ctx` and `frame` and calls `update` each frame:
    /// // eframe::run_native(..., Box::new(|cc| Ok(Box::new(BrowserTestApp::new(cc)))));
    /// ```
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