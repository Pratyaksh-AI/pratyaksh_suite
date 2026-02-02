use eframe::egui;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct ComplianceRisk {
    penalty_estimate: i32,
    risk_level: String,
    act_section: String,
}

#[derive(PartialEq)]
enum Page {
    Compliance,
    Settings
}

struct PratyakshApp {
    current_page: Page,
    // Configuration
    backend_url: String, 
    
    // Compliance Data
    fy_end: String,
    form_type: String,
    result: Option<ComplianceRisk>,
    status_msg: String,
}

impl Default for PratyakshApp {
    fn default() -> Self {
        Self {
            current_page: Page::Compliance,
            // Default to localhost, but allowing user change
            backend_url: "http://127.0.0.1:8080".to_owned(), 
            fy_end: "2023-03-31".to_owned(),
            form_type: "AOC-4".to_owned(),
            result: None,
            status_msg: "Ready".to_owned(),
        }
    }
}

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // TOP BAR
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🇮🇳 PRATYAKSH AI");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙️ Config").clicked() { self.current_page = Page::Settings; }
                    if ui.button("📋 Dashboard").clicked() { self.current_page = Page::Compliance; }
                });
            });
        });

        // CENTRAL PANEL
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Compliance => {
                    ui.heading("Compliance Risk Engine");
                    ui.separator();
                    ui.add_space(10.0);

                    // Inputs
                    egui::Grid::new("input_grid").spacing([20.0, 10.0]).show(ui, |ui| {
                        ui.label("FY End Date:");
                        ui.text_edit_singleline(&mut self.fy_end);
                        ui.end_row();

                        ui.label("Form Type:");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.form_type, "AOC-4".to_string(), "AOC-4");
                            ui.radio_value(&mut self.form_type, "MGT-7".to_string(), "MGT-7");
                        });
                        ui.end_row();
                    });

                    ui.add_space(20.0);
                    if ui.button("🚀 Analyze Risk").clicked() {
                        self.fetch_risk();
                    }

                    // Results
                    ui.separator();
                    if let Some(res) = &self.result {
                        ui.colored_label(
                            if res.risk_level == "CRITICAL" { egui::Color32::RED } else { egui::Color32::GREEN },
                            format!("RISK LEVEL: {}", res.risk_level)
                        );
                        ui.label(format!("Penalty: ₹{}", res.penalty_estimate));
                        ui.label(format!("Section: {}", res.act_section));
                    } else {
                        ui.label(&self.status_msg);
                    }
                    
                    ui.add_space(20.0);
                    ui.small(format!("Connected to: {}", self.backend_url));
                },
                Page::Settings => {
                    ui.heading("System Configuration");
                    ui.separator();
                    ui.label("Server Connection URL:");
                    ui.text_edit_singleline(&mut self.backend_url);
                    ui.small("If running backend in Cloud/Codespaces, paste the Public URL here.");
                    ui.small("Example: https://musical-space-waddle-8080.app.github.dev");
                }
            }
        });
    }
}

impl PratyakshApp {
    fn fetch_risk(&mut self) {
        // Remove trailing slash if user added it
        let base = self.backend_url.trim_end_matches('/');
        let url = format!("{}/api/v1/compliance/analyze?fy_end_date={}&form_type={}", 
            base, self.fy_end, self.form_type);
        
        match reqwest::blocking::get(&url) {
            Ok(resp) => {
                if let Ok(data) = resp.json::<ComplianceRisk>() {
                    self.result = Some(data);
                    self.status_msg = "Success".to_string();
                } else {
                    self.status_msg = "Error parsing server response".to_string();
                }
            },
            Err(e) => {
                self.status_msg = format!("Connection Failed: {}", e);
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PratyakshAI",
        options,
        Box::new(|_cc| Box::new(PratyakshApp::default())),
    )
}