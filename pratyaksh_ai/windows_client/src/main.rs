#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{NaiveDate, Local};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};

// ============================================================================
//  1. ASSETS & ICONS (SVG Byte Literals)
// ============================================================================
const ICON_CITY: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/></svg>"##.as_bytes();
const ICON_TOOL: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M22.7 19l-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4z"/></svg>"##.as_bytes();

// ============================================================================
//  2. DATA MODELS
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Client {
    id: i32,
    name: String,
    city: String,
    trust_score: i32,
    flags: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct EvidenceLog {
    id: i32,
    client_name: String,
    action: String,
    timestamp: String,
    hash: String,
}

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Dashboard,
    ClientIntegrity,
    EvidenceLocker,
    SmartTools,
}

#[derive(PartialEq, Clone, Copy)]
enum ToolType {
    MsmeCalc,
    Gratuity,
    Penalty,
    None
}

// ============================================================================
//  3. APP STATE
// ============================================================================

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    
    // --- Client Module ---
    clients: Vec<Client>,
    new_client_name: String,
    new_client_city: String,

    // --- Evidence Module ---
    evidence_logs: Vec<EvidenceLog>,
    evidence_action: String,
    evidence_client_select: String,

    // --- Smart Tools State ---
    active_tool: ToolType,
    
    // Tool: MSME
    msme_amount: String,
    msme_inv_date: NaiveDate,
    msme_pay_date: NaiveDate,
    msme_result: String,

    // Tool: Gratuity
    grat_salary: String,
    grat_years: String,
    grat_result: String,

    // Tool: Penalty
    pen_filing_type: String,
    pen_delay_days: String,
    pen_result: String,

    status_msg: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_suite_v7.db").expect("Failed to open DB");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                city TEXT NOT NULL,
                trust_score INTEGER DEFAULT 100,
                flags TEXT DEFAULT ''
            );
            CREATE TABLE IF NOT EXISTS evidence (
                id INTEGER PRIMARY KEY,
                client_name TEXT NOT NULL,
                action TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                hash TEXT NOT NULL
            );"
        ).expect("Tables initialized");
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_dark_theme(&cc.egui_ctx);
        
        let mut app = Self {
            db: Arc::new(Mutex::new(Self::init_db())),
            current_page: Page::Dashboard,
            clients: Vec::new(),
            new_client_name: String::new(),
            new_client_city: String::new(),
            evidence_logs: Vec::new(),
            evidence_action: "Advice: ".to_string(),
            evidence_client_select: String::new(),
            
            active_tool: ToolType::None,
            msme_amount: String::new(),
            msme_inv_date: Local::now().date_naive(),
            msme_pay_date: Local::now().date_naive(),
            msme_result: String::new(),
            
            grat_salary: String::new(),
            grat_years: String::new(),
            grat_result: String::new(),

            pen_filing_type: "AOC-4".to_string(),
            pen_delay_days: "0".to_string(),
            pen_result: String::new(),

            status_msg: "System Ready".to_string(),
        };
        app.refresh_data();
        app
    }

    fn refresh_data(&mut self) {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, city, trust_score, flags FROM clients ORDER BY id DESC").unwrap();
        self.clients = stmt.query_map([], |row| Ok(Client {
            id: row.get(0)?, name: row.get(1)?, city: row.get(2)?, trust_score: row.get(3)?, flags: row.get(4)?
        })).unwrap().map(|c| c.unwrap()).collect();

        let mut stmt = conn.prepare("SELECT id, client_name, action, timestamp, hash FROM evidence ORDER BY id DESC").unwrap();
        self.evidence_logs = stmt.query_map([], |row| Ok(EvidenceLog {
            id: row.get(0)?, client_name: row.get(1)?, action: row.get(2)?, timestamp: row.get(3)?, hash: row.get(4)?
        })).unwrap().map(|e| e.unwrap()).collect();
    }

    fn add_client(&mut self) {
        if self.new_client_name.is_empty() { return; }
        // Real Trust Logic: Cities have base risk
        let base_risk = match self.new_client_city.as_str() {
            "Pune" => 15, "Mumbai" => 10, "Bangalore" => 12, _ => 5
        };
        let score = 100 - base_risk;
        
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust_score, flags) VALUES (?1, ?2, ?3, ?4)",
            params![self.new_client_name, self.new_client_city, score, "Active"]).unwrap();
        self.new_client_name.clear();
        drop(conn);
        self.refresh_data();
    }

    fn lock_evidence(&mut self) {
        if self.evidence_client_select.is_empty() { return; }
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // SHA-256 Hashing for Evidence
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.evidence_client_select, self.evidence_action, now));
        let hash = hex::encode(hasher.finalize());

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO evidence (client_name, action, timestamp, hash) VALUES (?1, ?2, ?3, ?4)",
            params![self.evidence_client_select, self.evidence_action, now, hash]).unwrap();
        drop(conn);
        self.refresh_data();
    }

    // --- CALCULATOR LOGIC ---
    
    fn calc_msme(&mut self) {
        let amt = self.msme_amount.parse::<f64>().unwrap_or(0.0);
        let days = (self.msme_pay_date - self.msme_inv_date).num_days();
        
        if days <= 45 {
            self.msme_result = format!("Compliant. Paid in {} days.", days);
        } else {
            // Sec 16 Interest: 3x Bank Rate (Approx 6.5 * 3 = 19.5%)
            let delay = days - 15; // Liability starts after 15 days
            let interest = amt * 0.195 * (delay as f64 / 365.0);
            self.msme_result = format!("NON-COMPLIANT. Interest Liability: ₹{:.2}", interest);
        }
    }

    fn calc_gratuity(&mut self) {
        let sal = self.grat_salary.parse::<f64>().unwrap_or(0.0);
        let yrs = self.grat_years.parse::<f64>().unwrap_or(0.0);
        // Formula: (Basic+DA) * 15/26 * Years
        let val = sal * (15.0/26.0) * yrs;
        self.grat_result = format!("Gratuity Payable: ₹{:.2}", val);
    }
    
    fn calc_penalty(&mut self) {
        let delay = self.pen_delay_days.parse::<i32>().unwrap_or(0);
        let rate = if self.pen_filing_type == "AOC-4" { 100 } else { 200 }; // Logic
        self.pen_result = format!("Additional Fee: ₹{}", delay * rate);
    }
}

