from fastapi import FastAPI
from app.modules.compliance.logic import ComplianceCalculator
from datetime import date

app = FastAPI(title="PratyakshAI CS Suite")

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
