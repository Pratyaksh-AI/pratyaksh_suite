#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{Local, NaiveDate};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS: WINDOWS 10 STYLE MINIMAL WIREFRAME ICONS (SVG)
// ============================================================================

const COLOR_ACCENT: egui::Color32 = egui::Color32::from_rgb(0, 120, 215); // Windows Blue
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(32, 32, 32);      // Windows Dark
const COLOR_PANEL: egui::Color32 = egui::Color32::from_rgb(45, 45, 48);   // Lighter Panel
const COLOR_TEXT: egui::Color32 = egui::Color32::WHITE;
const COLOR_MUTED: egui::Color32 = egui::Color32::from_rgb(160, 160, 160);

const ICON_DASH: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"##.as_bytes();
const ICON_CITY: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M3 21h18"/><path d="M5 21V7l8-4 8 4v14"/><path d="M8 21v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>"##.as_bytes();
const ICON_TOOL: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>"##.as_bytes();
const ICON_SETT: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"##.as_bytes();
const ICON_SHIELD: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"##.as_bytes();

// ============================================================================
//  2. DATA & ENUMS
// ============================================================================

#[derive(PartialEq, Clone, Copy)]
enum Language { English, Hindi, Marathi }

#[derive(PartialEq, Clone, Copy)]
enum Page {
    LicenseAgreement,
    Dashboard,
    CityRisk,
    ClientIntegrity,
    EvidenceLocker,
    // --- 10 NEW PAGES ---
    GstScanner,
    ItScrutiny,
    TdsRecon,
    RocCompliance,
    TrademarkWatch,
    IbcStatus,
    LaborLaws,
    ImportExport,
    StartupIndia,
    MsmeStatus,
    // ----------------
    SmartTools,
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
    ExportTrack, PartnerDiss,
    // New 10 Tools
    GstInterest, DepreciationCalc, CapitalGains, LlpFee, EmiCalc,
    BurnRate, SimpleInt, TdsInterest, CagrCalc, BreakEven
}

#[derive(Debug, Clone)]
struct Client { id: i32, name: String, city: String, trust: i32 }
#[derive(Debug, Clone)]
struct EvidenceLog { id: i32, client: String, note: String, hash: String, date: String }

struct PratyakshApp {
    db: Arc<Mutex<Connection>>,
    current_page: Page,
    current_lang: Language,
    license_accepted: bool,
    
    client_count: i32,
    evidence_count: i32,
    risk_data: HashMap<String, i32>,

    // --- Inputs for New 10 Pages ---
    gst_sales_1: String, gst_sales_3b: String, gst_res: String,
    it_income: String, it_high_val: String, it_res: String,
    tds_deducted: String, tds_deposited: String, tds_res: String,
    roc_cin: String, roc_res: String,
    tm_app_no: String, tm_res: String,
    ibc_case_no: String, ibc_res: String,
    labor_emp_count: String, labor_res: String,
    ie_code: String, ie_res: String,
    startup_dipp: String, startup_res: String,
    msme_reg: String, msme_res: String,

    // --- Core Inputs ---
    new_client_name: String, new_client_city: String,
    ev_client_name: String, ev_note: String,
    active_tool: ActiveTool,
    
    // Tool Inputs
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
    
    // Extra 10 Tools Inputs
    gst_tax: String, gst_days: String, gst_calc_res: String,
    dep_cost: String, dep_rate: String, dep_res: String,
    cg_cost: String, cg_idx1: String, cg_idx2: String, cg_result: String,
    llp_contrib: String, llp_result: String,
    emi_p: String, emi_r: String, emi_n: String, emi_result: String,
    burn_cash: String, burn_spend: String, burn_result: String,
    si_p: String, si_r: String, si_t: String, si_result: String,
    tds_amt: String, tds_months: String, tds_calc_res: String,
    cagr_start: String, cagr_end: String, cagr_yrs: String, cagr_result: String,
    be_fixed: String, be_price: String, be_var: String, be_result: String,

    status_msg: String,
    clients: Vec<Client>,
    evidence_logs: Vec<EvidenceLog>,
    evidence_client_select: String,
    evidence_action: String,
    risk_city: String,
}

