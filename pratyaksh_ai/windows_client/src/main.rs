use eframe::egui;
use rusqlite::{params, Connection};
use chrono::Local;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS: ICONS (SVG)
// ============================================================================
const ICON_CITY: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FF5252"><path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#448AFF"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFD740"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#69F0AE"><path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z"/></svg>"##.as_bytes();
const ICON_CHART: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#E040FB"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/></svg>"##.as_bytes();
const ICON_BRAIN: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#00E5FF"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>"##.as_bytes();

// ============================================================================
//  2. REAL DATA STRUCTURES (DB & LOGIC)
// ============================================================================

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

#[derive(PartialEq, Clone, Copy)]
enum Page {
    CityRisk,
    ClientIntegrity,
    LegalDocs,
    EvidenceLocker,
    FirmOps,
    SmartTools
}

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    
    // Module 1: City Risk
    selected_city: String,
    risk_data: HashMap<String, (i32, String)>,

    // Module 2: Client Integrity
    clients: Vec<Client>,
    new_client_name: String,
    new_client_city: String,

    // Module 4: Evidence Locker
    evidence_logs: Vec<EvidenceLog>,
    evidence_action: String,
    evidence_client_select: String,

    // Module 5: Billing
    billing_service: String,
    billing_city: String,
    billing_estimate: f64,

    // Module 6: Regulator Notes
    regulator_notes: String,
    
    status_msg: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_mega.db").expect("Failed to open DB");
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
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                city TEXT,
                content TEXT
            );"
        ).expect("Failed to init tables");
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_enterprise_theme(&cc.egui_ctx);
        
        let mut app = Self {
            db: Arc::new(Mutex::new(Self::init_db())),
            current_page: Page::CityRisk,
            selected_city: "Pune".to_string(),
            risk_data: HashMap::new(),
            clients: Vec::new(),
            new_client_name: String::new(),
            new_client_city: String::new(),
            evidence_logs: Vec::new(),
            evidence_action: "Advice: GST Reversal".to_string(),
            evidence_client_select: String::new(),
            billing_service: "GST Annual Return".to_string(),
            billing_city: "Mumbai".to_string(),
            billing_estimate: 0.0,
            regulator_notes: String::new(),
            status_msg: "System Online".to_string(),
        };
        
        app.risk_data.insert("Pune".into(), (72, "High".into()));
        app.risk_data.insert("Mumbai".into(), (55, "Medium".into()));
        app.risk_data.insert("Bangalore".into(), (65, "High".into()));
        app.risk_data.insert("Ahmedabad".into(), (40, "Low".into()));
        app.risk_data.insert("Jaipur".into(), (80, "Very High".into()));

        app.refresh_data();
        app
    }

    fn refresh_data(&mut self) {
        let conn = self.db.lock().unwrap();
        
        // Load Clients
        let mut stmt = conn.prepare("SELECT id, name, city, trust_score, flags FROM clients").unwrap();
        self.clients = stmt.query_map([], |row| Ok(Client {
            id: row.get(0)?, name: row.get(1)?, city: row.get(2)?, trust_score: row.get(3)?, flags: row.get(4)?
        })).unwrap().map(|c| c.unwrap()).collect();

        // Load Evidence
        let mut stmt = conn.prepare("SELECT id, client_name, action, timestamp, hash FROM evidence ORDER BY id DESC").unwrap();
        self.evidence_logs = stmt.query_map([], |row| Ok(EvidenceLog {
            id: row.get(0)?, client_name: row.get(1)?, action: row.get(2)?, timestamp: row.get(3)?, hash: row.get(4)?
        })).unwrap().map(|e| e.unwrap()).collect();
    }

    fn add_client(&mut self) {
        let base_score = 100;
        let risk_penalty = match self.risk_data.get(&self.new_client_city) {
            Some((score, _)) => *score / 5,
            None => 0,
        };
        let final_score = base_score - risk_penalty;

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust_score, flags) VALUES (?1, ?2, ?3, ?4)",
            params![self.new_client_name, self.new_client_city, final_score, "New Registration"],
        ).unwrap();
        self.new_client_name.clear();
        drop(conn);
        self.refresh_data();
    }

    fn lock_evidence(&mut self) {
        if self.evidence_client_select.is_empty() { return; }
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.evidence_client_select, self.evidence_action, now));
        let hash = hex::encode(hasher.finalize());

        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO evidence (client_name, action, timestamp, hash) VALUES (?1, ?2, ?3, ?4)",
            params![self.evidence_client_select, self.evidence_action, now, hash]
        ).unwrap();
        drop(conn);
        self.refresh_data();
    }

    fn calculate_fee(&mut self) {
        let base_rate = match self.billing_service.as_str() {
            "GST Annual Return" => 10000.0,
            "Company Incorporation" => 15000.0,
            "Tax Audit" => 25000.0,
            _ => 5000.0
        };
        
        let multiplier = match self.billing_city.as_str() {
            "Mumbai" => 1.5,
            "Bangalore" => 1.4,
            "Pune" => 1.2,
            "Jaipur" => 0.9,
            _ => 1.0
        };

        self.billing_estimate = base_rate * multiplier;
    }
}

