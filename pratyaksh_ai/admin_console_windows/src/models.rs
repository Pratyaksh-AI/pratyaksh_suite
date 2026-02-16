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

#[derive(Debug, Serialize, Deserialize)]
pub struct FirestoreDocument {
    pub name: String,
    pub fields: PaymentFields,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    pub stringValue: String,
}

// User Access Schema (For unlocking Download.jsx)
#[derive(Serialize)]
pub struct AccessGrant {
    pub fields: AccessFields,
}

#[derive(Serialize)]
pub struct AccessFields {
    pub canDownload: BooleanValue,
    pub plan: StringValue,
    pub grantedAt: StringValue,
}

#[derive(Serialize)]
pub struct BooleanValue {
    pub booleanValue: bool,
}