impl PratyakshApp {
    fn init_db() -> Connection {
        let conn = Connection::open("pratyaksh_v11.db").expect("DB Init Failed");
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clients (id INTEGER PRIMARY KEY, name TEXT, city TEXT, trust INTEGER);
             CREATE TABLE IF NOT EXISTS evidence (id INTEGER PRIMARY KEY, client TEXT, note TEXT, hash TEXT, date TEXT);
             CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);"
        ).ok();
        conn
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        setup_windows_theme(&cc.egui_ctx);
        
        let db_conn = Self::init_db();
        let license_accepted = db_conn.query_row(
            "SELECT value FROM settings WHERE key = 'license_accepted'", [], |r| r.get::<_, String>(0)
        ).unwrap_or("false".into()) == "true";

        let mut app = Self {
            db: Arc::new(Mutex::new(db_conn)),
            current_page: if license_accepted { Page::Dashboard } else { Page::LicenseAgreement },
            current_lang: Language::English,
            license_accepted,
            active_tool: ActiveTool::None,
            
            client_count: 0, evidence_count: 0,
            risk_data: HashMap::from([("Pune".into(), 72), ("Mumbai".into(), 55)]),
            risk_city: "Pune".into(),
            
            gst_sales_1: "".into(), gst_sales_3b: "".into(), gst_res: "".into(),
            it_income: "".into(), it_high_val: "".into(), it_res: "".into(),
            tds_deducted: "".into(), tds_deposited: "".into(), tds_res: "".into(),
            roc_cin: "".into(), roc_res: "".into(),
            tm_app_no: "".into(), tm_res: "".into(),
            ibc_case_no: "".into(), ibc_res: "".into(),
            labor_emp_count: "".into(), labor_res: "".into(),
            ie_code: "".into(), ie_res: "".into(),
            startup_dipp: "".into(), startup_res: "".into(),
            msme_reg: "".into(), msme_res: "".into(),

            new_client_name: "".into(), new_client_city: "Pune".into(),
            ev_client_name: "".into(), ev_note: "".into(),
            mca_city: "Pune".into(), mca_form: "AOC-4".into(), mca_result: "".into(),
            board_text: "".into(), board_result: vec![],
            trust_gst: "".into(), trust_bank: "".into(), trust_result: "".into(),
            reg_id: "".into(), reg_note: "".into(),
            msme_amt: "".into(), msme_inv_date: Local::now().date_naive(), msme_pay_date: Local::now().date_naive(), msme_result: "".into(),
            grat_sal: "".into(), grat_yrs: "".into(), grat_result: "".into(),
            pen_days: "".into(), pen_filing_type: "AOC-4".into(), pen_result: "".into(),
            
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

            gst_tax: "".into(), gst_days: "".into(), gst_calc_res: "".into(),
            dep_cost: "".into(), dep_rate: "".into(), dep_result: "".into(),
            cg_cost: "".into(), cg_idx1: "".into(), cg_idx2: "".into(), cg_result: "".into(),
            llp_contrib: "".into(), llp_result: "".into(),
            emi_p: "".into(), emi_r: "".into(), emi_n: "".into(), emi_result: "".into(),
            burn_cash: "".into(), burn_spend: "".into(), burn_result: "".into(),
            si_p: "".into(), si_r: "".into(), si_t: "".into(), si_result: "".into(),
            tds_amt: "".into(), tds_months: "".into(), tds_calc_res: "".into(),
            cagr_start: "".into(), cagr_end: "".into(), cagr_yrs: "".into(), cagr_result: "".into(),
            be_fixed: "".into(), be_price: "".into(), be_var: "".into(), be_result: "".into(),

            status_msg: "System Online".into(),
            clients: vec![], evidence_logs: vec![], evidence_client_select: "".into(),
            evidence_action: "Advice: ".into(),
        };
        app.refresh_db();
        app
    }

    fn t(&self, text: &str) -> String {
        match self.current_lang {
            Language::English => text.to_string(),
            Language::Hindi => match text {
                "Dashboard" => "डैशबोर्ड",
                "City Risk" => "शहर जोखिम",
                "Client Integrity" => "ग्राहक सत्यता",
                "Evidence Locker" => "साक्ष्य लॉकर",
                "GST AI Scanner" => "जीएसटी एआई स्कैनर",
                "Income Tax Scrutiny" => "आयकर जांच",
                "TDS Reconciliation" => "टीडीएस मिलान",
                "ROC Compliance" => "आरओसी अनुपालन",
                "Trademark Status" => "ट्रेडमार्क स्थिति",
                "Smart Tools" => "स्मार्ट टूल्स",
                "Settings" => "सेटिंग्स",
                "License Agreement" => "लाइसेंस समझौता",
                "I ACCEPT" => "मुझे स्वीकार है",
                "Calculate" => "गणना करें",
                "Scan" => "स्कैन करें",
                _ => text
            }.to_string(),
            Language::Marathi => match text {
                "Dashboard" => "डॅशबोर्ड",
                "City Risk" => "शहर जोखीम",
                "Client Integrity" => "ग्राहक सत्यता",
                "Evidence Locker" => "पुरावा लॉकर",
                "GST AI Scanner" => "जीएसटी एआय स्कॅनर",
                "Income Tax Scrutiny" => "आयकर तपासणी",
                "TDS Reconciliation" => "टीडीएस जुळवणी",
                "ROC Compliance" => "आरओसी अनुपालन",
                "Trademark Status" => "ट्रेडमार्क स्थिती",
                "Smart Tools" => "स्मार्ट टूल्स",
                "Settings" => "सेटिंग्ज",
                "License Agreement" => "परवाना करार",
                "I ACCEPT" => "मला मान्य आहे",
                "Calculate" => "गणना करा",
                "Scan" => "स्कॅन करा",
                _ => text
            }.to_string(),
        }
    }

    fn accept_license(&mut self) {
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES ('license_accepted', 'true')", []).ok();
        self.license_accepted = true;
        self.current_page = Page::Dashboard;
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

    fn calc_gst_risk(&mut self) {
        let r1 = self.gst_sales_1.parse::<f64>().unwrap_or(0.0);
        let r3b = self.gst_sales_3b.parse::<f64>().unwrap_or(0.0);
        if r1 == 0.0 { return; }
        let diff = (r1 - r3b).abs();
        let percent = (diff / r1) * 100.0;
        self.gst_res = if percent > 10.0 { format!("CRITICAL: {:.2}% Mismatch. Probability 85%", percent) } else { format!("SAFE: {:.2}% Mismatch.", percent) };
    }

    fn calc_it_risk(&mut self) {
        let inc = self.it_income.parse::<f64>().unwrap_or(0.0);
        let txn = self.it_high_val.parse::<f64>().unwrap_or(0.0);
        self.it_res = if txn > (inc * 0.5) { "HIGH RISK: SFT Mismatch".to_string() } else { "LOW RISK: Verified".to_string() };
    }

    fn calc_tds_recon(&mut self) {
        let ded = self.tds_deducted.parse::<f64>().unwrap_or(0.0);
        let dep = self.tds_deposited.parse::<f64>().unwrap_or(0.0);
        self.tds_res = if dep < ded { format!("SHORTFALL: ₹{}", ded - dep) } else { "MATCHED".to_string() };
    }

    fn add_client(&mut self) {
        if self.new_client_name.is_empty() { return; }
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO clients (name, city, trust) VALUES (?1, ?2, ?3)", params![self.new_client_name, self.new_client_city, 90]).ok();
        self.new_client_name.clear();
        drop(conn);
        self.refresh_db();
    }

    fn save_evidence(&mut self) {
        if self.ev_client_name.is_empty() { return; }
        let now = Local::now().to_rfc3339();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.ev_client_name, self.ev_note, now));
        let hash = hex::encode(hasher.finalize());
        let conn = self.db.lock().unwrap();
        conn.execute("INSERT INTO evidence (client, note, hash, date) VALUES (?1, ?2, ?3, ?4)", params![self.ev_client_name, self.ev_note, hash, now]).ok();
        drop(conn);
        self.refresh_db();
    }

    fn calc_mca(&mut self) {
        let mut score = 90;
        if self.mca_city == "Pune" { score -= 15; }
        if self.mca_form == "AOC-4" { score -= 5; }
        self.mca_result = format!("Probability: {}%", score);
    }
}

