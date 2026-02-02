import re
from datetime import date

class GovernanceEngine:
    
    @staticmethod
    def check_din_validity(din: str):
        """
        Real Logic: Validates DIN format (8 digits).
        """
        if not re.match(r"^\d{8}$", din):
            return {"valid": False, "error": "Invalid DIN Format. Must be 8 digits."}
        return {"valid": True}

    @staticmethod
    def predict_disqualification_risk(non_filing_years: int):
        """
        Real Logic: Section 164(2)(a) - Companies Act 2013
        A director is disqualified if the company fails to file 
        Financials/Annual Returns for 3 consecutive years.
        """
        if non_filing_years >= 3:
            return {
                "status": "CRITICAL",
                "risk_score": 100,
                "message": "IMMEDIATE DISQUALIFICATION RISK (Sec 164(2)). Director office will be vacated."
            }
        elif non_filing_years == 2:
            return {
                "status": "HIGH",
                "risk_score": 75,
                "message": "Warning: 1 more year of non-filing will trigger disqualification."
            }
        else:
            return {
                "status": "SAFE",
                "risk_score": 0,
                "message": "Compliance is track."
            }

    @staticmethod
    def analyze_resolution_text(text: str):
        """
        Real Logic: Keyword analysis for Section 185 (Loans to Directors) 
        and Section 188 (Related Party Transactions).
        """
        text = text.lower()
        flags = []
        score = 0
        
        # Section 185 Check
        if "loan" in text and ("director" in text or "relative" in text):
            flags.append("POTENTIAL SEC 185 VIOLATION: Loan to Director detected.")
            score += 50
            
        # Section 188 Check
        if "contract" in text and ("relative" in text or "subsidiary" in text):
            flags.append("POTENTIAL SEC 188 RPT: Board approval required.")
            score += 30
            
        return {
            "risk_score": min(score, 100),
            "flags": flags
        }
