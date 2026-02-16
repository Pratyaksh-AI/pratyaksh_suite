use crate::models::*;
use reqwest::blocking::Client;
use serde_json::json;

// CONFIGURATION (Replace these with your real Project ID if different)
const PROJECT_ID: &str = "pratyakshai-website"; 
const APP_ID_PATH: &str = "pratyaksh_ai_suite"; // Must match your React AppId

pub struct FirebaseClient {
    client: Client,
    base_url: String,
}

impl FirebaseClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: format!("https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents", PROJECT_ID),
        }
    }

    // 1. FETCH PENDING PAYMENTS
    pub fn fetch_pending(&self) -> Result<Vec<PaymentRequest>, String> {
        let url = format!("{}/artifacts/{}/public/data/payments", self.base_url, APP_ID_PATH);
        
        let resp = self.client.get(&url).send().map_err(|e| e.to_string())?;
        
        if !resp.status().is_success() {
            return Err(format!("API Error: {}", resp.status()));
        }

        let json: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
        
        let mut requests = Vec::new();
        
        if let Some(docs) = json.get("documents").and_then(|d| d.as_array()) {
            for doc in docs {
                if let Some(fields) = doc.get("fields") {
                    // Manual extraction to handle Firestore's verbose JSON format
                    let status = fields.get("status").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("");
                    
                    if status == "pending" {
                        let name = doc.get("name").and_then(|s| s.as_str()).unwrap_or("");
                        
                        requests.push(PaymentRequest {
                            user_id: fields.get("userId").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            email: fields.get("userEmail").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            amount: fields.get("amount").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            plan: fields.get("plan").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            status: status.to_string(),
                            txn_id: fields.get("txnId").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            device: fields.get("device").and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            doc_path: name.to_string(),
                        });
                    }
                }
            }
        }
        Ok(requests)
    }

    // 2. APPROVE USER (Updates status AND creates User Access record)
    pub fn approve_request(&self, req: &PaymentRequest) -> Result<(), String> {
        // A. Update Payment Status to 'approved'
        let payment_update_url = format!("https://firestore.googleapis.com/v1/{}?updateMask.fieldPaths=status", req.doc_path);
        let body = json!({
            "fields": {
                "status": { "stringValue": "approved" }
            }
        });
        
        self.client.patch(&payment_update_url).json(&body).send().map_err(|e| e.to_string())?;

        // B. Grant Access (Create record in user_access collection)
        // This effectively unlocks Download.jsx
        // PREFIXED WITH UNDERSCORE TO FIX WARNING
        let _access_url = format!("{}/artifacts/{}/public/data/user_access?documentId={}", self.base_url, APP_ID_PATH, req.user_id);
        
        let access_body = json!({
            "fields": {
                "canDownload": { "booleanValue": true },
                "plan": { "stringValue": req.plan },
                "grantedAt": { "stringValue": chrono::Local::now().to_rfc3339() }
            }
        });

        // Use POST with documentId, or PATCH if it might exist
        let patch_access_url = format!("{}/artifacts/{}/public/data/user_access/{}", self.base_url, APP_ID_PATH, req.user_id);
        self.client.patch(&patch_access_url).json(&access_body).send().map_err(|e| e.to_string())?;

        Ok(())
    }

    // 3. DENY USER
    pub fn deny_request(&self, req: &PaymentRequest) -> Result<(), String> {
        let url = format!("https://firestore.googleapis.com/v1/{}?updateMask.fieldPaths=status", req.doc_path);
        let body = json!({
            "fields": {
                "status": { "stringValue": "denied" }
            }
        });
        self.client.patch(&url).json(&body).send().map_err(|e| e.to_string())?;
        Ok(())
    }
}