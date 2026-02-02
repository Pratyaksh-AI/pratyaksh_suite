from fastapi import APIRouter
from .logic import ClientRiskEngine

router = APIRouter()

@router.get("/score-client")
def score_client_endpoint(late_payments: int = 0, deviations: int = 0, litigations: int = 0):
    return ClientRiskEngine.calculate_client_risk(late_payments, deviations, litigations)
