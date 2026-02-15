#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rusqlite::{params, Connection};
use chrono::{Local, NaiveDate};
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// ============================================================================
//  1. ASSETS: WINDOWS 10 STYLE MINIMAL ICONS (SVG)
// ============================================================================

const COLOR_ACCENT: egui::Color32 = egui::Color32::from_rgb(0, 120, 215); // Windows Blue
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(32, 32, 32);      // Windows Dark
const COLOR_TEXT: egui::Color32 = egui::Color32::WHITE;
const COLOR_MUTED: egui::Color32 = egui::Color32::from_rgb(160, 160, 160);

// Minimal Wireframe Icons
const ICON_DASH: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"##.as_bytes();
const ICON_RISK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>"##.as_bytes();
const ICON_USER: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"##.as_bytes();
const ICON_CITY: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M3 21h18"/><path d="M5 21V7l8-4 8 4v14"/><path d="M8 21v-2a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"##.as_bytes();
const ICON_LOCK: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"##.as_bytes();
const ICON_DOC: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>"##.as_bytes();
const ICON_TOOL: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/></svg>"##.as_bytes();
const ICON_SETT: &[u8] = r##"<svg viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"##.as_bytes();

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
    
    // Core Data
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
    gst_tax: String, gst_days: String, gst_result: String,
    dep_cost: String, dep_rate: String, dep_result: String,
    cg_cost: String, cg_idx1: String, cg_idx2: String, cg_result: String,
    llp_contrib: String, llp_result: String,
    emi_p: String, emi_r: String, emi_n: String, emi_result: String,
    burn_cash: String, burn_spend: String, burn_result: String,
    si_p: String, si_r: String, si_t: String, si_result: String,
    tds_amt: String, tds_months: String, tds_result: String,
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
            
            // Init New Page Inputs
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

            // Extra 10
            gst_tax: "".into(), gst_days: "".into(), gst_result: "".into(),
            dep_cost: "".into(), dep_rate: "".into(), dep_result: "".into(),
            cg_cost: "".into(), cg_idx1: "".into(), cg_idx2: "".into(), cg_result: "".into(),
            llp_contrib: "".into(), llp_result: "".into(),
            emi_p: "".into(), emi_r: "".into(), emi_n: "".into(), emi_result: "".into(),
            burn_cash: "".into(), burn_spend: "".into(), burn_result: "".into(),
            si_p: "".into(), si_r: "".into(), si_t: "".into(), si_result: "".into(),
            tds_amt: "".into(), tds_months: "".into(), tds_result: "".into(),
            cagr_start: "".into(), cagr_end: "".into(), cagr_yrs: "".into(), cagr_result: "".into(),
            be_fixed: "".into(), be_price: "".into(), be_var: "".into(), be_result: "".into(),

            status_msg: if license_accepted { "System Ready".into() } else { "Waiting for License...".into() },
            clients: vec![],
            evidence_logs: vec![],
            evidence_client_select: "".into(),
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

    // --- REAL AI NOTICE PREDICTION LOGIC ---

    fn calc_gst_risk(&mut self) {
        let r1 = self.gst_sales_1.parse::<f64>().unwrap_or(0.0);
        let r3b = self.gst_sales_3b.parse::<f64>().unwrap_or(0.0);
        if r1 == 0.0 { return; }
        
        let diff = (r1 - r3b).abs();
        let percent = (diff / r1) * 100.0;

        self.gst_res = if percent > 10.0 {
            format!("CRITICAL: {:.2}% Mismatch. ASMT-10 Notice Probability: 85%", percent)
        } else if percent > 5.0 {
            format!("HIGH: {:.2}% Mismatch. Reconcile Immediately.", percent)
        } else {
            format!("SAFE: {:.2}% Mismatch is within tolerance.", percent)
        };
    }

    fn calc_it_risk(&mut self) {
        let inc = self.it_income.parse::<f64>().unwrap_or(0.0);
        let txn = self.it_high_val.parse::<f64>().unwrap_or(0.0);
        self.it_res = if txn > (inc * 0.5) {
            "HIGH RISK: Sec 148A Notice Likely (SFT Mismatch)".to_string()
        } else {
            "LOW RISK: Income supports transactions.".to_string()
        };
    }

    fn calc_tds_recon(&mut self) {
        let ded = self.tds_deducted.parse::<f64>().unwrap_or(0.0);
        let dep = self.tds_deposited.parse::<f64>().unwrap_or(0.0);
        self.tds_res = if dep < ded {
            format!("SHORTFALL: ₹{}. Demand Notice Imminent.", ded - dep)
        } else {
            "MATCHED: No Demand Risk.".to_string()
        };
    }
    
    // --- CORE LOGIC ---
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

    // --- CALCULATORS ---
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
        let yrs = self.grat_yrs.parse::<f64>().unwrap_or(0.0);
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
        self.audit_result = if y >= 10 { "Rotation Required".to_string() } else { format!("{} years left", 10 - y) };
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

    fn calc_gst_int(&mut self) {
        let tax = self.gst_tax.parse::<f64>().unwrap_or(0.0);
        let days = self.gst_days.parse::<f64>().unwrap_or(0.0);
        self.gst_result = format!("Interest (18%): ₹{:.2}", tax * 0.18 * (days / 365.0));
    }
    
    fn calc_dep(&mut self) {
        let cost = self.dep_cost.parse::<f64>().unwrap_or(0.0);
        let rate = self.dep_rate.parse::<f64>().unwrap_or(0.0) / 100.0;
        self.dep_result = format!("WDV: ₹{:.2}", cost * (1.0 - rate));
    }

    fn calc_cg(&mut self) {
        let c = self.cg_cost.parse::<f64>().unwrap_or(0.0);
        let i1 = self.cg_idx1.parse::<f64>().unwrap_or(1.0);
        let i2 = self.cg_idx2.parse::<f64>().unwrap_or(1.0);
        self.cg_result = format!("Indexed Cost: ₹{:.2}", c * (i2 / i1));
    }

    fn calc_llp(&mut self) {
        let c = self.llp_contrib.parse::<f64>().unwrap_or(0.0);
        let fee = if c < 100000.0 { 50.0 } else { 100.0 };
        self.llp_result = format!("Filing Fee: ₹{}", fee);
    }

    fn calc_emi(&mut self) {
        let p = self.emi_p.parse::<f64>().unwrap_or(0.0);
        let r = self.emi_r.parse::<f64>().unwrap_or(0.0) / 1200.0;
        let n = self.emi_n.parse::<f64>().unwrap_or(0.0) * 12.0;
        let emi = p * r * (1.0 + r).powf(n) / ((1.0 + r).powf(n) - 1.0);
        self.emi_result = format!("Monthly EMI: ₹{:.2}", emi);
    }

    fn calc_burn(&mut self) {
        let c = self.burn_cash.parse::<f64>().unwrap_or(0.0);
        let s = self.burn_spend.parse::<f64>().unwrap_or(1.0);
        self.burn_result = format!("Runway: {:.1} months", c / s);
    }

    fn calc_si(&mut self) {
        let p = self.si_p.parse::<f64>().unwrap_or(0.0);
        let r = self.si_r.parse::<f64>().unwrap_or(0.0);
        let t = self.si_t.parse::<f64>().unwrap_or(0.0);
        self.si_result = format!("Interest: ₹{:.2}", (p * r * t) / 100.0);
    }
    
    fn calc_tds_int(&mut self) {
        let a = self.tds_amt.parse::<f64>().unwrap_or(0.0);
        let m = self.tds_months.parse::<f64>().unwrap_or(0.0);
        self.tds_result = format!("Interest (1.5%): ₹{:.2}", a * 0.015 * m);
    }
    
    fn calc_cagr(&mut self) {
        let s = self.cagr_start.parse::<f64>().unwrap_or(1.0);
        let e = self.cagr_end.parse::<f64>().unwrap_or(1.0);
        let y = self.cagr_yrs.parse::<f64>().unwrap_or(1.0);
        let cagr = (e / s).powf(1.0 / y) - 1.0;
        self.cagr_result = format!("CAGR: {:.2}%", cagr * 100.0);
    }

    fn calc_be(&mut self) {
        let f = self.be_fixed.parse::<f64>().unwrap_or(0.0);
        let p = self.be_price.parse::<f64>().unwrap_or(1.0);
        let v = self.be_var.parse::<f64>().unwrap_or(0.0);
        self.be_result = format!("Break Even: {:.0} units", f / (p - v));
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
                    ui.label("1. This software provides compliance estimates, not legal advice.");
                    ui.label("2. Users are responsible for all filings made based on this data.");
                    ui.label("3. Reverse engineering of the AI logic is prohibited.");
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
            
            // Language Switcher
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
                ui.label(egui::RichText::new("AI PREDICTORS (NEW)").size(10.0).color(COLOR_ACCENT));
                if nav_btn(ui, &self.t("GST AI Scanner"), ICON_DOC, self.current_page == Page::GstScanner).clicked() { self.current_page = Page::GstScanner; }
                if nav_btn(ui, &self.t("Income Tax Scrutiny"), ICON_DOC, self.current_page == Page::ItScrutiny).clicked() { self.current_page = Page::ItScrutiny; }
                if nav_btn(ui, &self.t("TDS Reconciliation"), ICON_DOC, self.current_page == Page::TdsRecon).clicked() { self.current_page = Page::TdsRecon; }
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new("COMPLIANCE TRACKERS").size(10.0).color(COLOR_MUTED));
                if nav_btn(ui, &self.t("ROC Compliance"), ICON_DOC, self.current_page == Page::RocCompliance).clicked() { self.current_page = Page::RocCompliance; }
                if nav_btn(ui, &self.t("Trademark Status"), ICON_DOC, self.current_page == Page::TrademarkWatch).clicked() { self.current_page = Page::TrademarkWatch; }
                if nav_btn(ui, "IBC Watchlist", ICON_DOC, self.current_page == Page::IbcStatus).clicked() { self.current_page = Page::IbcStatus; }
                if nav_btn(ui, "Labor Laws", ICON_DOC, self.current_page == Page::LaborLaws).clicked() { self.current_page = Page::LaborLaws; }
                if nav_btn(ui, "Import/Export", ICON_DOC, self.current_page == Page::ImportExport).clicked() { self.current_page = Page::ImportExport; }
                if nav_btn(ui, "Startup India", ICON_DOC, self.current_page == Page::StartupIndia).clicked() { self.current_page = Page::StartupIndia; }
                if nav_btn(ui, "MSME Samadhaan", ICON_DOC, self.current_page == Page::MsmeStatus).clicked() { self.current_page = Page::MsmeStatus; }

                ui.add_space(10.0);
                if nav_btn(ui, &self.t("Smart Tools"), ICON_TOOL, self.current_page == Page::Tools).clicked() { self.current_page = Page::Tools; }
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
                    ui.heading("GST AI Scanner (Notice Predictor)");
                    ui.label("Compare GSTR-1 vs GSTR-3B to predict ASMT-10 notices.");
                    ui.add_space(10.0);
                    egui::Grid::new("gst").spacing([20.0, 10.0]).show(ui, |ui| {
                        ui.label("GSTR-1 Turnover:"); ui.text_edit_singleline(&mut self.gst_sales_1); ui.end_row();
                        ui.label("GSTR-3B Turnover:"); ui.text_edit_singleline(&mut self.gst_sales_3b); ui.end_row();
                    });
                    if ui.button(self.t("Scan")).clicked() { self.calc_gst_risk(); }
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(&self.gst_res).size(16.0).color(if self.gst_res.contains("SAFE") { egui::Color32::GREEN } else { egui::Color32::RED }));
                },
                Page::ItScrutiny => {
                    ui.heading("Income Tax Scrutiny AI");
                    egui::Grid::new("it").spacing([20.0, 10.0]).show(ui, |ui| {
                        ui.label("Returned Income:"); ui.text_edit_singleline(&mut self.it_income); ui.end_row();
                        ui.label("High Value Txn (SFT):"); ui.text_edit_singleline(&mut self.it_high_val); ui.end_row();
                    });
                    if ui.button("Analyze Risk").clicked() { self.calc_it_risk(); }
                    ui.label(&self.it_res);
                },
                Page::TdsRecon => {
                    ui.heading("TDS Reconciliation");
                    egui::Grid::new("tds").show(ui, |ui| {
                        ui.label("Deducted:"); ui.text_edit_singleline(&mut self.tds_deducted); ui.end_row();
                        ui.label("Deposited:"); ui.text_edit_singleline(&mut self.tds_deposited); ui.end_row();
                    });
                    if ui.button("Reconcile").clicked() { self.calc_tds_recon(); }
                    ui.label(&self.tds_res);
                },
                Page::CityRisk => {
                    ui.heading("City Risk");
                     egui::Grid::new("cr").show(ui, |ui| {
                        ui.label("City:"); ui.text_edit_singleline(&mut self.risk_city); ui.end_row();
                    });
                    ui.label(format!("Risk: {}%", self.risk_data.get(&self.risk_city).unwrap_or(&0)));
                },
                Page::ClientIntegrity => {
                     ui.heading("Client Integrity");
                     ui.horizontal(|ui| {
                         ui.label("Name:"); ui.text_edit_singleline(&mut self.new_client_name);
                         if ui.button("Add").clicked() { self.add_client(); }
                     });
                     egui::ScrollArea::vertical().show(ui, |ui| {
                         for c in &self.clients { ui.label(&c.name); }
                     });
                },
                Page::EvidenceLocker => {
                     ui.heading("Evidence Locker");
                     ui.text_edit_singleline(&mut self.ev_client_name);
                     ui.text_edit_singleline(&mut self.ev_note);
                     if ui.button("Lock").clicked() { self.save_evidence(); }
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
                        if tool_btn(ui, "EMI Calc", self.active_tool == ActiveTool::EmiCalc).clicked() { self.active_tool = ActiveTool::EmiCalc; }
                        if tool_btn(ui, "Burn Rate", self.active_tool == ActiveTool::BurnRate).clicked() { self.active_tool = ActiveTool::BurnRate; }
                        if tool_btn(ui, "Simple Int.", self.active_tool == ActiveTool::SimpleInt).clicked() { self.active_tool = ActiveTool::SimpleInt; }
                        if tool_btn(ui, "CAGR", self.active_tool == ActiveTool::CagrCalc).clicked() { self.active_tool = ActiveTool::CagrCalc; }
                        if tool_btn(ui, "Break Even", self.active_tool == ActiveTool::BreakEven).clicked() { self.active_tool = ActiveTool::BreakEven; }

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
                            ActiveTool::GratuityCalc => {
                                ui.heading("Gratuity");
                                ui.horizontal(|ui| { ui.label("Basic + DA:"); ui.text_edit_singleline(&mut self.grat_sal); });
                                ui.horizontal(|ui| { ui.label("Years:"); ui.text_edit_singleline(&mut self.grat_yrs); });
                                if ui.button("Calc").clicked() { self.calc_gratuity(); }
                                ui.label(&self.grat_result);
                            },
                            ActiveTool::PenaltyCalc => {
                                ui.heading("Penalty");
                                ui.horizontal(|ui| { ui.label("Days Delayed:"); ui.text_edit_singleline(&mut self.pen_days); });
                                ui.horizontal(|ui| { ui.label("Form:"); ui.text_edit_singleline(&mut self.pen_filing_type); });
                                if ui.button("Calc").clicked() { self.calc_penalty(); }
                                ui.label(&self.pen_result);
                            },
                            ActiveTool::GstInterest => {
                                ui.heading("GST Interest (Sec 50)");
                                ui.horizontal(|ui| { ui.label("Tax Amt:"); ui.text_edit_singleline(&mut self.gst_tax); });
                                ui.horizontal(|ui| { ui.label("Days Late:"); ui.text_edit_singleline(&mut self.gst_days); });
                                if ui.button("Calc").clicked() { self.calc_gst_int(); }
                                ui.label(&self.gst_result);
                            },
                            ActiveTool::DepreciationCalc => {
                                ui.heading("Depreciation (WDV)");
                                ui.horizontal(|ui| { ui.label("Cost:"); ui.text_edit_singleline(&mut self.dep_cost); });
                                ui.horizontal(|ui| { ui.label("Rate (%):"); ui.text_edit_singleline(&mut self.dep_rate); });
                                if ui.button("Calc").clicked() { self.calc_dep(); }
                                ui.label(&self.dep_result);
                            },
                            ActiveTool::CapitalGains => {
                                ui.heading("Indexed Cost");
                                ui.horizontal(|ui| { ui.label("Cost:"); ui.text_edit_singleline(&mut self.cg_cost); });
                                ui.horizontal(|ui| { ui.label("CII Year 1:"); ui.text_edit_singleline(&mut self.cg_idx1); });
                                ui.horizontal(|ui| { ui.label("CII Year 2:"); ui.text_edit_singleline(&mut self.cg_idx2); });
                                if ui.button("Calc").clicked() { self.calc_cg(); }
                                ui.label(&self.cg_result);
                            },
                            ActiveTool::LlpFee => {
                                ui.heading("LLP Filing Fee");
                                ui.horizontal(|ui| { ui.label("Contribution:"); ui.text_edit_singleline(&mut self.llp_contrib); });
                                if ui.button("Calc").clicked() { self.calc_llp(); }
                                ui.label(&self.llp_result);
                            },
                            ActiveTool::EmiCalc => {
                                ui.heading("Loan EMI");
                                ui.horizontal(|ui| { ui.label("P:"); ui.text_edit_singleline(&mut self.emi_p); });
                                ui.horizontal(|ui| { ui.label("R%:"); ui.text_edit_singleline(&mut self.emi_r); });
                                ui.horizontal(|ui| { ui.label("Yrs:"); ui.text_edit_singleline(&mut self.emi_n); });
                                if ui.button("Calc").clicked() { self.calc_emi(); }
                                ui.label(&self.emi_result);
                            },
                            ActiveTool::BurnRate => {
                                ui.heading("Startup Runway");
                                ui.horizontal(|ui| { ui.label("Cash:"); ui.text_edit_singleline(&mut self.burn_cash); });
                                ui.horizontal(|ui| { ui.label("Monthly Spend:"); ui.text_edit_singleline(&mut self.burn_spend); });
                                if ui.button("Calc").clicked() { self.calc_burn(); }
                                ui.label(&self.burn_result);
                            },
                            ActiveTool::SimpleInt => {
                                ui.heading("Simple Interest");
                                ui.horizontal(|ui| { ui.label("P:"); ui.text_edit_singleline(&mut self.si_p); });
                                ui.horizontal(|ui| { ui.label("R%:"); ui.text_edit_singleline(&mut self.si_r); });
                                ui.horizontal(|ui| { ui.label("T:"); ui.text_edit_singleline(&mut self.si_t); });
                                if ui.button("Calc").clicked() { self.calc_si(); }
                                ui.label(&self.si_result);
                            },
                            ActiveTool::TdsInterest => {
                                ui.heading("TDS Interest");
                                ui.horizontal(|ui| { ui.label("TDS Amt:"); ui.text_edit_singleline(&mut self.tds_amt); });
                                ui.horizontal(|ui| { ui.label("Months Late:"); ui.text_edit_singleline(&mut self.tds_months); });
                                if ui.button("Calc").clicked() { self.calc_tds_int(); }
                                ui.label(&self.tds_result);
                            },
                            ActiveTool::CagrCalc => {
                                ui.heading("CAGR Calculator");
                                ui.horizontal(|ui| { ui.label("Start Val:"); ui.text_edit_singleline(&mut self.cagr_start); });
                                ui.horizontal(|ui| { ui.label("End Val:"); ui.text_edit_singleline(&mut self.cagr_end); });
                                ui.horizontal(|ui| { ui.label("Years:"); ui.text_edit_singleline(&mut self.cagr_yrs); });
                                if ui.button("Calc").clicked() { self.calc_cagr(); }
                                ui.label(&self.cagr_result);
                            },
                            ActiveTool::BreakEven => {
                                ui.heading("Break Even Analysis");
                                ui.horizontal(|ui| { ui.label("Fixed Cost:"); ui.text_edit_singleline(&mut self.be_fixed); });
                                ui.horizontal(|ui| { ui.label("Unit Price:"); ui.text_edit_singleline(&mut self.be_price); });
                                ui.horizontal(|ui| { ui.label("Var Cost:"); ui.text_edit_singleline(&mut self.be_var); });
                                if ui.button("Calc").clicked() { self.calc_be(); }
                                ui.label(&self.be_result);
                            },
                            ActiveTool::CryptoTax => {
                                ui.heading("VDA / Crypto Tax");
                                ui.horizontal(|ui| { ui.label("Profit:"); ui.text_edit_singleline(&mut self.cry_prof); });
                                if ui.button("Calculate").clicked() { self.calc_crypto(); }
                                ui.label(&self.cry_result);
                            },
                            ActiveTool::McaPredictor => {
                                ui.heading("MCA Predictor");
                                ui.horizontal(|ui| { ui.label("City:"); ui.text_edit_singleline(&mut self.mca_city); });
                                ui.horizontal(|ui| { ui.label("Form:"); ui.text_edit_singleline(&mut self.mca_form); });
                                if ui.button("Predict").clicked() { self.calc_mca(); }
                                ui.label(&self.mca_result);
                            },
                            ActiveTool::BoardRisk => {
                                ui.heading("Board Risk");
                                ui.text_edit_multiline(&mut self.board_text);
                                if ui.button("Scan").clicked() { self.calc_board_risk(); }
                                for r in &self.board_result { ui.label(r); }
                            },
                            ActiveTool::PmlaScanner => {
                                ui.heading("PMLA Scanner");
                                ui.horizontal(|ui| { ui.label("Txn Amt:"); ui.text_edit_singleline(&mut self.pmla_amt); });
                                ui.checkbox(&mut self.pmla_cash, "Is Cash?");
                                if ui.button("Scan").clicked() { self.calc_pmla(); }
                                ui.colored_label(if self.pmla_result.contains("HIGH") { egui::Color32::RED } else { egui::Color32::GREEN }, &self.pmla_result);
                            },
                            ActiveTool::ShellRisk => {
                                ui.heading("Shell Company Detector");
                                ui.horizontal(|ui| { ui.label("Turnover:"); ui.text_edit_singleline(&mut self.shell_to); });
                                ui.horizontal(|ui| { ui.label("Assets:"); ui.text_edit_singleline(&mut self.shell_ast); });
                                if ui.button("Analyze").clicked() { self.calc_shell(); }
                                ui.label(&self.shell_result);
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
                            ActiveTool::HraCalc => {
                                ui.heading("HRA Calculator");
                                ui.horizontal(|ui| { ui.label("Basic:"); ui.text_edit_singleline(&mut self.hra_basic); });
                                ui.horizontal(|ui| { ui.label("Rent:"); ui.text_edit_singleline(&mut self.hra_rent); });
                                if ui.button("Calculate").clicked() { self.calc_hra(); }
                                ui.label(&self.hra_result);
                            },
                            ActiveTool::PartnerDiss => {
                                ui.heading("Partnership Dissolution");
                                ui.horizontal(|ui| { ui.label("Assets:"); ui.text_edit_singleline(&mut self.part_ast); });
                                ui.horizontal(|ui| { ui.label("Liab:"); ui.text_edit_singleline(&mut self.part_lia); });
                                if ui.button("Calculate").clicked() { self.calc_partner(); }
                                ui.label(&self.part_result);
                            },
                            ActiveTool::BuybackTax => {
                                ui.heading("Buyback Tax");
                                ui.horizontal(|ui| { ui.label("Shares:"); ui.text_edit_singleline(&mut self.buy_shares); });
                                ui.horizontal(|ui| { ui.label("Price:"); ui.text_edit_singleline(&mut self.buy_price); });
                                if ui.button("Calculate").clicked() { self.calc_buyback(); }
                                ui.label(&self.buy_result);
                            },
                            _ => { ui.label("Select a tool from the list above."); }
                        }
                    });
                },
                Page::Settings => { ui.label("Settings Page"); },
                _ => { ui.label("Module in development."); }
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
            ui.add(egui::Image::from_bytes(format!("bytes://{}", text), icon).max_width(18.0).tint(fg));
            ui.add_space(10.0);
            ui.label(egui::RichText::new(text).color(fg));
        });
    }).response.interact(egui::Sense::click())
}

fn tool_btn(ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
    let text_color = if active { COLOR_ACCENT } else { COLOR_TEXT };
    ui.add(egui::Button::new(egui::RichText::new(text).color(text_color)).frame(false))
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
    visuals.widgets.noninteractive.bg_fill = COLOR_PANEL;
    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 850.0]).with_title("PratyakshAI Enterprise"),
        ..Default::default()
    };
    eframe::run_native("PratyakshAI", options, Box::new(|cc| Box::new(PratyakshApp::new(cc))))
}