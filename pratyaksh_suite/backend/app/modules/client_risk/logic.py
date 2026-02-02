class ClientRiskEngine:
    
    @staticmethod
    def calculate_client_risk(
        late_payments_count: int,
        compliance_deviations: int,
        litigation_cases: int
    ):
        """
        Real Logic: Generates a 'Trust Score' (0-100).
        Lower is Riskier.
        """
        base_score = 100
        
        # Deduction Logic
        # 1. Financial Risk: If they pay fees late, they risk your cashflow.
        base_score -= (late_payments_count * 5)
        
        # 2. Compliance Risk: If they ignore your advice.
        base_score -= (compliance_deviations * 10)
        
        # 3. Legal Risk: Active court cases.
        base_score -= (litigation_cases * 15)
        
        # Cap limits
        final_score = max(0, min(100, base_score))
        
        status = "TRUSTED"
        if final_score < 50:
            status = "HIGH RISK (Require Advance Payment)"
        elif final_score < 75:
            status = "MODERATE RISK"
            
        return {
            "score": final_score,
            "status": status,
            "advisory": "Collect 100% advance" if final_score < 50 else "Standard Terms"
        }
