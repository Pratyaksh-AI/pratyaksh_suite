#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{Local, NaiveDate};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS: PROFESSIONAL SVG ICONS (Material Design)
// ============================================================================

const COLOR_ACCENT: egui::Color32 = egui::Color32::from_rgb(79, 249, 120); // Neon Green
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(17, 17, 17);       // Deep Black
const COLOR_TEXT: egui::Color32 = egui::Color32::WHITE;
const COLOR_MUTED: egui::Color32 = egui::Color32::GRAY;

const ICON_GRID: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"##.as_bytes();
const ICON_CITY: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><path d="M3 21h18"/><path d="M5 21V7l8-4 8 4v14"/><path d="M8 21v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"##.as_bytes();
const ICON_CALC: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><rect x="4" y="2" width="16" height="20" rx="2"/><line x1="8" x2="16" y1="6" y2="6"/><line x1="16" x2="16" y1="14" y2="18"/><path d="M12 18h.01"/><path d="M8 18h.01"/></svg>"##.as_bytes();
const ICON_SETTINGS: &[u8] = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"##.as_bytes();

// ============================================================================
//  2. DATA MODELS
// ============================================================================

#[derive(PartialEq, Clone, Copy)]
enum Page {
    LicenseAgreement,
    Dashboard,
    Modules,
    Tools,
    Settings
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum ActiveTool {
    None,
    McaPredictor, BoardRisk, TrustScore, RegulatorNotes,
    MsmeCalc, GratuityCalc, PenaltyCalc,
    TaxRegime, CryptoTax, PmlaScanner, ShellRisk, HraCalc,
    AdvanceTax, LeaseCalc, AngelTax, BuybackTax,
    EsgCheck, UdinValid, AuditRot, NetWorth,
    ExportTrack, PartnerDiss
}

#[derive(Debug, Clone)]
struct Client { id: i32, name: String, city: String, trust: i32 }

#[derive(Debug, Clone)]
struct EvidenceLog { id: i32, client: String, note: String, hash: String, date: String }

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    active_tool: ActiveTool,
    license_accepted: bool,
    
    // Core Data
    client_count: i32,
    evidence_count: i32,
    risk_data: HashMap<String, i32>,

    // --- INPUT STATES ---
    new_client_name: String, new_client_city: String,
    ev_client_name: String, ev_note: String,
    risk_city: String,

    // Tools
    mca_city: String, mca_form: String, mca_result: String,
    board_text: String, board_result: Vec<String>,
    trust_gst: String, trust_bank: String, trust_result: String,
    reg_id: String, reg_note: String,
    
    // Calculators
    msme_amt: String, msme_inv_date: NaiveDate, msme_pay_date: NaiveDate, msme_result: String,
    grat_sal: String, grat_yrs: String, grat_result: String,
    pen_days: String, pen_filing_type: String, pen_result: String,
    
    // New 30+ Tools Inputs
    tax_inc: String, tax_ded: String, tax_result: String,
    cry_prof: String, cry_result: String,
    pmla_amt: String, pmla_cash: bool, pmla_result: String,
    shell_to: String, shell_ast: String, shell_result: String,
    hra_basic: String, hra_rent: String, hra_result: String,
    adv_tax: String, adv_paid: String, adv_result: String,
    lease_pmt: String, lease_rate: String, lease_years: String, lease_result: String,
    angel_issue: String, angel_fmv: String, angel_result: String,
    buy_shares: String, buy_price: String, buy_result: String,
    esg_mcap: String, esg_result: String,
    udin_val: String, udin_result: String,
    audit_years: String, audit_result: String,
    nw_cap: String, nw_res: String, nw_result: String,
    exp_date: NaiveDate, exp_result: String,
    part_ast: String, part_lia: String, part_result: String,

