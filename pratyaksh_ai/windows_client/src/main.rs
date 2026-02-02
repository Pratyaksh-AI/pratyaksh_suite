use eframe::egui;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

// ============================================================================
//  1. EMBEDDED ASSETS (FIXED SYNTAX)
//  We use r##" ... "## to safely include special characters like # inside strings.
// ============================================================================

const ICON_LOGO: &[u8] = r##"
<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="#00BFFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
<path d="M2 17L12 22L22 17" stroke="#00BFFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
<path d="M2 12L12 17L22 12" stroke="#00BFFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
"##.as_bytes();

const ICON_DASHBOARD: &[u8] = r##"
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" xmlns="http://www.w3.org/2000/svg">
<rect x="3" y="3" width="7" height="9"></rect>
<rect x="14" y="3" width="7" height="5"></rect>
<rect x="14" y="12" width="7" height="9"></rect>
<rect x="3" y="16" width="7" height="5"></rect>
</svg>
"##.as_bytes();

const ICON_SETTINGS: &[u8] = r##"
<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" xmlns="http://www.w3.org/2000/svg">
<circle cx="12" cy="12" r="3"></circle>
<path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
</svg>
"##.as_bytes();

// ============================================================================
//  2. DATA & CONFIG
// ============================================================================

#[derive(Deserialize, Debug, Clone)]
struct ComplianceRisk {
    penalty_estimate: i32,
    risk_level: String,
    act_section: String,
}

#[derive(PartialEq, Clone, Copy)]
enum Page { Dashboard, Settings }

#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    backend_url: String,
    last_fy_end: String,
    last_form_type: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            backend_url: "https://your-codespace-url-here.app.github.dev".to_string(),
            last_fy_end: "2023-03-31".to_string(),
            last_form_type: "AOC-4".to_string(),
        }
    }
}

// ============================================================================
//  3. APP STATE
// ============================================================================

struct PratyakshApp {
    config: AppConfig,
    current_page: Page,
    fy_end: String,
    form_type: String,
    result: Option<ComplianceRisk>,
    is_loading: bool,
    error_msg: Option<String>,
    rx: Receiver<Result<ComplianceRisk, String>>,
    tx: Sender<Result<ComplianceRisk, String>>,
}

impl PratyakshApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 1. INSTALL SVG LOADERS
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // 2. LOAD CONFIG
        let config: AppConfig = confy::load("pratyaksh_ai", "config").unwrap_or_default();
        
        // 3. APPLY THEME
        setup_futuristic_theme(&cc.egui_ctx);
        
        let (tx, rx) = channel();

        Self {
            fy_end: config.last_fy_end.clone(),
            form_type: config.last_form_type.clone(),
            config,
            current_page: Page::Dashboard,
            result: None,
            is_loading: false,
            error_msg: None,
            rx,
            tx,
        }
    }

    fn save(&self) {
        let mut new_config = self.config.clone();
        new_config.last_fy_end = self.fy_end.clone();
        new_config.last_form_type = self.form_type.clone();
        let _ = confy::store("pratyaksh_ai", "config", new_config);
    }
}

// ============================================================================
//  4. UI RENDERING
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(res) = self.rx.try_recv() {
            self.is_loading = false;
            match res {
                Ok(data) => self.result = Some(data),
                Err(e) => self.error_msg = Some(e),
            }
        }

        egui::SidePanel::left("sidebar")
            .exact_width(240.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(30.0);
                ui.horizontal(|ui| {
                    ui.add(egui::Image::from_bytes("bytes://logo.svg", ICON_LOGO).max_width(40.0));
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new("PRATYAKSH").size(18.0).strong().color(egui::Color32::WHITE));
                        ui.label(egui::RichText::new("AI SUITE").size(10.0).color(egui::Color32::from_rgb(0, 191, 255)));
                    });
                });

                ui.add_space(50.0);

                if nav_btn(ui, "Dashboard", ICON_DASHBOARD, self.current_page == Page::Dashboard).clicked() {
                    self.current_page = Page::Dashboard;
                }
                ui.add_space(10.0);
                if nav_btn(ui, "Configuration", ICON_SETTINGS, self.current_page == Page::Settings).clicked() {
                    self.current_page = Page::Settings;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Dashboard => self.render_dashboard(ui),
                Page::Settings => self.render_settings(ui),
            }
        });
    }
}

