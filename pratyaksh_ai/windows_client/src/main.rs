use eframe::egui;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct ComplianceRisk {
    penalty_estimate: i32,
    risk_level: String,
    act_section: String,
}

struct PratyakshApp {
    fy_end: String,
    form_type: String,
    result: Option<ComplianceRisk>,
    status: String,
}

impl Default for PratyakshApp {
    fn default() -> Self {
        Self {
            fy_end: "2023-03-31".to_owned(),
            form_type: "AOC-4".to_owned(),
            result: None,
            status: "Ready".to_owned(),
        }
    }
}

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PratyakshAI CS Suite (Windows)");
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("FY End Date:");
                ui.text_edit_singleline(&mut self.fy_end);
            });
            
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.form_type, "AOC-4".to_string(), "AOC-4");
                ui.radio_value(&mut self.form_type, "MGT-7".to_string(), "MGT-7");
            });

            if ui.button("Check Risk").clicked() {
                // In production, run this in a thread!
                let url = format!("http://127.0.0.1:8080/api/v1/compliance/analyze?fy_end_date={}&form_type={}", self.fy_end, self.form_type);
                match reqwest::blocking::get(&url) {
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<ComplianceRisk>() {
                            self.result = Some(data);
                            self.status = "Success".to_string();
                        }
                    },
                    Err(e) => self.status = format!("Error: {}", e),
                }
            }

            if let Some(res) = &self.result {
                ui.colored_label(egui::Color32::RED, format!("Risk: {}", res.risk_level));
                ui.label(format!("Penalty: ₹{}", res.penalty_estimate));
            }
        });
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