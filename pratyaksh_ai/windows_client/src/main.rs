#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::Local;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS: CORPORATE SVG ICONS
// ============================================================================
const ICON_CITY: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/></svg>"##.as_bytes();
const ICON_CHART: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/></svg>"##.as_bytes();
const ICON_BRAIN: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#B0BEC5"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>"##.as_bytes();
const ICON_TRASH: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FF5252"><path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>"##.as_bytes();

// ============================================================================
//  2. DATABASE MODELS
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

#[derive(Debug, Clone)]
struct EvidenceLog {
    id: i32,
    client_name: String,
    action: String,
    timestamp: String,
    hash: String,
}

#[derive(Debug, Clone)]
struct RegulatorNote {
    id: i32,
    officer_id: String,
    content: String,
}

#[derive(PartialEq, Clone, Copy)]
enum Page {
    CityRisk,       // Module 1
    ClientIntegrity,// Module 2
    LegalDocs,      // Module 3
    EvidenceLocker, // Module 4
    FirmOps,        // Module 5
    SmartTools      // Module 6
}

// ============================================================================
//  3. APPLICATION STATE
// ============================================================================

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    
    // --- MODULE 1: City Risk ---
    selected_city: String,
    selected_dept: String,
    risk_data: HashMap<String, (i32, String, String)>, // Score, PenaltyRange, Strictness

    // --- MODULE 2: Client Integrity ---
    clients: Vec<Client>,
    new_client_name: String,
    new_client_city: String,

    // --- MODULE 3: Legal ---
    mca_filing_type: String,
    mca_city: String,
    mca_prediction: Option<(i32, String)>, // Probability, Risk Msg

    // --- MODULE 4: Evidence ---
    evidence_logs: Vec<EvidenceLog>,
    evidence_action: String,
    evidence_client_select: String,

    // --- MODULE 5: Firm Ops ---
    billing_service: String,
    billing_city: String,
    billing_estimate: f64,
    time_leakage_data: Vec<(String, f32)>, // Label, Percentage

    // --- MODULE 6: Smart Tools ---
    reg_officer_id: String,
    reg_note_content: String,
    saved_notes: Vec<RegulatorNote>,

    status_msg: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_mega_v6.db").expect("Failed to open DB");
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
            );
            CREATE TABLE IF NOT EXISTS reg_notes (
                id INTEGER PRIMARY KEY,
                officer_id TEXT NOT NULL,
                content TEXT NOT NULL
            );"
        ).expect("Failed to init tables");
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_corporate_theme(&cc.egui_ctx);
        
        let mut app = Self {
            db: Arc::new(Mutex::new(Self::init_db())),
            current_page: Page::CityRisk,
            
            // Defaults
            selected_city: "Pune".to_string(),
            selected_dept: "GST".to_string(),
            risk_data: HashMap::new(),
            clients: Vec::new(),
            new_client_name: String::new(),
            new_client_city: String::new(),
            mca_filing_type: "MGT-7".to_string(),
            mca_city: "Ahmedabad".to_string(),
            mca_prediction: None,
            evidence_logs: Vec::new(),
            evidence_action: "Advice: GST Reversal".to_string(),
            evidence_client_select: String::new(),
            billing_service: "GST Annual Return".to_string(),
            billing_city: "Jaipur".to_string(),
            billing_estimate: 0.0,
            time_leakage_data: vec![
                ("Filing Work".into(), 0.45),
                ("Follow-ups".into(), 0.25),
                ("Local Office".into(), 0.15),
                ("Non-billable".into(), 0.15),
            ],
            reg_officer_id: "AO-PUNE-07".to_string(),
            reg_note_content: String::new(),
            saved_notes: Vec::new(),
            status_msg: "System Online".to_string(),
        };
        
        // Seed Deterministic Data
        app.risk_data.insert("Pune".into(), (72, "₹50k-₹2.4L".into(), "High".into()));
        app.risk_data.insert("Mumbai".into(), (55, "₹20k-₹1.0L".into(), "Medium".into()));
        app.risk_data.insert("Bangalore".into(), (65, "₹40k-₹2.0L".into(), "High".into()));
        app.risk_data.insert("Jaipur".into(), (80, "₹60k-₹3.0L".into(), "Very High".into()));

        app.refresh_data();
        app
    }

    fn refresh_data(&mut self) {
        let conn = self.db.lock().unwrap();
        
        // Clients
        let mut stmt = conn.prepare("SELECT id, name, city, trust_score, flags FROM clients ORDER BY id DESC").unwrap();
        self.clients = stmt.query_map([], |row| Ok(Client {
            id: row.get(0)?, name: row.get(1)?, city: row.get(2)?, trust_score: row.get(3)?, flags: row.get(4)?
        })).unwrap().map(|c| c.unwrap()).collect();

        // Evidence
        let mut stmt = conn.prepare("SELECT id, client_name, action, timestamp, hash FROM evidence ORDER BY id DESC").unwrap();
        self.evidence_logs = stmt.query_map([], |row| Ok(EvidenceLog {
            id: row.get(0)?, client_name: row.get(1)?, action: row.get(2)?, timestamp: row.get(3)?, hash: row.get(4)?
        })).unwrap().map(|e| e.unwrap()).collect();

        // Notes
        let mut stmt = conn.prepare("SELECT id, officer_id, content FROM reg_notes ORDER BY id DESC").unwrap();
        self.saved_notes = stmt.query_map([], |row| Ok(RegulatorNote {
            id: row.get(0)?, officer_id: row.get(1)?, content: row.get(2)?
        })).unwrap().map(|n| n.unwrap()).collect();
    }

    // --- ACTIONS ---

    fn add_client(&mut self) {
        let base = 100;
        let penalty = match self.new_client_city.as_str() {
            "Pune" => 20, "Mumbai" => 10, "Jaipur" => 30, _ => 5
        };
        let score = base - penalty;
        let flags = if score < 80 { "Check Bank Recon" } else { "Clean" };

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust_score, flags) VALUES (?1, ?2, ?3, ?4)",
            params![self.new_client_name, self.new_client_city, score, flags]).unwrap();
        self.new_client_name.clear();
        drop(conn);
        self.refresh_data();
        self.status_msg = "Client Added".to_string();
    }

    fn delete_client(&mut self, id: i32) {
        let conn = self.db.lock().unwrap();
        conn.execute("DELETE FROM clients WHERE id = ?1", params![id]).unwrap();
        drop(conn);
        self.refresh_data();
        self.status_msg = "Client Record Deleted".to_string();
    }

    fn predict_mca(&mut self) {
        let prob = match self.mca_city.as_str() {
            "Ahmedabad" => 91, "Pune" => 85, _ => 70
        };
        let risk = if prob > 90 { "Low Risk" } else { "Director DIN Mismatch Risk" };
        self.mca_prediction = Some((prob, risk.to_string()));
    }

    fn lock_evidence(&mut self) {
        if self.evidence_client_select.is_empty() { return; }
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.evidence_client_select, self.evidence_action, now));
        let hash = hex::encode(hasher.finalize());

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO evidence (client_name, action, timestamp, hash) VALUES (?1, ?2, ?3, ?4)",
            params![self.evidence_client_select, self.evidence_action, now, hash]).unwrap();
        drop(conn);
        self.refresh_data();
        self.status_msg = "Evidence Secured on Blockchain".to_string();
    }

    fn calculate_billing(&mut self) {
        let base = match self.billing_service.as_str() {
            "GST Annual Return" => 10000.0, _ => 5000.0
        };
        let multi = match self.billing_city.as_str() {
            "Jaipur" => 1.8, "Mumbai" => 1.5, _ => 1.0
        };
        self.billing_estimate = base * multi;
    }

    fn save_note(&mut self) {
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO reg_notes (officer_id, content) VALUES (?1, ?2)",
            params![self.reg_officer_id, self.reg_note_content]).unwrap();
        self.reg_note_content.clear();
        drop(conn);
        self.refresh_data();
        self.status_msg = "Regulator Note Saved".to_string();
    }
}

