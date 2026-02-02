package com.pratyaksh_ai

import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.RadioButton
import android.widget.RadioGroup
import androidx.cardview.widget.CardView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.GET
import retrofit2.http.Query

// --- DATA MODEL ---
data class ComplianceRisk(
    val penalty_estimate: Int,
    val risk_level: String,
    val act_section: String
)

// --- API ---
interface PratyakshApi {
    @GET("/api/v1/compliance/analyze")
    suspend fun analyze(
        @Query("fy_end_date") fyEnd: String,
        @Query("form_type") formType: String
    ): ComplianceRisk
}

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // **IMPORTANT**: CHANGE THIS URL TO YOUR CODESPACES FORWARDED URL
        // Example: "https://musical-space-waddle-xxxxxx.github.dev/"
        val BACKEND_URL = "http://10.0.2.2:8080/" 

        val retrofit = Retrofit.Builder()
            .baseUrl(BACKEND_URL)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
        val api = retrofit.create(PratyakshApi::class.java)

        val etDate = findViewById<EditText>(R.id.etFyDate)
        val rgForm = findViewById<RadioGroup>(R.id.rgFormType)
        val btnCheck = findViewById<Button>(R.id.btnAnalyze)
        
        // Result Card Views
        val cardResult = findViewById<CardView>(R.id.cardResult)
        val txtRisk = findViewById<TextView>(R.id.txtRiskLevel)
        val txtPenalty = findViewById<TextView>(R.id.txtPenalty)
        val txtSection = findViewById<TextView>(R.id.txtSection)

        btnCheck.setOnClickListener {
            val date = etDate.text.toString()
            if (date.isEmpty()) return@setOnClickListener

            val selectedId = rgForm.checkedRadioButtonId
            val radioButton = findViewById<RadioButton>(selectedId)
            val formType = radioButton.text.toString()

            btnCheck.text = "ANALYZING..."
            btnCheck.isEnabled = false

            CoroutineScope(Dispatchers.IO).launch {
                try {
                    val result = api.analyze(date, formType)
                    
                    withContext(Dispatchers.Main) {
                        cardResult.visibility = View.VISIBLE
                        btnCheck.text = "ANALYZE RISK"
                        btnCheck.isEnabled = true
                        
                        txtRisk.text = result.risk_level
                        txtPenalty.text = "Penalty Estimate: ₹${result.penalty_estimate}"
                        txtSection.text = "Violation under ${result.act_section}"

                        if (result.risk_level == "CRITICAL") {
                            txtRisk.setTextColor(ContextCompat.getColor(this@MainActivity, R.color.risk_critical))
                        } else {
                            txtRisk.setTextColor(ContextCompat.getColor(this@MainActivity, R.color.risk_safe))
                        }
                    }
                } catch (e: Exception) {
                    withContext(Dispatchers.Main) {
                        btnCheck.text = "ERROR: RETRY"
                        btnCheck.isEnabled = true
                        // Show error in a simplified way for MVP
                        txtRisk.text = "CONNECTION ERROR"
                        txtPenalty.text = "Is the Backend URL correct?"
                        txtSection.text = e.message
                        cardResult.visibility = View.VISIBLE
                    }
                }
            }
        }
    }
}
