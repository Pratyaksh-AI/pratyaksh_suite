from fastapi import APIRouter
from .logic import RegionalLawEngine

router = APIRouter()

@router.get("/calc/stamp-duty")
def get_stamp_duty(state: str, instrument: str, value: float):
    return RegionalLawEngine.calculate_stamp_duty(state, instrument, value)