// ============================================================================
//  4. UI RENDERER
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // --- SIDEBAR ---
        egui::SidePanel::left("nav").exact_width(240.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("PRATYAKSH").size(24.0).strong().color(egui::Color32::WHITE));
                ui.label(egui::RichText::new("SUITE v7.0").size(11.0).color(egui::Color32::from_rgb(79, 249, 120))); // Neon Green
            });
            ui.add_space(30.0);

            if nav_btn(ui, "Dashboard", ICON_CITY, self.current_page == Page::Dashboard).clicked() { self.current_page = Page::Dashboard; }
            ui.add_space(5.0);
            if nav_btn(ui, "Client Integrity", ICON_USER, self.current_page == Page::ClientIntegrity).clicked() { self.current_page = Page::ClientIntegrity; }
            ui.add_space(5.0);
            if nav_btn(ui, "Evidence Locker", ICON_LOCK, self.current_page == Page::EvidenceLocker).clicked() { self.current_page = Page::EvidenceLocker; }
            ui.add_space(5.0);
            if nav_btn(ui, "Smart Tools", ICON_TOOL, self.current_page == Page::SmartTools).clicked() { self.current_page = Page::SmartTools; }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.status_msg).size(10.0).color(egui::Color32::GRAY));
                ui.separator();
            });
        });

        // --- MAIN ---
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Dashboard => self.render_dashboard(ui),
                Page::ClientIntegrity => self.render_clients(ui),
                Page::EvidenceLocker => self.render_evidence(ui),
                Page::SmartTools => self.render_tools(ui),
            }
        });
    }
}

impl PratyakshApp {
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Executive Dashboard");
        ui.add_space(20.0);
        