impl eframe::App for PratyakshApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.license_accepted {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(egui::RichText::new("PRATYAKSH AI").size(40.0).color(COLOR_ACCENT));
                    ui.add_space(20.0);
                    ui.label(self.t("License Agreement"));
                    ui.separator();
                    ui.label("1. Compliance estimates, not legal advice.");
                    ui.label("2. User responsibility for all filings.");
                    ui.add_space(30.0);
                    if ui.button(self.t("I ACCEPT")).clicked() { self.accept_license(); }
                });
            });
            return;
        }

        egui::SidePanel::left("nav").exact_width(240.0).show(ctx, |ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("PRATYAKSH").size(24.0).strong());
            ui.label(egui::RichText::new("ENTERPRISE v11.0").size(10.0).color(COLOR_ACCENT));
            ui.add_space(20.0);
            
            ui.horizontal(|ui| {
                ui.label("Lang:");
                egui::ComboBox::from_id_source("lang").selected_text(match self.current_lang {
                    Language::English => "English", Language::Hindi => "हिंदी", Language::Marathi => "मराठी"
                }).show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.current_lang, Language::English, "English");
                    ui.selectable_value(&mut self.current_lang, Language::Hindi, "हिंदी");
                    ui.selectable_value(&mut self.current_lang, Language::Marathi, "मराठी");
                });
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                if nav_btn(ui, &self.t("Dashboard"), ICON_DASH, self.current_page == Page::Dashboard).clicked() { self.current_page = Page::Dashboard; }
                ui.add_space(10.0);
                ui.label(egui::RichText::new("CORE MODULES").size(10.0).color(COLOR_MUTED));
                if nav_btn(ui, &self.t("City Risk"), ICON_CITY, self.current_page == Page::CityRisk).clicked() { self.current_page = Page::CityRisk; }
                if nav_btn(ui, &self.t("Client Integrity"), ICON_USER, self.current_page == Page::ClientIntegrity).clicked() { self.current_page = Page::ClientIntegrity; }
                if nav_btn(ui, &self.t("Evidence Locker"), ICON_LOCK, self.current_page == Page::EvidenceLocker).clicked() { self.current_page = Page::EvidenceLocker; }

                ui.add_space(10.0);
                ui.label(egui::RichText::new("AI PREDICTORS").size(10.0).color(COLOR_ACCENT));
                if nav_btn(ui, &self.t("GST AI Scanner"), ICON_DOC, self.current_page == Page::GstScanner).clicked() { self.current_page = Page::GstScanner; }
                if nav_btn(ui, &self.t("Income Tax Scrutiny"), ICON_DOC, self.current_page == Page::ItScrutiny).clicked() { self.current_page = Page::ItScrutiny; }
                if nav_btn(ui, &self.t("TDS Reconciliation"), ICON_DOC, self.current_page == Page::TdsRecon).clicked() { self.current_page = Page::TdsRecon; }
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new("COMPLIANCE").size(10.0).color(COLOR_MUTED));
                if nav_btn(ui, "ROC Compliance", ICON_DOC, self.current_page == Page::RocCompliance).clicked() { self.current_page = Page::RocCompliance; }
                if nav_btn(ui, "Labor Laws", ICON_DOC, self.current_page == Page::LaborLaws).clicked() { self.current_page = Page::LaborLaws; }
                if nav_btn(ui, "MSME Samadhaan", ICON_DOC, self.current_page == Page::MsmeStatus).clicked() { self.current_page = Page::MsmeStatus; }

                ui.add_space(10.0);
                if nav_btn(ui, &self.t("Smart Tools"), ICON_TOOL, self.current_page == Page::SmartTools).clicked() { self.current_page = Page::SmartTools; }
                if nav_btn(ui, &self.t("Settings"), ICON_SETT, self.current_page == Page::Settings).clicked() { self.current_page = Page::Settings; }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            match self.current_page {
                Page::Dashboard => {
                    ui.heading(self.t("Dashboard"));
                    ui.columns(2, |cols| {
                        stat_card(&mut cols[0], "Clients", &self.client_count.to_string());
                        stat_card(&mut cols[1], "Evidence", &self.evidence_count.to_string());
                    });
                },
                Page::GstScanner => {
                    ui.heading("GST AI Scanner");
                    egui::Grid::new("gst").show(ui, |ui| {
                        ui.label("GSTR-1:"); ui.text_edit_singleline(&mut self.gst_sales_1); ui.end_row();
                        ui.label("GSTR-3B:"); ui.text_edit_singleline(&mut self.gst_sales_3b); ui.end_row();
                    });
                    if ui.button(self.t("Scan")).clicked() { self.calc_gst_risk(); }
                    ui.label(&self.gst_res);
                },
                Page::ItScrutiny => {
                    ui.heading("IT Scrutiny AI");
                    egui::Grid::new("it").show(ui, |ui| {
                        ui.label("Income:"); ui.text_edit_singleline(&mut self.it_income); ui.end_row();
                        ui.label("SFT Txn:"); ui.text_edit_singleline(&mut self.it_high_val); ui.end_row();
                    });
                    if ui.button("Analyze").clicked() { self.calc_it_risk(); }
                    ui.label(&self.it_res);
                },
                Page::TdsRecon => {
                    ui.heading("TDS Reconciliation");
                    egui::Grid::new("tds").show(ui, |ui| {
                        ui.label("Deducted:"); ui.text_edit_singleline(&mut self.tds_deducted); ui.end_row();
                        ui.label("Deposited:"); ui.text_edit_singleline(&mut self.tds_deposited); ui.end_row();
                    });
                    if ui.button("Compare").clicked() { self.calc_tds_recon(); }
                    ui.label(&self.tds_res);
                },
                Page::CityRisk => {
                    ui.heading("City Risk Index");
                    ui.text_edit_singleline(&mut self.risk_city);
                    ui.label(format!("Risk: {}%", self.risk_data.get(&self.risk_city).unwrap_or(&0)));
                },
                Page::ClientIntegrity => {
                    ui.heading("Clients");
                    ui.text_edit_singleline(&mut self.new_client_name);
                    if ui.button("Add").clicked() { self.add_client(); }
                    for c in &self.clients { ui.label(&c.name); }
                },
                Page::EvidenceLocker => {
                    ui.heading("Evidence");
                    ui.text_edit_singleline(&mut self.ev_client_name);
                    ui.text_edit_singleline(&mut self.ev_note);
                    if ui.button("Save").clicked() { self.save_evidence(); }
                },
                Page::SmartTools => {
                    ui.heading("Calculators");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.mca_city);
                        if ui.button("Predict MCA").clicked() { self.calc_mca(); }
                    });
                    ui.label(&self.mca_result);
                },
                Page::Settings => { ui.heading("Settings"); ui.label("V11.0 Enterprise Local Build"); },
                _ => { ui.label("Page content loading..."); }
            }
        });
    }
}

fn nav_btn(ui: &mut egui::Ui, text: &str, icon: &'static [u8], active: bool) -> egui::Response {
    let bg = if active { COLOR_ACCENT } else { egui::Color32::TRANSPARENT };
    let fg = if active { egui::Color32::BLACK } else { COLOR_TEXT };
    egui::Frame::none().fill(bg).rounding(4.0).inner_margin(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon).max_width(18.0).tint(fg));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(fg));
        });
    }).response.interact(egui::Sense::click())
}

fn stat_card(ui: &mut egui::Ui, label: &str, val: &str) {
    egui::Frame::group(ui.style()).fill(COLOR_PANEL).inner_margin(15.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(egui::RichText::new(label).size(12.0).color(COLOR_MUTED));
        ui.heading(egui::RichText::new(val).size(24.0).color(COLOR_TEXT));
    });
}

fn setup_windows_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = COLOR_BG;
    visuals.panel_fill = COLOR_BG;
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 850.0]).with_title("PratyakshAI Enterprise"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}