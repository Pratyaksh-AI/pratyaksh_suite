#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dashboard;
mod firebase_api;
mod models;

use eframe::egui;
use firebase_api::FirebaseClient;
use models::PaymentRequest;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AdminApp {
    client: Arc<FirebaseClient>,
    requests: Arc<Mutex<Vec<PaymentRequest>>>,
    selected_id: Option<String>,
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
            requests: Arc::new(Mutex::new(Vec::new())),
            selected_id: None,
            status: "Ready".to_string(),
        };
        
        // Initial Fetch
        app.refresh_data();
        app
    }

    fn refresh_data(&self) {
        let client = self.client.clone();
        let requests = self.requests.clone();
        
        thread::spawn(move || {
            match client.fetch_pending() {
                Ok(data) => {
                    let mut lock = requests.lock().unwrap();
                    *lock = data;
                },
                Err(e) => println!("Fetch Error: {}", e),
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
        let requests_guard = self.requests.lock().unwrap();
        let requests = requests_guard.clone();
        drop(requests_guard);

        let mut approve_req: Option<PaymentRequest> = None;
        let mut deny_req: Option<PaymentRequest> = None;
        let mut do_refresh = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            dashboard::render_dashboard(
                ui, 
                &requests, 
                &mut self.selected_id,
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
        
        // Auto-refresh every 5 seconds (Optional, keeps UI lively)
        ctx.request_repaint_after(Duration::from_secs(5));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Pratyaksh Super Admin"),
        ..Default::default()
    };
    eframe::run_native(
        "PratyakshAdmin",
        options,
        Box::new(|cc| Box::new(AdminApp::new(cc))),
    )
}
