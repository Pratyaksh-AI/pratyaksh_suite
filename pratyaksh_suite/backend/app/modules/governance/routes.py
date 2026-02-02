from fastapi import APIRouter, HTTPException
from .logic import GovernanceEngine
from pydantic import BaseModel

router = APIRouter()

class ResolutionRequest(BaseModel):
    agenda_text: str

@router.get("/check-din/{din}")
def check_din(din: str):
    validation = GovernanceEngine.check_din_validity(din)
    if not validation["valid"]:
        raise HTTPException(status_code=400, detail=validation["error"])
    return {"din": din, "status": "Format Valid (Mock Lookup for MVP)"}

@router.get("/predict-disqualification")
def predict_risk(years_defaulting: int):
    # This endpoint simulates the "What If" analysis for clients
    return GovernanceEngine.predict_disqualification_risk(years_defaulting)

@router.post("/analyze-resolution")
def analyze_resolution(request: ResolutionRequest):
    return GovernanceEngine.analyze_resolution_text(request.agenda_text)
