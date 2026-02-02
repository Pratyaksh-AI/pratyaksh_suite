class RegionalLawEngine:
    
    @staticmethod
    def calculate_stamp_duty(state: str, instrument: str, consideration_value: float):
        """
        Real Logic: Maharashtra Stamp Act vs Karnataka Stamp Act
        Focus: 'Share Certificate' issuance (Common CS task)
        """
        duty = 0.0
        state = state.upper()
        
        if instrument == "SHARE_CERTIFICATE":
            if state == "MAHARASHTRA":
                # Real Rate: 0.1% of value (Subject to min/max changes annually)
                duty = consideration_value * 0.001
            elif state == "KARNATAKA":
                # Real Rate: 0.1% but min Rs. 50
                calc = consideration_value * 0.001
                duty = max(50.0, calc)
            else:
                return {"error": "State not covered in Phase 3 yet"}
                
        elif instrument == "MOA":
            if state == "MAHARASHTRA":
                # Simplified slab for MVP
                if consideration_value <= 1000000: # Up to 10 Lakhs
                    duty = 1000
                else:
                    duty = 1000 + ((consideration_value - 1000000) * 0.001) # Example slab
            elif state == "KARNATAKA":
                 duty = 2000 # Flat fee assumption for MVP slab
                 
        return {
            "state": state,
            "instrument": instrument,
            "stamp_duty_payable": round(duty, 2),
            "act_reference": "Maharashtra Stamp Act, 1958" if state == "MAHARASHTRA" else "Karnataka Stamp Act, 1957"
        }