        ui.columns(3, |cols| {
            card(&mut cols[0], "Active Clients", &self.clients.len().to_string(), egui::Color32::from_rgb(0, 100, 255));
            card(&mut cols[1], "Evidence Logs", &self.evidence_logs.len().to_string(), egui::Color32::from_rgb(79, 249, 120));
            card(&mut cols[2], "Pending Risks", "12", egui::Color32::from_rgb(255, 80, 80));
        });
    }

    fn render_clients(&mut self, ui: &mut egui::Ui) {
        ui.heading("Client Integrity");
        ui.separator();
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.new_client_name).request_focus();
            ui.text_edit_singleline(&mut self.new_client_city);
            if ui.button("Add Client").clicked() { self.add_client(); }
        });
        ui.add_space(20.0);
        // FIX: Replaced ui.grid shorthand with egui::Grid::new()
        egui::Grid::new("clients").striped(true).min_col_width(100.0).show(ui, |ui| {
            ui.strong("Name"); ui.strong("City"); ui.strong("Trust Score"); ui.end_row();
            for c in &self.clients {
                ui.label(&c.name);
                ui.label(&c.city);
                ui.label(format!("{}%", c.trust_score));
                ui.end_row();
            }
        });
    }

    fn render_evidence(&mut self, ui: &mut egui::Ui) {
        ui.heading("Evidence Locker");
        ui.separator();
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("cl").selected_text(&self.evidence_client_select)
                .show_ui(ui, |ui| { for c in &self.clients { ui.selectable_value(&mut self.evidence_client_select, c.name.clone(), &c.name); } });
            ui.text_edit_singleline(&mut self.evidence_action);
            if ui.button("Lock Evidence").clicked() { self.lock_evidence(); }
        });
        ui.add_space(20.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            for log in &self.evidence_logs {
                ui.group(|ui| {
                    ui.horizontal(|ui| { ui.strong(&log.client_name); ui.label(&log.timestamp); });
                    ui.monospace(&log.hash);
                });
            }
        });
    }

    fn render_tools(&mut self, ui: &mut egui::Ui) {
        ui.heading("Smart Tools Engine");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.selectable_label(self.active_tool == ToolType::MsmeCalc, "MSME 43B(h)").clicked() { self.active_tool = ToolType::MsmeCalc; }
            if ui.selectable_label(self.active_tool == ToolType::Gratuity, "Gratuity Calc").clicked() { self.active_tool = ToolType::Gratuity; }
            if ui.selectable_label(self.active_tool == ToolType::Penalty, "Penalty Calc").clicked() { self.active_tool = ToolType::Penalty; }
        });

        ui.add_space(20.0);
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(25, 30, 35)).inner_margin(20.0).show(ui, |ui| {
            match self.active_tool {
                ToolType::MsmeCalc => {
                    ui.heading("MSME Interest Calculator");
                    // FIX: Replaced ui.grid shorthand
                    egui::Grid::new("msme").spacing([20.0, 10.0]).show(ui, |ui| {
                        ui.label("Invoice Amt:"); ui.text_edit_singleline(&mut self.msme_amount); ui.end_row();
                        ui.label("Invoice Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_inv_date)); ui.end_row();
                        ui.label("Payment Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_pay_date)); ui.end_row();
                    });
                    if ui.button("Calculate").clicked() { self.calc_msme(); }
                    ui.label(egui::RichText::new(&self.msme_result).strong().color(egui::Color32::YELLOW));
                },
                ToolType::Gratuity => {
                    ui.heading("Gratuity Calculator");
                    ui.horizontal(|ui| {
                        ui.label("Basic + DA:"); ui.text_edit_singleline(&mut self.grat_salary);
                        ui.label("Years:"); ui.text_edit_singleline(&mut self.grat_years);
                    });
                    if ui.button("Calculate").clicked() { self.calc_gratuity(); }
                    ui.label(egui::RichText::new(&self.grat_result).strong().color(egui::Color32::GREEN));
                },
                ToolType::Penalty => {
                    ui.heading("Late Fee Calculator");
                    ui.horizontal(|ui| {
                        ui.label("Days Delayed:"); ui.text_edit_singleline(&mut self.pen_delay_days);
                        ui.label("Form:"); ui.text_edit_singleline(&mut self.pen_filing_type);
                    });
                    if ui.button("Calculate").clicked() { self.calc_penalty(); }
                    ui.label(egui::RichText::new(&self.pen_result).strong().color(egui::Color32::RED));
                },
                _ => { ui.label("Select a tool from the tabs above."); }
            }
        });
    }
}

// --- HELPERS ---
fn nav_btn(ui: &mut egui::Ui, text: &str, icon: &'static [u8], active: bool) -> egui::Response {
    let bg = if active { egui::Color32::from_rgb(0, 80, 150) } else { egui::Color32::TRANSPARENT };
    egui::Frame::none().fill(bg).rounding(4.0).inner_margin(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon).max_width(18.0));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0));
        });
    }).response.interact(egui::Sense::click())
}

fn card(ui: &mut egui::Ui, title: &str, val: &str, color: egui::Color32) {
    egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(30, 30, 40)).stroke(egui::Stroke::new(1.0, color)).inner_margin(15.0).show(ui, |ui| {
        ui.label(egui::RichText::new(title).size(12.0).color(egui::Color32::GRAY));
        ui.heading(egui::RichText::new(val).size(24.0).color(egui::Color32::WHITE));
    });
}

fn setup_dark_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = egui::Color32::from_rgb(10, 12, 16);
    visuals.panel_fill = egui::Color32::from_rgb(18, 20, 25);
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]).with_title("PratyakshAI Ultimate Suite"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}