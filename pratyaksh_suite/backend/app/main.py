from fastapi import FastAPI
from datetime import date
from app.modules.compliance.logic import ComplianceCalculator

# --- PHASE 2 IMPORTS (Moved to Top) ---
from app.modules.governance import routes as governance_routes

app = FastAPI(title="PratyakshAI CS Suite")

# --- REGISTER ROUTERS ---
app.include_router(governance_routes.router, prefix="/api/v1/governance", tags=["Governance"])

@app.get("/")
def health_check():
    return {"status": "PratyakshAI Core is Running"}

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
