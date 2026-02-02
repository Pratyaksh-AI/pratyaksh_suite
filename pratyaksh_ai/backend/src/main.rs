use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::Query,
};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Duration, Utc, Datelike};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct ComplianceRisk {
    agm_due_date: NaiveDate,
    filing_due_date: NaiveDate,
    penalty_estimate: i32,
    risk_level: String,
    act_section: String,
}

#[derive(Deserialize)]
struct RiskQuery {
    fy_end_date: String, // YYYY-MM-DD
    form_type: String,   // "AOC-4" or "MGT-7"
}

// REAL LOGIC ENGINE
fn calculate_risk(fy_end: NaiveDate, form_type: &str) -> ComplianceRisk {
    // Sec 96: AGM within 6 months
    let agm_deadline = fy_end + Duration::days(180);

    // Sec 137 vs Sec 92
    let (filing_deadline, section) = match form_type {
        "MGT-7" => (agm_deadline + Duration::days(60), "Sec 92"),
        _ => (agm_deadline + Duration::days(30), "Sec 137"),
    };

    let today = Utc::now().date_naive();
    let mut penalty = 0;
    let mut risk = "SAFE".to_string();

    if today > filing_deadline {
        let days_late = (today - filing_deadline).num_days();
        
        // Companies (Registration Offices and Fees) Rules, 2014
        let factor = match days_late {
            0..=30 => 2,
            31..=60 => 4,
            61..=90 => 6,
            91..=180 => 10,
            _ => 12,
        };
        penalty = 300 * factor;
        risk = if days_late > 60 { "CRITICAL" } else { "MODERATE" }.to_string();
    }

    ComplianceRisk {
        agm_due_date: agm_deadline,
        filing_due_date: filing_deadline,
        penalty_estimate: penalty,
        risk_level: risk,
        act_section: section.to_string(),
    }
}

async fn analyze_risk(Query(params): Query<RiskQuery>) -> Json<ComplianceRisk> {
    let fy_date = NaiveDate::parse_from_str(&params.fy_end_date, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());
    Json(calculate_risk(fy_date, &params.form_type))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/v1/compliance/analyze", get(analyze_risk))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Backend listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}