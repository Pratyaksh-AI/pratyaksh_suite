cat <<EOF > app/src/main/java/com/pratyaksh_ai/MainActivity.kt
package com.pratyaksh_ai

import android.content.Context
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import android.widget.RadioButton
import android.widget.RadioGroup
import android.widget.ImageButton
import android.app.AlertDialog
import android.widget.Toast
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

    private var currentApiUrl = "http://10.0.2.2:8080/" // Default for Emulator

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // 1. Load Saved URL (Hassle-Free Persistence)
        val prefs = getSharedPreferences("PratyakshPrefs", Context.MODE_PRIVATE)
        currentApiUrl = prefs.getString("backend_url", "http://10.0.2.2:8080/") ?: "http://10.0.2.2:8080/"

        val etDate = findViewById<EditText>(R.id.etFyDate)
        val rgForm = findViewById<RadioGroup>(R.id.rgFormType)
        val btnCheck = findViewById<Button>(R.id.btnAnalyze)
        val btnSettings = findViewById<ImageButton>(R.id.btnSettings) // We will add this button to XML
        
        val cardResult = findViewById<CardView>(R.id.cardResult)
        val txtRisk = findViewById<TextView>(R.id.txtRiskLevel)
        val txtPenalty = findViewById<TextView>(R.id.txtPenalty)
        val txtSection = findViewById<TextView>(R.id.txtSection)

        // 2. SETTINGS BUTTON LOGIC
        btnSettings.setOnClickListener {
            showUrlDialog()
        }

        // 3. ANALYZE BUTTON LOGIC
        btnCheck.setOnClickListener {
            val date = etDate.text.toString()
            if (date.isEmpty()) {
                etDate.error = "Required"
                return@setOnClickListener
            }

            // Ensure URL ends with slash
            if (!currentApiUrl.endsWith("/")) currentApiUrl += "/"

            val selectedId = rgForm.checkedRadioButtonId
            val radioButton = findViewById<RadioButton>(selectedId)
            val formType = radioButton.text.toString()

            btnCheck.text = "CONNECTING..."
            btnCheck.isEnabled = false

            // Rebuild Retrofit with current URL
            val retrofit = Retrofit.Builder()
                .baseUrl(currentApiUrl)
                .addConverterFactory(GsonConverterFactory.create())
                .build()
            val api = retrofit.create(PratyakshApi::class.java)

            CoroutineScope(Dispatchers.IO).launch {
                try {
                    val result = api.analyze(date, formType)
                    
                    withContext(Dispatchers.Main) {
                        cardResult.visibility = View.VISIBLE
                        btnCheck.text = "ANALYZE RISK"
                        btnCheck.isEnabled = true
                        
                        txtRisk.text = result.risk_level
                        txtPenalty.text = "Penalty Estimate: ₹\${result.penalty_estimate}"
                        txtSection.text = "Violation under \${result.act_section}"

                        if (result.risk_level == "CRITICAL") {
                            txtRisk.setTextColor(ContextCompat.getColor(this@MainActivity, R.color.risk_critical))
                        } else {
                            txtRisk.setTextColor(ContextCompat.getColor(this@MainActivity, R.color.risk_safe))
                        }
                    }
                } catch (e: Exception) {
                    withContext(Dispatchers.Main) {
                        btnCheck.text = "RETRY"
                        btnCheck.isEnabled = true
                        Toast.makeText(this@MainActivity, "Connection Failed. Check Settings.", Toast.LENGTH_LONG).show()
                        
                        txtRisk.text = "NETWORK ERROR"
                        txtRisk.setTextColor(ContextCompat.getColor(this@MainActivity, R.color.text_primary))
                        txtPenalty.text = "Could not reach server"
                        txtSection.text = e.message
                        cardResult.visibility = View.VISIBLE
                    }
                }
            }
        }
    }

    private fun showUrlDialog() {
        val input = EditText(this)
        input.setText(currentApiUrl)
        input.hint = "https://your-backend-url.com/"

        AlertDialog.Builder(this)
            .setTitle("Server Configuration")
            .setMessage("Enter the URL of your PratyakshAI Backend:")
            .setView(input)
            .setPositiveButton("Save") { _, _ ->
                var newUrl = input.text.toString().trim()
                if (newUrl.isNotEmpty()) {
                    currentApiUrl = newUrl
                    // Save it permanently
                    getSharedPreferences("PratyakshPrefs", Context.MODE_PRIVATE)
                        .edit()
                        .putString("backend_url", currentApiUrl)
                        .apply()
                    Toast.makeText(this, "URL Saved", Toast.LENGTH_SHORT).show()
                }
            }
            .setNegativeButton("Cancel", null)
            .show()
    }
}
EOF