use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{NaiveDate, Local};
use std::sync::{Arc, Mutex};

// ============================================================================
//  1. ASSETS: MATERIAL DESIGN SVGs (Embedded)
// ============================================================================

const ICON_HOME: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFFFFF"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>"##.as_bytes();
const ICON_RISK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FF5252"><path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z"/></svg>"##.as_bytes();
const ICON_CLIENT: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#448AFF"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>"##.as_bytes();
const ICON_LEGAL: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#FFD740"><path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8.5 7.5c0 .83-.67 1.5-1.5 1.5H9v2H7.5V7H10c.83 0 1.5.67 1.5 1.5v1zm5 2c0 .83-.67 1.5-1.5 1.5h-2.5V7H15c.83 0 1.5.67 1.5 1.5v3zm4-3H19v1h1.5V11H19v2h-1.5V7h3v1.5zM9 9.5h1v-1H9v1zm6 3.5h1v-1h-1v1z"/></svg>"##.as_bytes();
const ICON_OPS: &[u8] = r##"<svg viewBox="0 0 24 24" fill="#69F0AE"><path d="M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 4c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1.4c0-2 4-3.1 6-3.1s6 1.1 6 3.1V19z"/></svg>"##.as_bytes();

// ============================================================================
//  2. REAL DATA MODELS & DATABASE
// ============================================================================

#[derive(Debug, Clone)]
struct Client {
    id: i32,
    name: String,
    city: String,
    trust_score: i32,
    pending_fees: f64,
}

#[derive(PartialEq, Clone, Copy)]
enum Page { Dashboard, RiskTools, ClientIntegrity, LegalOps, FirmOps }

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    input_client_name: String,
    input_client_city: String,
    input_pending_fees: String,
    calc_filing_date: NaiveDate,
    calc_fy_end: NaiveDate,
    calc_penalty_result: i32,
    clients_list: Vec<Client>,
    status_message: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_data.db").expect("Failed to open DB");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                city TEXT NOT NULL,
                trust_score INTEGER DEFAULT 100,
                pending_fees REAL DEFAULT 0.0
            )",
            [],
        ).expect("Failed to create tables");
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_2026_theme(&cc.egui_ctx);
        
        let conn = Self::init_db();
        let mut app = Self {
            db: Arc::new(Mutex::new(conn)),
            current_page: Page::Dashboard,
            input_client_name: String::new(),
            input_client_city: String::new(),
            input_pending_fees: String::new(),
            calc_filing_date: Local::now().date_naive(),
            calc_fy_end: NaiveDate::from_ymd_opt(2023, 3, 31).unwrap(),
            calc_penalty_result: 0,
            clients_list: Vec::new(),
            status_message: "System Ready. Database Connected.".to_string(),
        };
        app.refresh_clients();
        app
    }

    fn refresh_clients(&mut self) {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, city, trust_score, pending_fees FROM clients ORDER BY id DESC").unwrap();
        
        let client_iter = stmt.query_map([], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                city: row.get(2)?,
                trust_score: row.get(3)?,
                pending_fees: row.get(4)?,
            })
        }).unwrap();

        self.clients_list = client_iter.map(|c| c.unwrap()).collect();
    }

    fn add_client(&mut self) {
        if self.input_client_name.is_empty() { return; }
        
        let fees: f64 = self.input_pending_fees.parse().unwrap_or(0.0);
        let trust = 100 - ((fees / 10000.0) as i32 * 10).max(0).min(100);

        let conn = self.db.lock().unwrap();
        conn.execute(
            "INSERT INTO clients (name, city, trust_score, pending_fees) VALUES (?1, ?2, ?3, ?4)",
            params![self.input_client_name, self.input_client_city, trust, fees],
        ).unwrap();
        
        self.status_message = format!("Client '{}' added with Trust Score {}", self.input_client_name, trust);
        self.input_client_name.clear();
        self.input_pending_fees.clear();
        drop(conn);
        self.refresh_clients();
    }

    fn calculate_penalty(&mut self) {
        let days_late = (self.calc_filing_date - self.calc_fy_end).num_days() - 30;
        if days_late <= 0 {
            self.calc_penalty_result = 0;
        } else {
            self.calc_penalty_result = (days_late as i32) * 100;
        }
    }
}

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("sidebar").exact_width(260.0).show(ctx, |ui| {
            ui.add_space(20.0);
            
            ui.vertical_centered(|ui| {
                let logo = egui::Image::new("file://logo.png").max_width(120.0);
                ui.add(logo); 
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("PRATYAKSH AI").size(22.0).strong().color(egui::Color32::from_rgb(0, 191, 255)));
                ui.label(egui::RichText::new("Enterprise 2026").size(12.0).color(egui::Color32::GRAY));
            });
            
            ui.add_space(40.0);

            if nav_btn(ui, "Dashboard", ICON_HOME, self.current_page == Page::Dashboard).clicked() { self.current_page = Page::Dashboard; }
            ui.add_space(8.0);
            if nav_btn(ui, "Risk Engine", ICON_RISK, self.current_page == Page::RiskTools).clicked() { self.current_page = Page::RiskTools; }
            ui.add_space(8.0);
            if nav_btn(ui, "Client Integrity", ICON_CLIENT, self.current_page == Page::ClientIntegrity).clicked() { self.current_page = Page::ClientIntegrity; }
            ui.add_space(8.0);
            if nav_btn(ui, "Legal & Board", ICON_LEGAL, self.current_page == Page::LegalOps).clicked() { self.current_page = Page::LegalOps; }
            ui.add_space(8.0);
            if nav_btn(ui, "Firm Ops", ICON_OPS, self.current_page == Page::FirmOps).clicked() { self.current_page = Page::FirmOps; }
            
            // FIX: Changed Align::Start to Align::Min
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.add_space(20.0);
                ui.separator();
                ui.label(egui::RichText::new(&self.status_message).size(10.0).color(egui::Color32::GREEN));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            match self.current_page {
                Page::Dashboard => self.render_dashboard(ui),
                Page::RiskTools => self.render_risk_tools(ui),
                Page::ClientIntegrity => self.render_client_manager(ui),
                _ => { ui.heading("Module Enabled. Coming in v4.1 Update."); }
            }
        });
    }
}