    status_msg: String,
    clients: Vec<Client>,
    evidence_logs: Vec<EvidenceLog>,
    evidence_client_select: String,
    evidence_action: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_v10.db").expect("DB Fail");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clients (id INTEGER PRIMARY KEY, name TEXT, city TEXT, trust INTEGER);
             CREATE TABLE IF NOT EXISTS evidence (id INTEGER PRIMARY KEY, client TEXT, note TEXT, hash TEXT, date TEXT);"
        ).ok();
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_source_theme(&cc.egui_ctx);
        
        let mut risk = HashMap::new();
        risk.insert("Pune".into(), 72); risk.insert("Mumbai".into(), 55);

        let mut app = Self {
            db: Arc::new(Mutex::new(Self::init_db())),
            current_page: Page::LicenseAgreement, 
            license_accepted: false,
            active_tool: ActiveTool::None,
            client_count: 0, evidence_count: 0,
            risk_data: risk, risk_city: "Pune".into(),
            
            // Init Strings
            new_client_name: "".into(), new_client_city: "Pune".into(),
            ev_client_name: "".into(), ev_note: "".into(),
            mca_city: "Pune".into(), mca_form: "AOC-4".into(), mca_result: "".into(),
            board_text: "".into(), board_result: vec![],
            trust_gst: "".into(), trust_bank: "".into(), trust_result: "".into(),
            reg_id: "".into(), reg_note: "".into(),
            msme_amt: "".into(), msme_inv_date: Local::now().date_naive(), msme_pay_date: Local::now().date_naive(), msme_result: "".into(),
            grat_sal: "".into(), grat_yrs: "".into(), grat_result: "".into(),
            pen_days: "".into(), pen_filing_type: "AOC-4".into(), pen_result: "".into(),
            
            // New Tools
            tax_inc: "".into(), tax_ded: "".into(), tax_result: "".into(),
            cry_prof: "".into(), cry_result: "".into(),
            pmla_amt: "".into(), pmla_cash: false, pmla_result: "".into(),
            shell_to: "".into(), shell_ast: "".into(), shell_result: "".into(),
            hra_basic: "".into(), hra_rent: "".into(), hra_result: "".into(),
            adv_tax: "".into(), adv_paid: "".into(), adv_result: "".into(),
            lease_pmt: "".into(), lease_rate: "10".into(), lease_years: "5".into(), lease_result: "".into(),
            angel_issue: "".into(), angel_fmv: "".into(), angel_result: "".into(),
            buy_shares: "".into(), buy_price: "".into(), buy_result: "".into(),
            esg_mcap: "".into(), esg_result: "".into(),
            udin_val: "".into(), udin_result: "".into(),
            audit_years: "".into(), audit_result: "".into(),
            nw_cap: "".into(), nw_res: "".into(), nw_result: "".into(),
            exp_date: Local::now().date_naive(), exp_result: "".into(),
            part_ast: "".into(), part_lia: "".into(), part_result: "".into(),

            status_msg: "Waiting for License Acceptance...".into(),
            clients: vec![],
            evidence_logs: vec![],
            evidence_client_select: "".into(),
            evidence_action: "Advice: ".into(),
        };
        app.refresh_db();
        app
    }

    fn refresh_db(&mut self) {
        let conn = self.db.lock().unwrap();
        self.client_count = conn.query_row("SELECT COUNT(*) FROM clients", [], |r| r.get(0)).unwrap_or(0);
        self.evidence_count = conn.query_row("SELECT COUNT(*) FROM evidence", [], |r| r.get(0)).unwrap_or(0);
        
        let mut stmt = conn.prepare("SELECT id, name, city, trust FROM clients").unwrap();
        self.clients = stmt.query_map([], |row| Ok(Client {
            id: row.get(0)?, name: row.get(1)?, city: row.get(2)?, trust: row.get(3)?
        })).unwrap().map(|c| c.unwrap()).collect();

        let mut stmt = conn.prepare("SELECT id, client, note, hash, date FROM evidence").unwrap();
        self.evidence_logs = stmt.query_map([], |row| Ok(EvidenceLog {
            id: row.get(0)?, client: row.get(1)?, note: row.get(2)?, hash: row.get(3)?, date: row.get(4)?
        })).unwrap().map(|e| e.unwrap()).collect();
    }

    // --- LOGIC IMPLEMENTATIONS ---
    
    fn add_client(&mut self) {
        if self.new_client_name.is_empty() { return; }
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust) VALUES (?1, ?2, ?3)", 
            params![self.new_client_name, self.new_client_city, 90]).ok();
        self.new_client_name.clear();
        drop(conn);
        self.refresh_db();
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
        self.refresh_db();
        self.status_msg = "Evidence Locked & Hashed".to_owned();
    }

    fn calc_mca(&mut self) {
        let mut score = 90;
        if self.mca_city == "Pune" { score -= 15; }
        if self.mca_form == "AOC-4" { score -= 5; }
        self.mca_result = format!("Probability: {}%", score);
    }
    
    fn calc_board_risk(&mut self) {
        self.board_result.clear();
        if self.board_text.to_lowercase().contains("loan") { self.board_result.push("Sec 185 Risk".into()); }
        else { self.board_result.push("No obvious risks".into()); }
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
        let days = self.pen_days.parse::<i32>().unwrap_or(0);
        let rate = if self.pen_filing_type == "AOC-4" { 100 } else { 200 };
        self.pen_result = format!("Fee: ₹{}", days * rate);
    }

    fn calc_tax_regime(&mut self) {
        let i = self.tax_inc.parse::<f64>().unwrap_or(0.0);
        let d = self.tax_ded.parse::<f64>().unwrap_or(0.0);
        let old_tax = (i - d - 50000.0) * 0.3;
        let new_tax = (i - 75000.0) * 0.2;
        self.tax_result = format!("Old: {:.0} | New: {:.0}", old_tax, new_tax);
    }
    
    fn calc_crypto(&mut self) {
        let p = self.cry_prof.parse::<f64>().unwrap_or(0.0);
        self.cry_result = format!("Tax: {:.2}", p * 0.312);
    }

    fn calc_hra(&mut self) {
        let b = self.hra_basic.parse::<f64>().unwrap_or(0.0);
        let r = self.hra_rent.parse::<f64>().unwrap_or(0.0);
        let ex = (r - (b * 0.1)).max(0.0);
        self.hra_result = format!("Exempt: {:.0}", ex);
    }

    fn calc_pmla(&mut self) {
        let amt = self.pmla_amt.parse::<f64>().unwrap_or(0.0);
        self.pmla_result = if amt > 1000000.0 || (amt > 50000.0 && self.pmla_cash) { "HIGH RISK".into() } else { "Standard".into() };
    }

    fn calc_shell(&mut self) {
        let t = self.shell_to.parse::<f64>().unwrap_or(1.0);
        let a = self.shell_ast.parse::<f64>().unwrap_or(1.0);
        self.shell_result = if (t / a) < 0.05 { "High Risk (Shell Indicator)" } else { "Active".into() }.into();
    }
    
    fn calc_advance_tax(&mut self) {
        let tax = self.adv_tax.parse::<f64>().unwrap_or(0.0);
        let paid = self.adv_paid.parse::<f64>().unwrap_or(0.0);
        self.adv_result = format!("Due: ₹{:.2}", (tax * 0.15) - paid);
    }

    fn calc_lease(&mut self) {
        let pmt = self.lease_pmt.parse::<f64>().unwrap_or(0.0);
        let r = self.lease_rate.parse::<f64>().unwrap_or(0.0) / 100.0;
        let n = self.lease_years.parse::<f64>().unwrap_or(0.0);
        let rou = pmt * ((1.0 - (1.0 + r).powf(-n)) / r);
        self.lease_result = format!("ROU Asset: ₹{:.2}", rou);
    }

    fn calc_angel(&mut self) {
        let issue = self.angel_issue.parse::<f64>().unwrap_or(0.0);
        let fmv = self.angel_fmv.parse::<f64>().unwrap_or(0.0);
        self.angel_result = if issue > fmv { format!("Taxable: ₹{}", issue - fmv) } else { "Safe".into() };
    }

    fn calc_buyback(&mut self) {
        let sh = self.buy_shares.parse::<f64>().unwrap_or(0.0);
        let pr = self.buy_price.parse::<f64>().unwrap_or(0.0);
        self.buy_result = format!("Tax: ₹{:.2}", sh * pr * 0.23296);
    }

    fn calc_esg(&mut self) {
        let mc = self.esg_mcap.parse::<f64>().unwrap_or(0.0);
        self.esg_result = if mc > 5000.0 { "Mandatory BRSR" } else { "Voluntary" }.into();
    }

    fn calc_udin(&mut self) {
        self.udin_result = if self.udin_val.len() == 18 { "Valid Format" } else { "Invalid" }.into();
    }

    fn calc_audit(&mut self) {
        let y = self.audit_years.parse::<i32>().unwrap_or(0);
        self.audit_result = if y >= 10 { "Rotation Required" } else { format!("{} years left", 10 - y) };
    }

    fn calc_networth(&mut self) {
        let c = self.nw_cap.parse::<f64>().unwrap_or(0.0);
        let r = self.nw_res.parse::<f64>().unwrap_or(0.0);
        self.nw_result = format!("Net Worth: ₹{}", c + r);
    }

    fn calc_export(&mut self) {
        let days = (Local::now().date_naive() - self.exp_date).num_days();
        self.exp_result = if days > 270 { "Overdue (FEMA Risk)" } else { "Compliant" }.into();
    }

    fn calc_partner(&mut self) {
        let a = self.part_ast.parse::<f64>().unwrap_or(0.0);
        let l = self.part_lia.parse::<f64>().unwrap_or(0.0);
        self.part_result = format!("Net Asset: ₹{}", a - l);
    }
}

