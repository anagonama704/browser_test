mod analysis;
mod markdown;
mod model;

use analysis::{analyze_document, AnalysisReport};
use eframe::egui;
use markdown::parse_markdown;
use model::{BlockKind, Document};

const MIN_TAB_COUNT: usize = 1;
const TAB_SPACING: f32 = 6.0;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Philosophy Browser",
        options,
        Box::new(|cc| Ok(Box::new(BrowserTestApp::new(cc)))),
    )
}

struct BrowserTestApp {
    dark_mode: bool,
    tabs: Vec<TabState>,
    active_tab: usize,
}

impl BrowserTestApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dark_mode = cc.egui_ctx.style().visuals.dark_mode;
        Self {
            dark_mode,
            tabs: vec![TabState::new()],
            active_tab: 0,
        }
    }

    fn add_tab(&mut self) {
        self.tabs.push(TabState::new());
        self.active_tab = self.tabs.len().saturating_sub(1);
    }

    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= MIN_TAB_COUNT || index >= self.tabs.len() {
            return;
        }
        self.tabs.remove(index);
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len().saturating_sub(1);
        } else if self.active_tab > index {
            self.active_tab = self.active_tab.saturating_sub(1);
        }
    }

    fn active_tab_mut(&mut self) -> Option<&mut TabState> {
        self.tabs.get_mut(self.active_tab)
    }
}

struct TabState {
    markdown_path: String,
    document: Option<Document>,
    report: Option<AnalysisReport>,
    last_error: Option<String>,
}

impl TabState {
    fn new() -> Self {
        Self {
            markdown_path: String::new(),
            document: None,
            report: None,
            last_error: None,
        }
    }

    fn title(&self, index: usize) -> String {
        let trimmed = self.markdown_path.trim();
        if trimmed.is_empty() {
            return format!("Tab {}", index.saturating_add(1));
        }
        let path = std::path::Path::new(trimmed);
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            return file_name.to_string();
        }
        format!("Tab {}", index.saturating_add(1))
    }

    fn load_markdown(&mut self) {
        let path = self.markdown_path.trim();
        if path.is_empty() {
            self.document = None;
            self.report = None;
            self.last_error = Some("Please enter a Markdown file path.".to_string());
            return;
        }

        match std::fs::read_to_string(path) {
            Ok(content) => {
                let document = parse_markdown(&content);
                let report = analyze_document(&document);
                self.document = Some(document);
                self.report = Some(report);
                self.last_error = None;
            }
            Err(err) => {
                self.document = None;
                self.report = None;
                self.last_error = Some(format!("Failed to read file: {}", err));
            }
        }
    }
}

impl eframe::App for BrowserTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Philosophy Browser (Markdown v0)");
            ui.horizontal(|ui| {
                let mut selected_tab: Option<usize> = None;
                let mut close_tab: Option<usize> = None;
                let mut add_tab = false;

                for (idx, tab) in self.tabs.iter().enumerate() {
                    ui.push_id(idx, |ui| {
                        let label = tab.title(idx);
                        if ui.selectable_label(self.active_tab == idx, label).clicked() {
                            selected_tab = Some(idx);
                        }
                        if self.tabs.len() > MIN_TAB_COUNT {
                            if ui.small_button("x").clicked() {
                                close_tab = Some(idx);
                            }
                        }
                    });
                    ui.add_space(TAB_SPACING);
                }

                if ui.button("+").clicked() {
                    add_tab = true;
                }

                if let Some(idx) = close_tab {
                    self.close_tab(idx);
                }
                if add_tab {
                    self.add_tab();
                }
                if let Some(idx) = selected_tab {
                    if idx < self.tabs.len() {
                        self.active_tab = idx;
                    }
                }
            });
            ui.separator();
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
            if let Some(tab) = self.active_tab_mut() {
                ui.horizontal(|ui| {
                    ui.label("Markdown path:");
                    let response = ui.text_edit_singleline(&mut tab.markdown_path);
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        tab.load_markdown();
                    }
                    if ui.button("Load").clicked() {
                        tab.load_markdown();
                    }
                });

                if let Some(error) = &tab.last_error {
                    ui.colored_label(egui::Color32::RED, error);
                }

                ui.separator();
                ui.heading("Rendered Page");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(document) = &tab.document {
                        render_document(ui, document);
                    } else {
                        ui.label("No Markdown loaded yet.");
                    }
                });

                ui.separator();
                ui.heading("Hard Constraints Report");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(report) = &tab.report {
                        render_report(ui, report, tab.document.as_ref());
                    } else {
                        ui.label("No analysis yet.");
                    }
                });
            } else {
                ui.colored_label(
                    egui::Color32::RED,
                    "No tabs available. Please add a new tab.",
                );
            }
        });
    }
}

fn render_document(ui: &mut egui::Ui, document: &Document) {
    for block in &document.blocks {
        match &block.kind {
            BlockKind::Heading { level } => {
                let size = match level {
                    1 => 24.0,
                    2 => 20.0,
                    3 => 18.0,
                    _ => 16.0,
                };
                ui.label(egui::RichText::new(&block.text).size(size).strong());
            }
            BlockKind::Paragraph => {
                ui.label(&block.text);
            }
            BlockKind::ListItem { ordered, .. } => {
                let bullet = if *ordered { "1." } else { "-" };
                ui.horizontal(|ui| {
                    ui.label(bullet);
                    ui.label(&block.text);
                });
            }
            BlockKind::CodeBlock { .. } => {
                ui.label(egui::RichText::new(&block.text).monospace());
            }
            BlockKind::BlockQuote => {
                ui.label(egui::RichText::new(format!("> {}", block.text)).italics());
            }
        }
        ui.add_space(4.0);
    }
}

fn render_report(ui: &mut egui::Ui, report: &AnalysisReport, document: Option<&Document>) {
    if report.hard_fail() {
        ui.colored_label(
            egui::Color32::RED,
            format!("Hard fail: {} violation(s)", report.violations.len()),
        );
    } else {
        ui.colored_label(egui::Color32::GREEN, "No violations");
    }

    for violation in &report.violations {
        ui.separator();
        ui.label(format!(
            "Constraint {}: {}{}",
            violation.constraint_id,
            violation.summary,
            if violation.assumption {
                " (v0 assumption)"
            } else {
                ""
            }
        ));
        ui.label(&violation.details);
        if let Some(suggestion) = &violation.suggestion {
            ui.label(format!("Suggestion: {}", suggestion));
        }

        if let Some(index) = violation.block_index {
            if let Some(doc) = document {
                if let Some(block) = doc.blocks.get(index) {
                    let snippet = summarize(&block.text, 80);
                    ui.label(format!(
                        "Location: {} #{} - {}",
                        block.kind.label(),
                        index + 1,
                        snippet
                    ));
                }
            }
        }
    }

    if !report.assumptions.is_empty() {
        ui.separator();
        ui.label("v0 assumptions:");
        for item in &report.assumptions {
            ui.label(format!("- {}", item));
        }
    }
}

fn summarize(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let mut trimmed = String::new();
    for (idx, ch) in text.chars().enumerate() {
        if idx >= limit {
            break;
        }
        trimmed.push(ch);
    }
    trimmed.push_str("...");
    trimmed
}
