from sqlalchemy import Column, Integer, String, DateTime, Float, ForeignKey
from sqlalchemy.sql import func
from ...db.session import Base

class EvidenceLog(Base):
    __tablename__ = "evidence_logs"
    
    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.id")) 
    evidence_type = Column(String) 
    description = Column(String)
    file_url = Column(String) 
    file_hash = Column(String) 
    latitude = Column(Float)
    longitude = Column(Float)
    captured_at = Column(DateTime(timezone=True), server_default=func.now())