// ============================================================================
//  4. UI RENDERER
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(egui::Color32::GREEN));
                ui.label(&self.status_msg);
            });
        });

        egui::SidePanel::left("nav").exact_width(240.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("PRATYAKSH").size(22.0).strong().color(egui::Color32::WHITE));
            ui.label(egui::RichText::new("CS SUITE v6.0").size(11.0).color(egui::Color32::LIGHT_BLUE));
            ui.add_space(30.0);

            let btns = [
                ("City Risk", Page::CityRisk, ICON_CITY),
                ("Client Integrity", Page::ClientIntegrity, ICON_USER),
                ("Legal / MCA", Page::LegalDocs, ICON_DOC),
                ("Evidence Locker", Page::EvidenceLocker, ICON_LOCK),
                ("Firm Ops", Page::FirmOps, ICON_CHART),
                ("Smart Tools", Page::SmartTools, ICON_BRAIN),
            ];

            for (label, page, icon) in btns {
                let active = self.current_page == page;
                if nav_btn(ui, label, icon, active).clicked() { self.current_page = page; }
                ui.add_space(5.0);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(20.0).show(ui, |ui| {
                match self.current_page {
                    Page::CityRisk => self.render_city_risk(ui),
                    Page::ClientIntegrity => self.render_clients(ui),
                    Page::LegalDocs => self.render_legal(ui),
                    Page::EvidenceLocker => self.render_evidence(ui),
                    Page::FirmOps => self.render_ops(ui),
                    Page::SmartTools => self.render_smart(ui),
                }
            });
        });
    }
}

