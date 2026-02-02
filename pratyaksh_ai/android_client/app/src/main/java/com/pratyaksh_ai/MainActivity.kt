package com.pratyaksh_ai

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.RadioButton
import android.widget.RadioGroup
import androidx.appcompat.app.AppCompatActivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.GET
import retrofit2.http.Query

// --- DATA MODEL (Matches Rust Backend) ---
data class ComplianceRisk(
    val penalty_estimate: Int,
    val risk_level: String,
    val act_section: String
)

// --- API INTERFACE ---
interface PratyakshApi {
    @GET("/api/v1/compliance/analyze")
    suspend fun analyze(
        @Query("fy_end_date") fyEnd: String,
        @Query("form_type") formType: String
    ): ComplianceRisk
}

// --- MAIN ACTIVITY ---
class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // NETWORK CONFIG
        // 10.0.2.2 is the special IP for Android Emulator to reach Localhost
        val retrofit = Retrofit.Builder()
            .baseUrl("http://10.0.2.2:8080/") 
            .addConverterFactory(GsonConverterFactory.create())
            .build()
        val api = retrofit.create(PratyakshApi::class.java)

        // UI BINDING
        val etDate = findViewById<EditText>(R.id.etFyDate)
        val rgForm = findViewById<RadioGroup>(R.id.rgFormType)
        val btnCheck = findViewById<Button>(R.id.btnAnalyze)
        val txtResult = findViewById<TextView>(R.id.txtResult)

        btnCheck.setOnClickListener {
            val date = etDate.text.toString()
            if (date.isEmpty()) {
                txtResult.text = "Please enter a valid date (YYYY-MM-DD)"
                return@setOnClickListener
            }

            val selectedId = rgForm.checkedRadioButtonId
            val radioButton = findViewById<RadioButton>(selectedId)
            val formType = radioButton.text.toString().split(" ")[0]

            txtResult.text = "Analyzing..."

            CoroutineScope(Dispatchers.IO).launch {
                try {
                    val result = api.analyze(date, formType)
                    withContext(Dispatchers.Main) {
                        txtResult.text = "Risk Level: ${result.risk_level}\nPenalty: ₹${result.penalty_estimate}\nSection: ${result.act_section}"
                        
                        if (result.risk_level == "CRITICAL") {
                            txtResult.setTextColor(android.graphics.Color.RED)
                        } else {
                            txtResult.setTextColor(android.graphics.Color.GREEN)
                        }
                    }
                } catch (e: Exception) {
                    withContext(Dispatchers.Main) {
                        txtResult.text = "Connection Error: Is Backend Running?\n${e.message}"
                        txtResult.setTextColor(android.graphics.Color.BLACK)
                    }
                }
            }
        }
    }
}
