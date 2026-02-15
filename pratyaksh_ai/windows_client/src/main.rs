#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{Local, NaiveDate};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS & THEME
// ============================================================================

const COLOR_ACCENT: egui::Color32 = egui::Color32::from_rgb(79, 249, 120); // #4FF978 (Neon Green)
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(17, 17, 17);       // #111111 (Deep Black)
const COLOR_TEXT: egui::Color32 = egui::Color32::WHITE;
const COLOR_MUTED: egui::Color32 = egui::Color32::GRAY;

const ICON_GRID: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFFFFF"><path d="M4 4h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4zM4 10h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4zM4 16h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4z"/></svg>"##.as_bytes();
const ICON_CALC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFFFFF"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-6 13h-2v-2h2v2zm0-4h-2V7h2v5z"/></svg>"##.as_bytes();
const ICON_SHIELD: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFFFFF"><path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/></svg>"##.as_bytes();
const ICON_SETTINGS: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFFFFF"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.57 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>"##.as_bytes();

// ============================================================================
//  2. DATA MODELS
// ============================================================================

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Dashboard,
    Modules,
    Tools,
    Settings
}

#[derive(PartialEq, Clone, Copy)]
enum ActiveTool {
    None,
    McaPredictor,
    BoardRisk,
    TrustScore,
    RegulatorNotes,
    MsmeCalc,
    GratuityCalc,
    PenaltyCalc,
}

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    active_tool: ActiveTool,
    
    // Data
    client_count: i32,
    evidence_count: i32,
    risk_data: HashMap<String, i32>,

    // Inputs
    risk_city: String,
    new_client_name: String,
    new_client_city: String,
    ev_client_name: String,
    ev_note: String,
    
    // Tool Inputs
    mca_city: String,
    mca_form: String, // Was causing warning, fixed usage below
    mca_result: String,
    board_text: String,
    board_result: Vec<String>,
    trust_gst: String,
    trust_bank: String,
    trust_result: String,
    reg_id: String,
    reg_note: String,
    msme_amt: String,
    msme_inv_date: NaiveDate,
    msme_pay_date: NaiveDate,
    msme_result: String,
    grat_sal: String,
    grat_years: String,
    grat_result: String,
    pen_filing_type: String,
    pen_delay: String,
    pen_result: String,

    status_msg: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_v9.db").expect("DB Fail");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clients (id INTEGER PRIMARY KEY, name TEXT, city TEXT, trust INTEGER);
             CREATE TABLE IF NOT EXISTS evidence (id INTEGER PRIMARY KEY, client TEXT, note TEXT, hash TEXT, date TEXT);"
        ).ok();
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_source_theme(&cc.egui_ctx);
        
        let mut risk_map = HashMap::new();
        risk_map.insert("Pune".into(), 72);
        risk_map.insert("Mumbai".into(), 55);
        risk_map.insert("Bangalore".into(), 65);

        let mut app = Self {
            db: Arc::new(Mutex::new(Self::init_db())),
            current_page: Page::Dashboard,
            active_tool: ActiveTool::None,
            client_count: 0,
            evidence_count: 0,
            risk_city: "Pune".to_owned(),
            risk_data: risk_map,
            new_client_name: String::new(),
            new_client_city: "Pune".to_owned(),
            ev_client_name: String::new(),
            ev_note: String::new(),
            
            // Tool Init
            mca_city: "Pune".to_owned(),
            mca_form: "AOC-4".to_owned(),
            mca_result: String::new(),
            board_text: String::new(),
            board_result: Vec::new(),
            trust_gst: String::new(),
            trust_bank: String::new(),
            trust_result: String::new(),
            reg_id: String::new(),
            reg_note: String::new(),
            msme_amt: String::new(),
            msme_inv_date: Local::now().date_naive(),
            msme_pay_date: Local::now().date_naive(),
            msme_result: String::new(),
            grat_sal: String::new(),
            grat_years: String::new(),
            grat_result: String::new(),
            pen_filing_type: "AOC-4".to_owned(),
            pen_delay: "0".to_owned(),
            pen_result: String::new(),
            status_msg: "System Online".to_owned(),
        };
        app.update_counts();
        app
    }

    fn update_counts(&mut self) {
        let conn = self.db.lock().unwrap();
        self.client_count = conn.query_row("SELECT COUNT(*) FROM clients", [], |r| r.get(0)).unwrap_or(0);
        self.evidence_count = conn.query_row("SELECT COUNT(*) FROM evidence", [], |r| r.get(0)).unwrap_or(0);
    }

    fn add_client(&mut self) {
        if self.new_client_name.is_empty() { return; }
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust) VALUES (?1, ?2, ?3)", 
            params![self.new_client_name, self.new_client_city, 90]).ok();
        self.new_client_name.clear();
        drop(conn);
        self.update_counts();
        self.status_msg = "Client Saved".to_owned();
    }

    fn save_evidence(&mut self) {
        if self.ev_client_name.is_empty() { return; }
        let now = Local::now().to_rfc3339();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.ev_client_name, self.ev_note, now));
        let hash = hex::encode(hasher.finalize());

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO evidence (client, note, hash, date) VALUES (?1, ?2, ?3, ?4)",
            params![self.ev_client_name, self.ev_note, hash, now]).ok();
        drop(conn);
        self.update_counts();
        self.status_msg = "Evidence Locked & Hashed".to_owned();
    }

    // --- FIXED CALCULATORS ---
    fn calc_mca(&mut self) {
        // Logic: Start with base risk, adjust based on City AND Form
        let mut score = 90;
        if self.mca_city == "Pune" { score -= 15; }
        if self.mca_city == "Bangalore" { score -= 10; }
        
        // FIX: Now using mca_form to influence the result
        if self.mca_form == "AOC-4" { score -= 5; } // Financials are scrutinized more
        if self.mca_form == "MGT-7" { score += 5; } // Annual returns slightly safer

        let risk_label = if score > 80 { "Low Risk" } else { "High Scrutiny Risk" };
        self.mca_result = format!("Acceptance Probability: {}% ({})", score, risk_label);
    }

    fn calc_board_risk(&mut self) {
        self.board_result.clear();
        let t = self.board_text.to_lowercase();
        if t.contains("loan") { self.board_result.push("Sec 185 Alert: Loan to Director".to_owned()); }
        if t.contains("sell") { self.board_result.push("Sec 188 Alert: Related Party".to_owned()); }
        if self.board_result.is_empty() { self.board_result.push("No obvious risks detected.".to_owned()); }
    }

    fn calc_trust(&mut self) {
        let g = self.trust_gst.parse::<f64>().unwrap_or(0.0);
        let b = self.trust_bank.parse::<f64>().unwrap_or(0.0);
        if b > 0.0 {
            let score = 100.0 - ((g - b).abs() / b * 100.0);
            self.trust_result = format!("{:.0}/100", score.clamp(0.0, 100.0));
        }
    }

    fn calc_regulator(&mut self) {
        self.reg_note = format!("Looking up intelligence for ID: {}...", self.reg_id);
    }

    fn calc_msme(&mut self) {
        let amt = self.msme_amt.parse::<f64>().unwrap_or(0.0);
        let days = (self.msme_pay_date - self.msme_inv_date).num_days();
        if days > 45 {
             let interest = amt * 0.18 * ((days - 15) as f64 / 365.0);
             self.msme_result = format!("Non-Compliant. Liability: ₹{:.2}", interest);
        } else {
             self.msme_result = "Compliant".to_owned();
        }
    }

    fn calc_gratuity(&mut self) {
        let sal = self.grat_sal.parse::<f64>().unwrap_or(0.0);
        let yrs = self.grat_years.parse::<f64>().unwrap_or(0.0);
        self.grat_result = format!("Payable: ₹{:.0}", sal * (15.0/26.0) * yrs);
    }

    fn calc_penalty(&mut self) {
        let days = self.pen_delay.parse::<i32>().unwrap_or(0);
        let rate = if self.pen_filing_type == "AOC-4" { 100 } else { 200 };
        self.pen_result = format!("Fee: ₹{}", days * rate);
    }
}