impl PratyakshApp {
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Executive Summary");
        ui.add_space(20.0);
        
        let total_clients = self.clients_list.len();
        let total_fees: f64 = self.clients_list.iter().map(|c| c.pending_fees).sum();
        let low_trust = self.clients_list.iter().filter(|c| c.trust_score < 50).count();

        ui.columns(3, |cols| {
            metric_card(&mut cols[0], "Active Clients", &total_clients.to_string(), egui::Color32::from_rgb(0, 100, 255));
            metric_card(&mut cols[1], "Pending Revenue", &format!("₹ {:.0}", total_fees), egui::Color32::from_rgb(255, 170, 0));
            metric_card(&mut cols[2], "Risk Alerts", &low_trust.to_string(), egui::Color32::from_rgb(255, 50, 50));
        });
    }

    fn render_risk_tools(&mut self, ui: &mut egui::Ui) {
        ui.heading("Local Risk & Notice Tools");
        ui.label("Modules 1-10: Penalty & Delay Prediction");
        ui.add_space(20.0);

        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(20, 20, 30))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.heading("Penalty Forecast Tool");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("FY End Date:");
                    ui.add(egui_extras::DatePickerButton::new(&mut self.calc_fy_end));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Actual Filing Date:");
                    ui.add(egui_extras::DatePickerButton::new(&mut self.calc_filing_date));
                });

                if ui.button("Calculate Liability").clicked() {
                    self.calculate_penalty();
                }

                if self.calc_penalty_result > 0 {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, format!("Estimated Penalty: ₹ {}", self.calc_penalty_result));
                    ui.small("Based on Companies Act, 2013 Additional Fee Rules");
                }
            });
    }

    fn render_client_manager(&mut self, ui: &mut egui::Ui) {
        ui.heading("Client Integrity Analyzer");
        ui.add_space(10.0);

        egui::CollapsingHeader::new("➕ Add New Client / Transaction").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.input_client_name);
                ui.label("City:");
                ui.text_edit_singleline(&mut self.input_client_city);
            });
            ui.horizontal(|ui| {
                ui.label("Pending Fees (₹):");
                ui.text_edit_singleline(&mut self.input_pending_fees);
                if ui.button("Analyze & Save").clicked() {
                    self.add_client();
                }
            });
        });

        ui.add_space(20.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("client_grid").striped(true).spacing([40.0, 10.0]).show(ui, |ui| {
                ui.label(egui::RichText::new("ID").strong());
                ui.label(egui::RichText::new("Client Name").strong());
                ui.label(egui::RichText::new("City").strong());
                ui.label(egui::RichText::new("Trust Score").strong());
                ui.label(egui::RichText::new("Dues").strong());
                ui.end_row();

                for client in &self.clients_list {
                    ui.label(client.id.to_string());
                    ui.label(&client.name);
                    ui.label(&client.city);
                    
                    let score_color = if client.trust_score > 80 { egui::Color32::GREEN } else { egui::Color32::RED };
                    ui.colored_label(score_color, format!("{} / 100", client.trust_score));
                    
                    ui.label(format!("₹ {}", client.pending_fees));
                    ui.end_row();
                }
            });
        });
    }
}

fn metric_card(ui: &mut egui::Ui, title: &str, value: &str, accent: egui::Color32) {
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgb(15, 18, 25))
        .stroke(egui::Stroke::new(1.0, accent))
        .rounding(10.0)
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(title).size(12.0).color(egui::Color32::GRAY));
            ui.add_space(5.0);
            ui.label(egui::RichText::new(value).size(24.0).strong().color(egui::Color32::WHITE));
        });
}

fn nav_btn(ui: &mut egui::Ui, text: &str, icon_bytes: &'static [u8], active: bool) -> egui::Response {
    let bg = if active { egui::Color32::from_rgb(0, 50, 100) } else { egui::Color32::TRANSPARENT };
    let fg = if active { egui::Color32::WHITE } else { egui::Color32::GRAY };
    
    egui::Frame::none().fill(bg).rounding(8.0).inner_margin(10.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon_bytes).max_width(20.0).tint(fg));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(fg).size(14.0));
        });
    }).response.interact(egui::Sense::click())
}

fn setup_2026_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = egui::Color32::from_rgb(8, 10, 14); 
    visuals.panel_fill = egui::Color32::from_rgb(8, 10, 14);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 22, 30);
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("PratyakshAI Enterprise 2026"),
        ..Default::default()
    };
    eframe::run_native(
        "PratyakshAI",
        options,
        Box::new(|cc| Box::new(PratyakshApp::new(cc))),
    )
}