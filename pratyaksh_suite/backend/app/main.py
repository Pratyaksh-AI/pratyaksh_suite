from fastapi import FastAPI
from datetime import date
from app.modules.compliance.logic import ComplianceCalculator

# --- IMPORTS ---
from app.modules.governance import routes as governance_routes
from app.modules.client_risk import routes as client_risk_routes
from app.modules.regional import routes as regional_routes

app = FastAPI(title="PratyakshAI CS Suite (Full MVP)")

# --- REGISTER ALL ROUTERS ---
app.include_router(governance_routes.router, prefix="/api/v1/governance", tags=["Governance"])
app.include_router(client_risk_routes.router, prefix="/api/v1/risk", tags=["Client Risk"])
app.include_router(regional_routes.router, prefix="/api/v1/regional", tags=["Regional Intelligence"])

@app.get("/")
def health_check():
    return {"status": "PratyakshAI Core is Running (Phases 1-3 Active)"}

@app.get("/api/v1/compliance/calculate")
def get_compliance_risk(fy_end: str, filing_type: str):
    try:
        fy_date = date.fromisoformat(fy_end)
        dates = ComplianceCalculator.calculate_due_dates(fy_date)
        
        target_due = dates['aoc_4_due'] if filing_type == 'AOC-4' else dates['mgt_7_due']
        risk = ComplianceCalculator.calculate_penalty(target_due)
        
        return {
            "deadlines": dates,
            "current_status": risk
        }
    except Exception as e:
        return {"error": str(e)}