// ============================================================================
//  4. RENDERERS
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nav").exact_width(200.0).resizable(false).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("PRATYAKSH").size(20.0).strong().color(COLOR_TEXT));
            ui.label(egui::RichText::new("V9.0 ENTERPRISE").size(10.0).color(COLOR_ACCENT));
            ui.add_space(30.0);
            
            if nav_btn(ui, "Dashboard", ICON_GRID, self.current_page == Page::Dashboard).clicked() { self.current_page = Page::Dashboard; self.active_tool = ActiveTool::None; }
            ui.add_space(5.0);
            if nav_btn(ui, "Core Modules", ICON_SHIELD, self.current_page == Page::Modules).clicked() { self.current_page = Page::Modules; self.active_tool = ActiveTool::None; }
            ui.add_space(5.0);
            if nav_btn(ui, "Smart Tools", ICON_CALC, self.current_page == Page::Tools).clicked() { self.current_page = Page::Tools; }
            ui.add_space(5.0);
            if nav_btn(ui, "Settings", ICON_SETTINGS, self.current_page == Page::Settings).clicked() { self.current_page = Page::Settings; self.active_tool = ActiveTool::None; }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(20.0);
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Dashboard => self.render_dashboard(ui),
                Page::Modules => self.render_modules(ui),
                Page::Tools => self.render_tools(ui),
                Page::Settings => { ui.heading("Configuration"); ui.label("Database Path: ./pratyaksh_v9.db"); }
            }
        });
    }
}

