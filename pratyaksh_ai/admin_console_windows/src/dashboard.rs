use eframe::egui;
use crate::models::PaymentRequest;

pub fn render_dashboard(
    ui: &mut egui::Ui, 
    requests: &Vec<PaymentRequest>, 
    selected_id: &mut Option<String>,
    on_approve: &mut dyn FnMut(&PaymentRequest),
    on_deny: &mut dyn FnMut(&PaymentRequest),
    on_refresh: &mut dyn FnMut()
) {
    // --- TOP BAR ---
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("PRATYAKSH ADMIN").size(24.0).strong().color(egui::Color32::WHITE));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🔄 Refresh Data").clicked() {
                on_refresh();
            }
            ui.label(egui::RichText::new("● Live").color(egui::Color32::GREEN));
        });
    });
    
    ui.add_space(20.0);

    // --- SPLIT VIEW ---
    egui::SidePanel::left("list_panel")
        .resizable(false)
        .exact_width(300.0)
        .show_inside(ui, |ui| {
            ui.heading("Pending Requests");
            ui.add_space(10.0);
            
            egui::ScrollArea::vertical().show(ui, |ui| {
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

    egui::CentralPanel::default().show_inside(ui, |ui| {
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
}

fn render_detail_view(
    ui: &mut egui::Ui, 
    req: &PaymentRequest,
    on_approve: &mut dyn FnMut(&PaymentRequest),
    on_deny: &mut dyn FnMut(&PaymentRequest)
) {
    ui.heading("Transaction Details");
    ui.add_space(20.0);

    let grid_clr = egui::Color32::from_rgb(40, 40, 40);
    
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