impl PratyakshApp {
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.heading(egui::RichText::new("Compliance Intelligence").size(28.0).strong());
        ui.label(egui::RichText::new("Real-time risk assessment engine").color(egui::Color32::GRAY));
        ui.add_space(30.0);

        let card_bg = egui::Color32::from_rgb(20, 25, 35);
        let border = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 60, 80));

        egui::Frame::group(ui.style())
            .fill(card_bg)
            .stroke(border)
            .rounding(16.0)
            .inner_margin(30.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("FY End Date");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.fy_end).desired_width(150.0));
                    });
                });
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.label("Form Type");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                         ui.radio_value(&mut self.form_type, "MGT-7".to_string(), "MGT-7");
                         ui.radio_value(&mut self.form_type, "AOC-4".to_string(), "AOC-4");
                    });
                });

                ui.add_space(30.0);

                let btn_text = if self.is_loading { "PROCESSING..." } else { "RUN ANALYSIS" };
                let btn = egui::Button::new(egui::RichText::new(btn_text).size(16.0).color(egui::Color32::WHITE))
                    .min_size(egui::vec2(ui.available_width(), 50.0))
                    .fill(egui::Color32::from_rgb(0, 100, 255))
                    .rounding(12.0);

                if ui.add_enabled(!self.is_loading, btn).clicked() {
                    self.analyze();
                }
            });

        if let Some(res) = &self.result {
            ui.add_space(30.0);
            let is_safe = res.risk_level != "CRITICAL";
            let color = if is_safe { egui::Color32::GREEN } else { egui::Color32::from_rgb(255, 60, 60) };

            egui::Frame::group(ui.style())
                .fill(color.linear_multiply(0.1))
                .stroke(egui::Stroke::new(1.0, color))
                .rounding(16.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(egui::RichText::new(&res.risk_level).color(color).size(24.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.heading(egui::RichText::new(format!("₹{}", res.penalty_estimate)).color(egui::Color32::WHITE));
                        });
                    });
                    ui.label(format!("Based on Section {}", res.act_section));
                });
        }
        
        if let Some(err) = &self.error_msg {
            ui.add_space(20.0);
            ui.colored_label(egui::Color32::RED, format!("⚠️ {}", err));
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.heading("Configuration");
        ui.add_space(20.0);
        ui.label("Cloud Backend URL:");
        if ui.add(egui::TextEdit::singleline(&mut self.config.backend_url).desired_width(400.0)).changed() {
            self.save();
        }
    }

    fn analyze(&mut self) {
        self.is_loading = true;
        self.result = None;
        self.error_msg = None;
        self.save();

        let tx = self.tx.clone();
        let url = format!("{}/api/v1/compliance/analyze?fy_end_date={}&form_type={}", 
            self.config.backend_url.trim_end_matches('/'), self.fy_end, self.form_type);

        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                tokio::time::sleep(Duration::from_millis(500)).await;
                match reqwest::get(&url).await {
                    Ok(resp) => match resp.json::<ComplianceRisk>().await {
                        Ok(data) => { let _ = tx.send(Ok(data)); },
                        Err(_) => { let _ = tx.send(Err("Data Parse Error".into())); }
                    },
                    Err(_) => { let _ = tx.send(Err("Connection Failed".into())); }
                }
            });
        });
    }
}

fn nav_btn(ui: &mut egui::Ui, text: &str, icon_bytes: &[u8], active: bool) -> egui::Response {
    let bg = if active { egui::Color32::from_rgb(30, 35, 50) } else { egui::Color32::TRANSPARENT };
    let fg = if active { egui::Color32::WHITE } else { egui::Color32::GRAY };
    
    egui::Frame::none().fill(bg).rounding(8.0).inner_margin(10.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon_bytes).max_width(20.0).tint(fg));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(fg).size(16.0));
        });
    }).response.interact(egui::Sense::click())
}

fn setup_futuristic_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = egui::Color32::from_rgb(10, 12, 16);
    visuals.panel_fill = egui::Color32::from_rgb(10, 12, 16);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 25, 30);
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 255);
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_title("PRATYAKSH 2026"),
        ..Default::default()
    };
    eframe::run_native(
        "PratyakshAI",
        options,
        Box::new(|cc| Box::new(PratyakshApp::new(cc))),
    )
}