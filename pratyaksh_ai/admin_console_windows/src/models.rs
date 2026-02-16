use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentRequest {
    // Firestore fields often come wrapped, but for this REST client
    // we will map the flat JSON structure we expect.
    #[serde(rename = "userId")]
    pub user_id: String,
    
    #[serde(rename = "userEmail")]
    pub email: String,
    
    pub amount: String,
    pub plan: String,
    pub status: String, // "pending", "approved", "denied"
    
    #[serde(rename = "txnId")]
    pub txn_id: String,
    
    pub device: String,
    
    // Internal use for UI (not in DB)
    #[serde(skip)]
    pub doc_path: String, 
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FirestoreDocument {
    pub name: String,
    pub fields: PaymentFields,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFields {
    pub userId: StringValue,
    pub userEmail: StringValue,
    pub amount: StringValue,
    pub plan: StringValue,
    pub status: StringValue,
    pub txnId: StringValue,
    pub device: StringValue,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    pub stringValue: String,
}

// User Access Schema (For unlocking Download.jsx)
#[allow(dead_code)]
#[derive(Serialize)]
pub struct AccessGrant {
    pub fields: AccessFields,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AccessFields {
    pub canDownload: BooleanValue,
    pub plan: StringValue,
    pub grantedAt: StringValue,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct BooleanValue {
    pub booleanValue: bool,
}

// --- NEW DATA STRUCTURES ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessRecord {
    pub user_id: String,
    pub plan: String,
    pub granted_at: String,
    pub can_download: bool,
}

#[derive(Debug, Default, Clone)]
pub struct DashboardStats {
    pub total_pending: usize,
    pub total_approved: usize,
    pub total_denied: usize,
    pub total_revenue: f64, // Calculated from parsed amount strings
    pub currency_symbol: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionHistory {
    pub requests: Vec<PaymentRequest>,
}