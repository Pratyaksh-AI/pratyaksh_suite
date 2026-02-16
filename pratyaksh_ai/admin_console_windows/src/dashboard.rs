use eframe::egui;
use crate::models::{PaymentRequest, UserAccessRecord, DashboardStats};

#[derive(PartialEq, Clone, Copy)]
pub enum DashboardTab {
    Pending,
    ApprovedUsers,
    FullHistory,
    Statistics,
}

pub fn render_dashboard(
    ui: &mut egui::Ui, 
    current_tab: &mut DashboardTab,
    pending_requests: &Vec<PaymentRequest>, 
    approved_users: &Vec<UserAccessRecord>,
    full_history: &Vec<PaymentRequest>,
    stats: &DashboardStats,
    selected_id: &mut Option<String>,
    search_query: &mut String,
    on_approve: &mut dyn FnMut(&PaymentRequest),
    on_deny: &mut dyn FnMut(&PaymentRequest),
    on_refresh: &mut dyn FnMut()
) {
    // --- TOP BAR & STATS OVERVIEW ---
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("PRATYAKSH ADMIN").size(24.0).strong().color(egui::Color32::WHITE));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 Refresh Data").clicked() {
                    on_refresh();
                }
                ui.label(egui::RichText::new("● Live").color(egui::Color32::GREEN));
            });
        });

        ui.add_space(10.0);

        // Stats Overview Row
        ui.horizontal(|ui| {
            render_stat_widget(ui, "Total Approvals", &stats.total_approved.to_string(), egui::Color32::from_rgb(79, 249, 120));
            render_stat_widget(ui, "Pending", &stats.total_pending.to_string(), egui::Color32::GOLD);
            render_stat_widget(ui, "Revenue", &format!("{}{:.2}", stats.currency_symbol, stats.total_revenue), egui::Color32::WHITE);
        });
    });
    
    ui.add_space(20.0);
    ui.separator();
    ui.add_space(10.0);

    // --- TAB NAVIGATION ---
    ui.horizontal(|ui| {
        ui.selectable_value(current_tab, DashboardTab::Pending, "📥 Pending Requests");
        ui.selectable_value(current_tab, DashboardTab::ApprovedUsers, "👥 Approved Users");
        ui.selectable_value(current_tab, DashboardTab::FullHistory, "📜 Full History");
        ui.selectable_value(current_tab, DashboardTab::Statistics, "📊 Analytics");
    });

    ui.add_space(20.0);

    // --- TAB CONTENT ---
    match current_tab {
        DashboardTab::Pending => render_pending_tab(ui, pending_requests, selected_id, on_approve, on_deny),
        DashboardTab::ApprovedUsers => render_approved_users_tab(ui, approved_users, search_query),
        DashboardTab::FullHistory => render_history_tab(ui, full_history, search_query),
        DashboardTab::Statistics => render_statistics_tab(ui, stats),
    }
}

// --- TAB: PENDING REQUESTS (Legacy Split View) ---
fn render_pending_tab(
    ui: &mut egui::Ui,
    requests: &Vec<PaymentRequest>,
    selected_id: &mut Option<String>,
    on_approve: &mut dyn FnMut(&PaymentRequest),
    on_deny: &mut dyn FnMut(&PaymentRequest)
) {
    ui.columns(2, |cols| {
        // Left Column: List
        cols[0].vertical(|ui| {
            ui.heading("Awaiting Verification");
            ui.add_space(10.0);
            
            egui::ScrollArea::vertical().id_source("pending_scroll").show(ui, |ui| {
                if requests.is_empty() {
                    ui.label("No pending payments.");
                }
                
                for req in requests {
                    let is_selected = Some(req.user_id.clone()) == *selected_id;
                    let bg = if is_selected { egui::Color32::from_rgb(0, 80, 0) } else { egui::Color32::from_rgb(30, 30, 30) };
                    
                    egui::Frame::none().fill(bg).inner_margin(10.0).rounding(5.0).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&req.email).strong().color(egui::Color32::WHITE));
                        });
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&req.amount).color(egui::Color32::from_rgb(79, 249, 120)));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(&req.txn_id);
                            });
                        });
                    }).response.interact(egui::Sense::click()).clicked().then(|| {
                        *selected_id = Some(req.user_id.clone());
                    });
                    ui.add_space(5.0);
                }
            });
        });

        // Right Column: Details
        cols[1].vertical(|ui| {
            if let Some(sel_id) = selected_id {
                if let Some(req) = requests.iter().find(|r| r.user_id == *sel_id) {
                    render_detail_view(ui, req, on_approve, on_deny);
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a transaction to view details.");
                });
            }
        });
    });
}

// --- TAB: APPROVED USERS ---
fn render_approved_users_tab(ui: &mut egui::Ui, users: &Vec<UserAccessRecord>, query: &mut String) {
    ui.horizontal(|ui| {
        ui.label("🔍 Search User ID:");
        ui.text_edit_singleline(query);
    });
    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("approved_grid").striped(true).min_col_width(150.0).show(ui, |ui| {
            ui.label(egui::RichText::new("User ID").strong());
            ui.label(egui::RichText::new("Plan").strong());
            ui.label(egui::RichText::new("Granted At").strong());
            ui.label(egui::RichText::new("Status").strong());
            ui.end_row();

            for user in users.iter().filter(|u| u.user_id.contains(query.as_str())) {
                ui.monospace(&user.user_id);
                ui.label(&user.plan);
                ui.label(&user.granted_at);
                ui.label(if user.can_download { "✅ Active" } else { "❌ Revoked" });
                ui.end_row();
            }
        });
    });
}