impl PratyakshApp {
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Executive Dashboard");
        ui.add_space(20.0);
        
        ui.columns(2, |cols| {
            stat_card(&mut cols[0], "Total Clients", &self.client_count.to_string());
            stat_card(&mut cols[1], "Evidence Logs", &self.evidence_count.to_string());
        });
        ui.add_space(20.0);
        ui.heading("Quick Actions");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Client Name:");
            ui.text_edit_singleline(&mut self.new_client_name);
            if ui.button("Quick Add").clicked() { self.add_client(); }
        });
    }

    fn render_modules(&mut self, ui: &mut egui::Ui) {
        ui.heading("Core Modules");
        ui.add_space(20.0);
        
        egui::Grid::new("mods").spacing([20.0, 20.0]).show(ui, |ui| {
            ui.group(|ui| {
                ui.heading("City Risk Engine");
                ui.horizontal(|ui| {
                    ui.label("City:");
                    ui.text_edit_singleline(&mut self.risk_city);
                });
                if let Some(score) = self.risk_data.get(&self.risk_city) {
                    ui.label(format!("Risk Score: {}%", score));
                } else {
                    ui.label("Risk Data: Unknown");
                }
            });
            
            ui.group(|ui| {
                ui.heading("Evidence Locker");
                ui.text_edit_singleline(&mut self.ev_client_name);
                ui.text_edit_singleline(&mut self.ev_note);
                if ui.button("Lock Evidence").clicked() { self.save_evidence(); }
            });
            ui.end_row();
        });
    }

    fn render_tools(&mut self, ui: &mut egui::Ui) {
        if self.active_tool == ActiveTool::None {
            ui.heading("Select a Tool");
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button("MCA Predictor").clicked() { self.active_tool = ActiveTool::McaPredictor; }
                if ui.button("Board Risk").clicked() { self.active_tool = ActiveTool::BoardRisk; }
                if ui.button("Trust Score").clicked() { self.active_tool = ActiveTool::TrustScore; }
                if ui.button("Regulator Info").clicked() { self.active_tool = ActiveTool::RegulatorNotes; }
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("MSME Calc").clicked() { self.active_tool = ActiveTool::MsmeCalc; }
                if ui.button("Gratuity Calc").clicked() { self.active_tool = ActiveTool::GratuityCalc; }
                if ui.button("Penalty Calc").clicked() { self.active_tool = ActiveTool::PenaltyCalc; }
            });
        } else {
            if ui.button("← Back").clicked() { self.active_tool = ActiveTool::None; }
            ui.separator();
            self.render_active_tool(ui);
        }
    }

    fn render_active_tool(&mut self, ui: &mut egui::Ui) {
        match self.active_tool {
            ActiveTool::McaPredictor => {
                ui.heading("MCA Predictor");
                ui.horizontal(|ui| {
                    ui.label("City:"); ui.text_edit_singleline(&mut self.mca_city);
                    ui.label("Form:"); ui.text_edit_singleline(&mut self.mca_form);
                });
                if ui.button("Predict").clicked() { self.calc_mca(); }
                ui.label(&self.mca_result);
            },
            ActiveTool::BoardRisk => {
                ui.heading("Board Risk");
                ui.text_edit_multiline(&mut self.board_text);
                if ui.button("Scan").clicked() { self.calc_board_risk(); }
                for r in &self.board_result { ui.label(r); }
            },
            ActiveTool::TrustScore => {
                ui.heading("Trust Score");
                ui.horizontal(|ui| {
                    ui.label("GST Turnover:"); ui.text_edit_singleline(&mut self.trust_gst);
                    ui.label("Bank Credits:"); ui.text_edit_singleline(&mut self.trust_bank);
                });
                if ui.button("Calc").clicked() { self.calc_trust(); }
                ui.heading(egui::RichText::new(&self.trust_result).size(40.0));
            },
            ActiveTool::RegulatorNotes => {
                ui.heading("Regulator Notes");
                ui.text_edit_singleline(&mut self.reg_id);
                if ui.button("Search").clicked() { self.calc_regulator(); }
                ui.label(&self.reg_note);
            },
            ActiveTool::MsmeCalc => {
                ui.heading("MSME 43B(h)");
                ui.horizontal(|ui| {
                    ui.label("Amt:"); ui.text_edit_singleline(&mut self.msme_amt);
                    ui.label("Inv Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_inv_date));
                    ui.label("Pay Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_pay_date));
                });
                if ui.button("Check Compliance").clicked() { self.calc_msme(); }
                ui.label(egui::RichText::new(&self.msme_result).color(COLOR_ACCENT));
            },
            ActiveTool::GratuityCalc => {
                ui.heading("Gratuity");
                ui.horizontal(|ui| {
                    ui.label("Basic + DA:"); ui.text_edit_singleline(&mut self.grat_sal);
                    ui.label("Years:"); ui.text_edit_singleline(&mut self.grat_years);
                });
                if ui.button("Calc").clicked() { self.calc_gratuity(); }
                ui.label(egui::RichText::new(&self.grat_result).strong().color(egui::Color32::GREEN));
            },
            ActiveTool::PenaltyCalc => {
                ui.heading("Penalty");
                ui.horizontal(|ui| {
                    ui.label("Days Delayed:"); ui.text_edit_singleline(&mut self.pen_delay);
                    ui.label("Form:"); ui.text_edit_singleline(&mut self.pen_filing_type);
                });
                if ui.button("Calc").clicked() { self.calc_penalty(); }
                ui.label(egui::RichText::new(&self.pen_result).strong().color(egui::Color32::RED));
            },
            _ => {}
        }
    }
}

// --- HELPERS ---
fn nav_btn(ui: &mut egui::Ui, text: &str, icon: &'static [u8], active: bool) -> egui::Response {
    let bg = if active { COLOR_ACCENT } else { egui::Color32::TRANSPARENT };
    let fg = if active { egui::Color32::BLACK } else { COLOR_MUTED };
    
    egui::Frame::none().fill(bg).rounding(4.0).inner_margin(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon).max_width(16.0).tint(fg));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(fg));
        });
    }).response.interact(egui::Sense::click())
}

fn stat_card(ui: &mut egui::Ui, label: &str, val: &str) {
    egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(25, 25, 25)).stroke(egui::Stroke::NONE).inner_margin(15.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(egui::RichText::new(label).size(12.0).color(COLOR_MUTED));
        ui.heading(egui::RichText::new(val).size(24.0).color(COLOR_TEXT));
    });
}

fn setup_source_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = COLOR_BG;
    visuals.panel_fill = COLOR_BG;
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]).with_title("PratyakshAI Ultimate Suite"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}