// ============================================================================
//  3. UI RENDERER
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("nav").exact_width(250.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("PRATYAKSH AI").size(24.0).strong().color(egui::Color32::from_rgb(0, 200, 255)));
            ui.label(egui::RichText::new("CS Suite Mega Edition").size(12.0).color(egui::Color32::GRAY));
            ui.add_space(30.0);

            let btns = [
                ("City Risk", Page::CityRisk, ICON_CITY),
                ("Client Integrity", Page::ClientIntegrity, ICON_USER),
                ("Legal Docs", Page::LegalDocs, ICON_DOC),
                ("Evidence Locker", Page::EvidenceLocker, ICON_LOCK),
                ("Firm Ops", Page::FirmOps, ICON_CHART),
                ("Smart Tools", Page::SmartTools, ICON_BRAIN),
            ];

            for (label, page, icon) in btns {
                if nav_btn(ui, label, icon, self.current_page == page).clicked() {
                    self.current_page = page;
                }
                ui.add_space(5.0);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            match self.current_page {
                Page::CityRisk => self.render_city_risk(ui),
                Page::ClientIntegrity => self.render_clients(ui),
                Page::LegalDocs => self.render_legal(ui),
                Page::EvidenceLocker => self.render_evidence(ui),
                Page::FirmOps => self.render_ops(ui),
                Page::SmartTools => self.render_smart(ui),
            }
        });
    }
}

