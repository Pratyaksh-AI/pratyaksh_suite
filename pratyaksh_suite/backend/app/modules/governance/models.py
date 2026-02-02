from sqlalchemy import Column, Integer, String, Date, Boolean, ForeignKey, Enum
from sqlalchemy.orm import relationship
from ...db.session import Base
import enum

class DINStatus(str, enum.Enum):
    APPROVED = "APPROVED"
    DISQUALIFIED = "DISQUALIFIED"
    DEACTIVATED = "DEACTIVATED"

class Director(Base):
    __tablename__ = "directors"
    
    id = Column(Integer, primary_key=True, index=True)
    din = Column(String(8), unique=True, index=True) # Real 8-digit DIN
    full_name = Column(String)
    status = Column(Enum(DINStatus), default=DINStatus.APPROVED)
    
    # Sec 164 Check: Date of last non-compliance
    disqualification_date = Column(Date, nullable=True)
    disqualification_reason = Column(String, nullable=True)

class BoardResolution(Base):
    __tablename__ = "board_resolutions"
    
    id = Column(Integer, primary_key=True, index=True)
    title = Column(String)
    agenda_text = Column(String) # The text we analyze for risk
    risk_score = Column(Integer) # 0-100
    risk_flags = Column(String) # "SEC_185_VIOLATION, RELATED_PARTY"
