from datetime import date, timedelta

class ComplianceCalculator:
    
    @staticmethod
    def calculate_due_dates(fy_end_date: date):
        # AGM Deadline (30th Sept usually)
        agm_deadline = fy_end_date + timedelta(days=183) 
        
        # Filing Deadlines
        aoc4_due = agm_deadline + timedelta(days=30)
        mgt7_due = agm_deadline + timedelta(days=60)
        
        return {
            "agm_deadline": agm_deadline,
            "aoc_4_due": aoc4_due,
            "mgt_7_due": mgt7_due
        }

    @staticmethod
    def calculate_penalty(due_date: date, actual_date: date = None) -> dict:
        if not actual_date:
            actual_date = date.today()
            
        if actual_date <= due_date:
            return {"days_delayed": 0, "additional_fee": 0, "risk_level": "SAFE"}
            
        delay = (actual_date - due_date).days
        normal_fee = 300 
        
        if delay <= 30: factor = 2
        elif 30 < delay <= 60: factor = 4
        elif 60 < delay <= 90: factor = 6
        elif 90 < delay <= 180: factor = 10
        else: factor = 12
            
        penalty = normal_fee * factor
        
        return {
            "days_delayed": delay,
            "additional_fee": penalty,
            "risk_level": "HIGH" if delay > 60 else "MODERATE"
        }