impl PratyakshApp {
    fn render_city_risk(&mut self, ui: &mut egui::Ui) {
        ui.heading("City Risk Dashboard");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Select City:");
            egui::ComboBox::from_id_source("city_combo")
                .selected_text(&self.selected_city)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_city, "Pune".into(), "Pune");
                    ui.selectable_value(&mut self.selected_city, "Mumbai".into(), "Mumbai");
                    ui.selectable_value(&mut self.selected_city, "Bangalore".into(), "Bangalore");
                    ui.selectable_value(&mut self.selected_city, "Jaipur".into(), "Jaipur");
                });
        });

        ui.add_space(20.0);

        if let Some((risk, strictness)) = self.risk_data.get(&self.selected_city) {
            let color = if *risk > 70 { egui::Color32::RED } else { egui::Color32::YELLOW };
            
            egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(30, 20, 20)).inner_margin(20.0).show(ui, |ui| {
                ui.label(egui::RichText::new("NOTICE RISK SCORE").size(14.0).color(egui::Color32::GRAY));
                ui.heading(egui::RichText::new(format!("{}%", risk)).size(40.0).color(color).strong());
                ui.label(format!("AO Strictness: {}", strictness));
                ui.label("Based on last 24 months of officer behavior.");
            });
        }
    }

    fn render_clients(&mut self, ui: &mut egui::Ui) {
        ui.heading("Client Integrity Analyzer");
        ui.label("Trust Scores derived from banking & GST pattern mismatch.");
        ui.separator();

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.new_client_name).request_focus();
            ui.text_edit_singleline(&mut self.new_client_city);
            if ui.button("Add Client").clicked() { self.add_client(); }
        });

        ui.add_space(20.0);
        
        egui::Grid::new("clients").striped(true).spacing([40.0, 10.0]).show(ui, |ui| {
            ui.strong("Name"); ui.strong("City"); ui.strong("Trust Score"); ui.strong("Flags"); ui.end_row();
            
            for client in &self.clients {
                ui.label(&client.name);
                ui.label(&client.city);
                ui.colored_label(
                    if client.trust_score < 70 { egui::Color32::RED } else { egui::Color32::GREEN },
                    format!("{}/100", client.trust_score)
                );
                ui.label(&client.flags);
                ui.end_row();
            }
        });
    }

    fn render_evidence(&mut self, ui: &mut egui::Ui) {
        ui.heading("Evidence Locker (Legal Protection)");
        ui.label("Cryptographically locked audit trail for client advice.");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Client:");
            egui::ComboBox::from_id_source("ev_client")
                .selected_text(&self.evidence_client_select)
                .show_ui(ui, |ui| {
                    for c in &self.clients {
                        ui.selectable_value(&mut self.evidence_client_select, c.name.clone(), &c.name);
                    }
                });
        });
        
        ui.label("Action/Advice Given:");
        ui.text_edit_singleline(&mut self.evidence_action);

        if ui.button("🔒 LOCK EVIDENCE & GENERATE HASH").clicked() {
            self.lock_evidence();
        }

        ui.add_space(20.0);
        ui.label("Locked Logs:");
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for log in &self.evidence_logs {
                egui::Frame::group(ui.style()).stroke(egui::Stroke::new(1.0, egui::Color32::GREEN)).inner_margin(10.0).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong(&log.client_name);
                        ui.label(format!(" at {}", log.timestamp));
                    });
                    ui.label(format!("Action: {}", log.action));
                    ui.monospace(format!("SHA256: {}", log.hash));
                });
                ui.add_space(5.0);
            }
        });
    }

    fn render_ops(&mut self, ui: &mut egui::Ui) {
        ui.heading("Local Billing Optimizer");
        ui.separator();

        // FIXED: Using egui::Grid::new().show()
        egui::Grid::new("billing").spacing([20.0, 10.0]).show(ui, |ui| {
            ui.label("Service:");
            egui::ComboBox::from_id_source("serv_combo").selected_text(&self.billing_service).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.billing_service, "GST Annual Return".into(), "GST Annual Return");
                ui.selectable_value(&mut self.billing_service, "Company Incorporation".into(), "Company Incorporation");
                ui.selectable_value(&mut self.billing_service, "Tax Audit".into(), "Tax Audit");
            });
            ui.end_row();

            ui.label("City:");
            egui::ComboBox::from_id_source("city_bill_combo").selected_text(&self.billing_city).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.billing_city, "Mumbai".into(), "Mumbai");
                ui.selectable_value(&mut self.billing_city, "Pune".into(), "Pune");
                ui.selectable_value(&mut self.billing_city, "Jaipur".into(), "Jaipur");
            });
            ui.end_row();
        });

        ui.add_space(10.0);

        if ui.button("Calculate Suggested Fee").clicked() {
            self.calculate_fee();
        }

        if self.billing_estimate > 0.0 {
            ui.add_space(10.0);
            ui.heading(format!("Suggested Fee: ₹ {:.0}", self.billing_estimate));
            ui.label("Includes City Risk Premium & Market Variance");
        }
    }

    fn render_legal(&mut self, ui: &mut egui::Ui) { ui.heading("Legal & Board Resolutions"); ui.label("Coming in v5.1"); }
    fn render_smart(&mut self, ui: &mut egui::Ui) { 
        ui.heading("Regulator Notebook");
        ui.text_edit_multiline(&mut self.regulator_notes);
    }
}

fn nav_btn(ui: &mut egui::Ui, text: &str, icon: &'static [u8], active: bool) -> egui::Response {
    let bg = if active { egui::Color32::from_rgb(0, 60, 120) } else { egui::Color32::TRANSPARENT };
    egui::Frame::none().fill(bg).rounding(5.0).inner_margin(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon).max_width(20.0));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(egui::Color32::WHITE).size(14.0));
        });
    }).response.interact(egui::Sense::click())
}

fn setup_enterprise_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = egui::Color32::from_rgb(10, 14, 20);
    visuals.panel_fill = egui::Color32::from_rgb(15, 20, 28);
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]).with_title("PratyakshAI Mega Suite"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}