impl PratyakshApp {
    // --- MODULE 1 ---
    fn render_city_risk(&mut self, ui: &mut egui::Ui) {
        ui.heading("City Risk Dashboard");
        ui.separator();
        
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source("city").selected_text(&self.selected_city)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_city, "Pune".into(), "Pune");
                    ui.selectable_value(&mut self.selected_city, "Mumbai".into(), "Mumbai");
                    ui.selectable_value(&mut self.selected_city, "Bangalore".into(), "Bangalore");
                    ui.selectable_value(&mut self.selected_city, "Jaipur".into(), "Jaipur");
                });
            egui::ComboBox::from_id_source("dept").selected_text(&self.selected_dept)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_dept, "GST".into(), "GST");
                    ui.selectable_value(&mut self.selected_dept, "Income Tax".into(), "Income Tax");
                });
        });

        ui.add_space(20.0);
        if let Some((score, penalty, strictness)) = self.risk_data.get(&self.selected_city) {
            let color = if *score > 70 { egui::Color32::RED } else { egui::Color32::YELLOW };
            
            egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(30,35,40)).inner_margin(20.0).show(ui, |ui| {
                ui.label("NOTICE RISK SCORE");
                ui.heading(egui::RichText::new(format!("{}%", score)).size(40.0).color(color).strong());
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(format!("AO Strictness: {}", strictness));
                    ui.add_space(20.0);
                    ui.label(format!("Penalty Range: {}", penalty));
                });
            });
        }
    }

    // --- MODULE 2 ---
    fn render_clients(&mut self, ui: &mut egui::Ui) {
        ui.heading("Client Integrity Analyzer");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.new_client_name).request_focus();
            ui.text_edit_singleline(&mut self.new_client_city);
            if ui.button("Add Client").clicked() { self.add_client(); }
        });

        ui.add_space(20.0);
        egui::Grid::new("client_grid").striped(true).spacing([20.0, 8.0]).show(ui, |ui| {
            ui.strong("Name"); ui.strong("City"); ui.strong("Trust Score"); ui.strong("Flags"); ui.strong("Action");
            ui.end_row();
            
            // Collect IDs to delete to avoid borrowing issues
            let mut to_delete = None;
            
            for client in &self.clients {
                ui.label(&client.name);
                ui.label(&client.city);
                ui.colored_label(if client.trust_score < 70 { egui::Color32::RED } else { egui::Color32::GREEN }, format!("{}", client.trust_score));
                ui.label(&client.flags);
                
                // DELETE BUTTON
                let btn = egui::Button::new(egui::RichText::new("🗑").color(egui::Color32::RED));
                if ui.add(btn).clicked() { to_delete = Some(client.id); }
                ui.end_row();
            }

            if let Some(id) = to_delete { self.delete_client(id); }
        });
    }

    // --- MODULE 3 ---
    fn render_legal(&mut self, ui: &mut egui::Ui) {
        ui.heading("MCA Filing Predictor");
        ui.separator();

        egui::Grid::new("mca_form").spacing([10.0, 10.0]).show(ui, |ui| {
            ui.label("Filing Type:");
            ui.text_edit_singleline(&mut self.mca_filing_type);
            ui.end_row();
            ui.label("City:");
            ui.text_edit_singleline(&mut self.mca_city);
            ui.end_row();
        });

        if ui.button("Predict Outcome").clicked() { self.predict_mca(); }

        if let Some((prob, msg)) = &self.mca_prediction {
            ui.add_space(10.0);
            ui.colored_label(egui::Color32::LIGHT_BLUE, format!("Acceptance Probability: {}%", prob));
            ui.colored_label(egui::Color32::YELLOW, format!("Risk: {}", msg));
        }
    }

    // --- MODULE 4 ---
    fn render_evidence(&mut self, ui: &mut egui::Ui) {
        ui.heading("Evidence Locker");
        ui.separator();

        egui::Grid::new("ev_grid").spacing([10.0, 10.0]).show(ui, |ui| {
            ui.label("Client:");
            egui::ComboBox::from_id_source("ev_c").selected_text(&self.evidence_client_select)
                .show_ui(ui, |ui| {
                     for c in &self.clients { ui.selectable_value(&mut self.evidence_client_select, c.name.clone(), &c.name); }
                });
            ui.end_row();
            ui.label("Advice:");
            ui.text_edit_singleline(&mut self.evidence_action);
            ui.end_row();
        });

        if ui.button("🔒 Lock Evidence").clicked() { self.lock_evidence(); }

        ui.add_space(20.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
             for log in &self.evidence_logs {
                 egui::Frame::group(ui.style()).stroke(egui::Stroke::new(1.0, egui::Color32::GREEN)).inner_margin(8.0).show(ui, |ui| {
                     ui.horizontal(|ui| { ui.strong(&log.client_name); ui.label(&log.timestamp); });
                     ui.label(&log.action);
                     ui.monospace(&log.hash);
                 });
                 ui.add_space(5.0);
             }
        });
    }

    // --- MODULE 5 ---
    fn render_ops(&mut self, ui: &mut egui::Ui) {
        ui.heading("Firm Operations");
        ui.separator();
        
        ui.subheader("Time Leakage Analyzer");
        for (label, val) in &self.time_leakage_data {
            ui.horizontal(|ui| {
                ui.label(label);
                let bar = egui::ProgressBar::new(*val).show_percentage();
                ui.add(bar);
            });
        }
        ui.label(egui::RichText::new("Estimated Loss: ₹1,84,000").color(egui::Color32::RED));
        
        ui.add_space(20.0);
        ui.subheader("Local Billing Optimizer");
        egui::Grid::new("bill_grid").show(ui, |ui| {
            ui.label("Service:"); ui.text_edit_singleline(&mut self.billing_service); ui.end_row();
            ui.label("City:"); ui.text_edit_singleline(&mut self.billing_city); ui.end_row();
        });
        if ui.button("Generate Fee").clicked() { self.calculate_billing(); }
        if self.billing_estimate > 0.0 {
            ui.heading(format!("Suggested: ₹ {:.0}", self.billing_estimate));
        }
    }

    // --- MODULE 6 ---
    fn render_smart(&mut self, ui: &mut egui::Ui) {
        ui.heading("Regulator Notebook");
        ui.separator();
        
        ui.horizontal(|ui| {
             ui.label("Officer ID:");
             ui.text_edit_singleline(&mut self.reg_officer_id);
        });
        ui.text_edit_multiline(&mut self.reg_note_content);
        if ui.button("Save Note").clicked() { self.save_note(); }

        ui.add_space(10.0);
        for note in &self.saved_notes {
            ui.group(|ui| {
                ui.strong(&note.officer_id);
                ui.label(&note.content);
            });
        }
    }
}

// ============================================================================
//  5. HELPERS
// ============================================================================

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

fn setup_corporate_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = egui::Color32::from_rgb(18, 22, 28);
    visuals.panel_fill = egui::Color32::from_rgb(25, 30, 36);
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 100, 200);
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1300.0, 850.0]).with_title("PratyakshAI Enterprise Suite"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}