// ============================================================================
//  UI RENDERER
// ============================================================================

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // --- LICENSE SCREEN ---
        if !self.license_accepted {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(egui::RichText::new("PRATYAKSH AI").size(40.0).strong().color(COLOR_ACCENT));
                    ui.add_space(20.0);
                    ui.label("END USER LICENSE AGREEMENT");
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    ui.label("By using this software, you agree that PratyakshAI is a decision support tool.");
                    ui.label("Professional judgment must be exercised. We are not liable for tax penalties.");
                    ui.add_space(30.0);
                    if ui.button("I ACCEPT & CONTINUE").clicked() {
                        self.license_accepted = true;
                        self.current_page = Page::Dashboard;
                        self.status_msg = "License Accepted. System Active.".into();
                    }
                });
            });
            return;
        }

        // --- MAIN APP ---
        egui::SidePanel::left("nav").exact_width(220.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("PRATYAKSH").size(20.0).strong());
            ui.label(egui::RichText::new("ULTIMATE v10.0").size(10.0).color(COLOR_ACCENT));
            ui.add_space(30.0);
            
            if nav_btn(ui, "Dashboard", ICON_GRID, self.current_page == Page::Dashboard).clicked() { self.current_page = Page::Dashboard; }
            if nav_btn(ui, "Core Modules", ICON_SHIELD, self.current_page == Page::Modules).clicked() { self.current_page = Page::Modules; }
            if nav_btn(ui, "Smart Tools", ICON_CALC, self.current_page == Page::Tools).clicked() { self.current_page = Page::Tools; }
            if nav_btn(ui, "Settings", ICON_SETTINGS, self.current_page == Page::Settings).clicked() { self.current_page = Page::Settings; }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Dashboard => {
                    ui.heading("Executive Dashboard");
                    ui.add_space(20.0);
                    ui.columns(2, |cols| {
                        stat_card(&mut cols[0], "Total Clients", &self.client_count.to_string());
                        stat_card(&mut cols[1], "Evidence Logs", &self.evidence_count.to_string());
                    });
                },
                Page::Modules => {
                    ui.heading("Core Modules");
                    egui::Grid::new("mods").spacing([20.0, 20.0]).show(ui, |ui| {
                        ui.group(|ui| {
                            ui.heading("City Risk Engine");
                            ui.horizontal(|ui| { ui.label("City:"); ui.text_edit_singleline(&mut self.risk_city); });
                            ui.label(format!("Risk: {}%", self.risk_data.get(&self.risk_city).unwrap_or(&0)));
                        });
                        ui.group(|ui| {
                            ui.heading("Evidence Locker");
                            ui.text_edit_singleline(&mut self.ev_client_name);
                            ui.text_edit_singleline(&mut self.ev_note);
                            if ui.button("Lock Evidence").clicked() { self.save_evidence(); }
                        });
                        ui.end_row();
                    });
                },
                Page::Tools => {
                    ui.heading("Smart Tools Library");
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("TAXATION:");
                        if tool_btn(ui, "Tax Regime Analyzer", self.active_tool == ActiveTool::TaxRegime).clicked() { self.active_tool = ActiveTool::TaxRegime; }
                        if tool_btn(ui, "Crypto Tax (115BBH)", self.active_tool == ActiveTool::CryptoTax).clicked() { self.active_tool = ActiveTool::CryptoTax; }
                        if tool_btn(ui, "HRA Calculator", self.active_tool == ActiveTool::HraCalc).clicked() { self.active_tool = ActiveTool::HraCalc; }
                        if tool_btn(ui, "Advance Tax", self.active_tool == ActiveTool::AdvanceTax).clicked() { self.active_tool = ActiveTool::AdvanceTax; }
                        if tool_btn(ui, "Angel Tax", self.active_tool == ActiveTool::AngelTax).clicked() { self.active_tool = ActiveTool::AngelTax; }
                        if tool_btn(ui, "Buyback Tax", self.active_tool == ActiveTool::BuybackTax).clicked() { self.active_tool = ActiveTool::BuybackTax; }
                        
                        ui.add_space(10.0);
                        ui.label("COMPLIANCE:");
                        if tool_btn(ui, "MCA Predictor", self.active_tool == ActiveTool::McaPredictor).clicked() { self.active_tool = ActiveTool::McaPredictor; }
                        if tool_btn(ui, "Board Risk", self.active_tool == ActiveTool::BoardRisk).clicked() { self.active_tool = ActiveTool::BoardRisk; }
                        if tool_btn(ui, "PMLA Scanner", self.active_tool == ActiveTool::PmlaScanner).clicked() { self.active_tool = ActiveTool::PmlaScanner; }
                        if tool_btn(ui, "Shell Co. Risk", self.active_tool == ActiveTool::ShellRisk).clicked() { self.active_tool = ActiveTool::ShellRisk; }
                        if tool_btn(ui, "ESG Applicability", self.active_tool == ActiveTool::EsgCheck).clicked() { self.active_tool = ActiveTool::EsgCheck; }
                        if tool_btn(ui, "UDIN Validator", self.active_tool == ActiveTool::UdinValid).clicked() { self.active_tool = ActiveTool::UdinValid; }
                        if tool_btn(ui, "Audit Rotation", self.active_tool == ActiveTool::AuditRot).clicked() { self.active_tool = ActiveTool::AuditRot; }
                        if tool_btn(ui, "Export Tracker", self.active_tool == ActiveTool::ExportTrack).clicked() { self.active_tool = ActiveTool::ExportTrack; }

                        ui.add_space(10.0);
                        ui.label("FINANCE:");
                        if tool_btn(ui, "MSME 43B(h)", self.active_tool == ActiveTool::MsmeCalc).clicked() { self.active_tool = ActiveTool::MsmeCalc; }
                        if tool_btn(ui, "Gratuity", self.active_tool == ActiveTool::GratuityCalc).clicked() { self.active_tool = ActiveTool::GratuityCalc; }
                        if tool_btn(ui, "Lease Liability", self.active_tool == ActiveTool::LeaseCalc).clicked() { self.active_tool = ActiveTool::LeaseCalc; }
                        if tool_btn(ui, "Net Worth", self.active_tool == ActiveTool::NetWorth).clicked() { self.active_tool = ActiveTool::NetWorth; }
                        if tool_btn(ui, "Partnership Diss.", self.active_tool == ActiveTool::PartnerDiss).clicked() { self.active_tool = ActiveTool::PartnerDiss; }

                        ui.add_space(20.0);
                        ui.separator();
                        
                        match self.active_tool {
                            ActiveTool::TaxRegime => {
                                ui.heading("Tax Regime Analyzer");
                                ui.horizontal(|ui| { ui.label("Income:"); ui.text_edit_singleline(&mut self.tax_inc); });
                                ui.horizontal(|ui| { ui.label("Deductions:"); ui.text_edit_singleline(&mut self.tax_ded); });
                                if ui.button("Compare").clicked() { self.calc_tax_regime(); }
                                ui.label(&self.tax_result);
                            },
                            ActiveTool::MsmeCalc => {
                                ui.heading("MSME 43B(h)");
                                ui.horizontal(|ui| { ui.label("Amt:"); ui.text_edit_singleline(&mut self.msme_amt); });
                                ui.horizontal(|ui| { ui.label("Inv Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_inv_date)); });
                                ui.horizontal(|ui| { ui.label("Pay Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.msme_pay_date)); });
                                if ui.button("Check").clicked() { self.calc_msme(); }
                                ui.label(&self.msme_result);
                            },
                            ActiveTool::CryptoTax => {
                                ui.heading("VDA / Crypto Tax");
                                ui.horizontal(|ui| { ui.label("Profit:"); ui.text_edit_singleline(&mut self.cry_prof); });
                                if ui.button("Calculate").clicked() { self.calc_crypto(); }
                                ui.label(&self.cry_result);
                            },
                            ActiveTool::AdvanceTax => {
                                ui.heading("Advance Tax");
                                ui.horizontal(|ui| { ui.label("Est Tax:"); ui.text_edit_singleline(&mut self.adv_tax); });
                                ui.horizontal(|ui| { ui.label("Paid:"); ui.text_edit_singleline(&mut self.adv_paid); });
                                if ui.button("Calculate").clicked() { self.calc_advance_tax(); }
                                ui.label(&self.adv_result);
                            },
                            ActiveTool::LeaseCalc => {
                                ui.heading("Lease Liability");
                                ui.horizontal(|ui| { ui.label("Pmt:"); ui.text_edit_singleline(&mut self.lease_pmt); });
                                ui.horizontal(|ui| { ui.label("Yrs:"); ui.text_edit_singleline(&mut self.lease_years); });
                                if ui.button("Calculate").clicked() { self.calc_lease(); }
                                ui.label(&self.lease_result);
                            },
                            ActiveTool::AngelTax => {
                                ui.heading("Angel Tax");
                                ui.horizontal(|ui| { ui.label("Issue:"); ui.text_edit_singleline(&mut self.angel_issue); });
                                ui.horizontal(|ui| { ui.label("FMV:"); ui.text_edit_singleline(&mut self.angel_fmv); });
                                if ui.button("Check").clicked() { self.calc_angel(); }
                                ui.label(&self.angel_result);
                            },
                            ActiveTool::EsgCheck => {
                                ui.heading("ESG Check");
                                ui.horizontal(|ui| { ui.label("MCap:"); ui.text_edit_singleline(&mut self.esg_mcap); });
                                if ui.button("Check").clicked() { self.calc_esg(); }
                                ui.label(&self.esg_result);
                            },
                            ActiveTool::UdinValid => {
                                ui.heading("UDIN Validator");
                                ui.horizontal(|ui| { ui.label("UDIN:"); ui.text_edit_singleline(&mut self.udin_val); });
                                if ui.button("Verify").clicked() { self.calc_udin(); }
                                ui.label(&self.udin_result);
                            },
                            ActiveTool::AuditRot => {
                                ui.heading("Audit Rotation");
                                ui.horizontal(|ui| { ui.label("Years Served:"); ui.text_edit_singleline(&mut self.audit_years); });
                                if ui.button("Check").clicked() { self.calc_audit(); }
                                ui.label(&self.audit_result);
                            },
                             ActiveTool::NetWorth => {
                                ui.heading("Net Worth");
                                ui.horizontal(|ui| { ui.label("Cap:"); ui.text_edit_singleline(&mut self.nw_cap); });
                                ui.horizontal(|ui| { ui.label("Res:"); ui.text_edit_singleline(&mut self.nw_res); });
                                if ui.button("Calculate").clicked() { self.calc_networth(); }
                                ui.label(&self.nw_result);
                            },
                            ActiveTool::ExportTrack => {
                                ui.heading("Export Realization");
                                ui.horizontal(|ui| { ui.label("Exp Date:"); ui.add(egui_extras::DatePickerButton::new(&mut self.exp_date)); });
                                if ui.button("Check").clicked() { self.calc_export(); }
                                ui.label(&self.exp_result);
                            },
                            _ => { ui.label("Select a tool from the list above."); }
                        }
                    });
                },
                Page::Settings => { ui.label("Settings Page"); },
                _ => {}
            }
        });
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
            ui.label(egui::RichText::new(text).color(fg));
        });
    }).response.interact(egui::Sense::click())
}

fn tool_btn(ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
    let text_color = if active { COLOR_ACCENT } else { COLOR_TEXT };
    ui.add(egui::Button::new(egui::RichText::new(text).color(text_color)).frame(false))
}

fn stat_card(ui: &mut egui::Ui, label: &str, val: &str) {
    egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(25,25,25)).inner_margin(15.0).show(ui, |ui| {
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
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]).with_title("PratyakshAI Enterprise"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}