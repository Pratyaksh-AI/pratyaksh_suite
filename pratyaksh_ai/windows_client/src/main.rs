use eframe::egui;
use serde::Deserialize;

// --- DATA STRUCTURES ---
#[derive(Deserialize, Default, Debug)]
struct ComplianceRisk {
    penalty_estimate: i32,
    risk_level: String,
    act_section: String,
}

#[derive(PartialEq)]
enum Page {
    Compliance,
    Governance,
    Settings
}

// --- APP STATE ---
struct PratyakshApp {
    // Navigation
    current_page: Page,
    
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
            fy_end: "2023-03-31".to_owned(),
            form_type: "AOC-4".to_owned(),
            result: None,
            status_msg: "Ready".to_owned(),
        }
    }
}

// --- UI LOGIC ---
impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // 1. TOP HEADER (The "Brand" Bar)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("🇮🇳 PRATYAKSH AI | Corporate Intelligence Suite");
            ui.add_space(8.0);
        });

        // 2. SIDEBAR (Navigation)
        egui::SidePanel::left("side_panel").exact_width(160.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading("MODULES");
            ui.separator();
            ui.add_space(10.0);

            if ui.button("📋 Compliance").clicked() { self.current_page = Page::Compliance; }
            ui.add_space(10.0);
            if ui.button("⚖️ Governance").clicked() { self.current_page = Page::Governance; }
            ui.add_space(10.0);
            if ui.button("⚙️ Settings").clicked() { self.current_page = Page::Settings; }
        });

        // 3. CENTRAL PANEL (The Work Area)
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Compliance => self.render_compliance(ui),
                Page::Governance => { ui.heading("Governance Engine Coming Soon"); },
                Page::Settings => { ui.heading("Settings"); }
            }
        });
    }
}

impl PratyakshApp {
    fn render_compliance(&mut self, ui: &mut egui::Ui) {
        ui.heading("Compliance Risk Calculator");
        ui.separator();
        ui.add_space(20.0);

        // Grid Layout for Inputs
        egui::Grid::new("my_grid").spacing([40.0, 20.0]).show(ui, |ui| {
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

        // Big Analyze Button
        if ui.add_sized([120.0, 40.0], egui::Button::new("ANALYZE RISK")).clicked() {
            self.fetch_risk();
        }

        ui.add_space(30.0);
        ui.separator();

        // Results Section
        if let Some(res) = &self.result {
            ui.add_space(10.0);
            ui.heading("Analysis Report");
            
            ui.horizontal(|ui| {
                ui.label("Risk Level:");
                ui.colored_label(
                    if res.risk_level == "CRITICAL" { egui::Color32::RED } else { egui::Color32::GREEN },
                    format!("{}", res.risk_level)
                );
            });
            
            ui.label(format!("Estimated Penalty: ₹{}", res.penalty_estimate));
            ui.label(format!("Section: {}", res.act_section));
        } else {
            ui.label(&self.status_msg);
        }
    }

    fn fetch_risk(&mut self) {
        // NOTE: In production, run this async. Blocking for MVP simplicity.
        // Change URL if using Forwarded Ports!
        let url = format!("http://127.0.0.1:8080/api/v1/compliance/analyze?fy_end_date={}&form_type={}", self.fy_end, self.form_type);
        
        match reqwest::blocking::get(&url) {
            Ok(resp) => {
                if let Ok(data) = resp.json::<ComplianceRisk>() {
                    self.result = Some(data);
                    self.status_msg = "Success".to_string();
                } else {
                    self.status_msg = "Error parsing data".to_string();
                }
            },
            Err(e) => self.status_msg = format!("Connection Error: {}", e),
        }
    }
}

fn main() -> eframe::Result<()> {
    // Set custom fonts/style here if desired
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PratyakshAI",
        options,
        Box::new(|_cc| Box::new(PratyakshApp::default())),
    )
}
