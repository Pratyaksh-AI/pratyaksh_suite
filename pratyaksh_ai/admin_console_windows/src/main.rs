#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dashboard;
mod firebase_api;
mod models;

use eframe::egui;
use firebase_api::FirebaseClient;
use models::{PaymentRequest, UserAccessRecord, DashboardStats};
use dashboard::DashboardTab;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AdminApp {
    client: Arc<FirebaseClient>,
    
    // --- Data Stores ---
    pending_requests: Arc<Mutex<Vec<PaymentRequest>>>,
    approved_users: Arc<Mutex<Vec<UserAccessRecord>>>,
    full_history: Arc<Mutex<Vec<PaymentRequest>>>,
    stats: Arc<Mutex<DashboardStats>>,
    
    // --- UI State ---
    current_tab: DashboardTab,
    selected_id: Option<String>,
    search_query: String,
    status: String,
}

impl AdminApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set Dark Theme
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::from_rgb(17, 17, 17);
        visuals.panel_fill = egui::Color32::from_rgb(25, 25, 25);
        cc.egui_ctx.set_visuals(visuals);

        let app = Self {
            client: Arc::new(FirebaseClient::new()),
            pending_requests: Arc::new(Mutex::new(Vec::new())),
            approved_users: Arc::new(Mutex::new(Vec::new())),
            full_history: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(DashboardStats::default())),
            
            current_tab: DashboardTab::Pending,
            selected_id: None,
            search_query: String::new(),
            status: "Ready".to_string(),
        };
        
        // Initial Fetch
        app.refresh_data();
        app
    }

    fn refresh_data(&self) {
        let client = self.client.clone();
        let pending = self.pending_requests.clone();
        let users = self.approved_users.clone();
        let history = self.full_history.clone();
        let stats_store = self.stats.clone();
        
        thread::spawn(move || {
            // 1. Fetch All Payments (History)
            if let Ok(all_data) = client.fetch_all_payments() {
                let mut h_lock = history.lock().unwrap();
                *h_lock = all_data.clone();
                
                // Filter Pending locally for speed
                let pending_data: Vec<PaymentRequest> = all_data.iter()
                    .filter(|r| r.status == "pending")
                    .cloned()
                    .collect();
                
                let mut p_lock = pending.lock().unwrap();
                *p_lock = pending_data;

                // Calculate Stats
                let total_approved = all_data.iter().filter(|r| r.status == "approved").count();
                let total_denied = all_data.iter().filter(|r| r.status == "denied").count();
                let total_pending = all_data.iter().filter(|r| r.status == "pending").count();
                
                // Simple revenue calc (assuming "₹19,999" format)
                let revenue: f64 = all_data.iter()
                    .filter(|r| r.status == "approved")
                    .map(|r| r.amount.replace("₹", "").replace(",", "").parse::<f64>().unwrap_or(0.0))
                    .sum();

                let mut s_lock = stats_store.lock().unwrap();
                *s_lock = DashboardStats {
                    total_pending,
                    total_approved,
                    total_denied,
                    total_revenue: revenue,
                    currency_symbol: "₹".to_string(),
                };
            }

            // 2. Fetch User Access Records
            if let Ok(access_data) = client.fetch_user_access() {
                let mut u_lock = users.lock().unwrap();
                *u_lock = access_data;
            }
        });
    }

    fn approve(&mut self, req: &PaymentRequest) {
        if let Err(e) = self.client.approve_request(req) {
            self.status = format!("Error: {}", e);
        } else {
            self.status = format!("Approved {}", req.email);
            self.refresh_data();
            self.selected_id = None;
        }
    }

    fn deny(&mut self, req: &PaymentRequest) {
        if let Err(e) = self.client.deny_request(req) {
            self.status = format!("Error: {}", e);
        } else {
            self.status = format!("Denied {}", req.email);
            self.refresh_data();
            self.selected_id = None;
        }
    }
}

impl eframe::App for AdminApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clone data to avoid borrow checker issues in closure
        let pending = self.pending_requests.lock().unwrap().clone();
        let approved = self.approved_users.lock().unwrap().clone();
        let history = self.full_history.lock().unwrap().clone();
        let stats = self.stats.lock().unwrap().clone();

        let mut approve_req: Option<PaymentRequest> = None;
        let mut deny_req: Option<PaymentRequest> = None;
        let mut do_refresh = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            dashboard::render_dashboard(
                ui, 
                &mut self.current_tab,
                &pending, 
                &approved,
                &history,
                &stats,
                &mut self.selected_id,
                &mut self.search_query,
                &mut |r| approve_req = Some(r.clone()),
                &mut |r| deny_req = Some(r.clone()),
                &mut || do_refresh = true
            );
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(&self.status);
            });
        });

        // Handle Actions outside the UI lock
        if let Some(r) = approve_req { self.approve(&r); }
        if let Some(r) = deny_req { self.deny(&r); }
        if do_refresh { self.refresh_data(); }
        
        // Auto-refresh every 5 seconds
        ctx.request_repaint_after(Duration::from_secs(5));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Pratyaksh Super Admin"),
        ..Default::default()
    };
    eframe::run_native(
        "PratyakshAdmin",
        options,
        Box::new(|cc| Box::new(AdminApp::new(cc))),
    )
}