// --- TAB: FULL HISTORY ---
fn render_history_tab(ui: &mut egui::Ui, history: &Vec<PaymentRequest>, query: &mut String) {
    ui.horizontal(|ui| {
        ui.label("🔍 Filter by Email/Txn:");
        ui.text_edit_singleline(query);
    });
    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("history_grid").striped(true).min_col_width(120.0).show(ui, |ui| {
            ui.label(egui::RichText::new("Transaction ID").strong());
            ui.label(egui::RichText::new("User Email").strong());
            ui.label(egui::RichText::new("Amount").strong());
            ui.label(egui::RichText::new("Status").strong());
            ui.end_row();

            for req in history.iter().filter(|r| r.email.contains(query.as_str()) || r.txn_id.contains(query.as_str())) {
                ui.monospace(&req.txn_id);
                ui.label(&req.email);
                ui.label(&req.amount);
                
                let color = match req.status.as_str() {
                    "approved" => egui::Color32::GREEN,
                    "denied" => egui::Color32::RED,
                    _ => egui::Color32::GOLD,
                };
                ui.label(egui::RichText::new(&req.status).color(color));
                ui.end_row();
            }
        });
    });
}

// --- TAB: STATISTICS ---
fn render_statistics_tab(ui: &mut egui::Ui, stats: &DashboardStats) {
    ui.heading("Financial Analytics");
    ui.add_space(20.0);

    egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 30)).inner_margin(20.0).rounding(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Revenue Breakdown").size(18.0).strong());
            ui.add_space(10.0);
            ui.label(format!("Gross Revenue: {}{:.2}", stats.currency_symbol, stats.total_revenue));
            ui.label(format!("Avg. Ticket Size: {}{:.2}", stats.currency_symbol, if stats.total_approved > 0 { stats.total_revenue / stats.total_approved as f64 } else { 0.0 }));
            
            ui.add_space(20.0);
            ui.label(egui::RichText::new("Request Volume").size(18.0).strong());
            ui.add_space(10.0);
            ui.label(format!("Successful Approvals: {}", stats.total_approved));
            ui.label(format!("Total Denials: {}", stats.total_denied));
            ui.label(format!("Conversion Rate: {:.1}%", if (stats.total_approved + stats.total_denied) > 0 { (stats.total_approved as f64 / (stats.total_approved + stats.total_denied) as f64) * 100.0 } else { 0.0 }));
        });
    });
}

// --- HELPERS ---

fn render_stat_widget(ui: &mut egui::Ui, label: &str, value: &str, color: egui::Color32) {
    egui::Frame::none().fill(egui::Color32::from_rgb(35, 35, 35)).inner_margin(15.0).rounding(8.0).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(label).size(10.0).color(egui::Color32::GRAY));
            ui.label(egui::RichText::new(value).size(20.0).strong().color(color));
        });
    });
    ui.add_space(10.0);
}

fn render_detail_view(
    ui: &mut egui::Ui, 
    req: &PaymentRequest,
    on_approve: &mut dyn FnMut(&PaymentRequest),
    on_deny: &mut dyn FnMut(&PaymentRequest)
) {
    ui.heading("Transaction Details");
    ui.add_space(20.0);

    let _grid_clr = egui::Color32::from_rgb(40, 40, 40);
    
    egui::Grid::new("details").spacing([40.0, 15.0]).show(ui, |ui| {
        ui.label("User Email:"); ui.label(egui::RichText::new(&req.email).strong().size(16.0)); ui.end_row();
        ui.label("Plan Selected:"); ui.label(&req.plan); ui.end_row();
        ui.label("Amount Paid:"); ui.label(egui::RichText::new(&req.amount).color(egui::Color32::GREEN).size(18.0)); ui.end_row();
        ui.label("Transaction ID:"); ui.monospace(&req.txn_id); ui.end_row();
        ui.label("Device:"); ui.label(&req.device); ui.end_row();
        ui.label("User ID:"); ui.monospace(&req.user_id); ui.end_row();
    });

    ui.add_space(50.0);
    ui.separator();
    ui.add_space(20.0);

    ui.horizontal(|ui| {
        let btn_deny = egui::Button::new(egui::RichText::new("🚫 DENY REQUEST").color(egui::Color32::WHITE))
            .fill(egui::Color32::RED)
            .min_size(egui::vec2(150.0, 40.0));
            
        if ui.add(btn_deny).clicked() {
            on_deny(req);
        }

        ui.add_space(20.0);

        let btn_approve = egui::Button::new(egui::RichText::new("✅ APPROVE & UNLOCK").color(egui::Color32::BLACK))
            .fill(egui::Color32::from_rgb(79, 249, 120))
            .min_size(egui::vec2(200.0, 40.0));
            
        if ui.add(btn_approve).clicked() {
            on_approve(req);
        }
    });
}