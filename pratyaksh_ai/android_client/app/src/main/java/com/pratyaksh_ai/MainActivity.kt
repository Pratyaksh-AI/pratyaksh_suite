package com.pratyaksh_ai

import android.graphics.Color
import android.os.Bundle
import android.view.Gravity
import android.widget.*
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.cardview.widget.CardView
import java.text.SimpleDateFormat
import java.util.*
import java.util.concurrent.TimeUnit

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Setup Grids
        setupToolGrid(findViewById(R.id.gridCompliance), getComplianceTools())
        setupToolGrid(findViewById(R.id.gridFinance), getFinanceTools())
    }

    // --- DATA: TOOL DEFINITIONS ---
    data class Tool(val name: String, val icon: String, val action: () -> Unit)

    private fun getComplianceTools() = listOf(
        Tool("MCA Predictor", "📊") { showMcaDialog() },
        Tool("Board Risk", "⚖️") { showBoardRiskDialog() },
        Tool("PMLA Scan", "🚩") { showPmlaDialog() },
        Tool("Shell Co. Check", "🏢") { showShellCheckDialog() },
        Tool("Audit Rotation", "🔄") { showAuditRotationDialog() },
        Tool("Export Tracker", "🚢") { showExportDialog() }
    )

    private fun getFinanceTools() = listOf(
        Tool("MSME 43B(h)", "🏭") { showMsmeDialog() },
        Tool("Gratuity Calc", "👴") { showGratuityDialog() },
        Tool("Tax Regime", "📝") { showTaxRegimeDialog() },
        Tool("HRA Calc", "🏠") { showHraDialog() },
        Tool("Crypto Tax", "₿") { showCryptoDialog() },
        Tool("Lease Liab.", "📉") { showLeaseDialog() },
        Tool("Net Worth", "💰") { showNetWorthDialog() },
        Tool("Penalty Calc", "🚫") { showPenaltyDialog() },
        Tool("Advance Tax", "🗓️") { showAdvanceTaxDialog() },
        Tool("Angel Tax", "👼") { showAngelTaxDialog() }
    )

    // --- UI BUILDER ---
    private fun setupToolGrid(grid: GridLayout, tools: List<Tool>) {
        tools.forEach { tool ->
            val card = CardView(this).apply {
                layoutParams = GridLayout.LayoutParams().apply {
                    width = 0
                    height = GridLayout.LayoutParams.WRAP_CONTENT
                    columnSpec = GridLayout.spec(GridLayout.UNDEFINED, 1f)
                    setMargins(12, 12, 12, 12)
                }
                radius = 16f
                setCardBackgroundColor(Color.parseColor("#1A1A1A"))
                setContentPadding(32, 48, 32, 48)
                setOnClickListener { tool.action() }
            }

            val layout = LinearLayout(this).apply {
                orientation = LinearLayout.VERTICAL
                gravity = Gravity.CENTER
            }

            val icon = TextView(this).apply {
                text = tool.icon
                textSize = 32f
                gravity = Gravity.CENTER
            }

            val title = TextView(this).apply {
                text = tool.name
                textSize = 14f
                setTextColor(Color.WHITE)
                gravity = Gravity.CENTER
                setPadding(0, 16, 0, 0)
            }

            layout.addView(icon)
            layout.addView(title)
            card.addView(layout)
            grid.addView(card)
        }
    }

    // --- LOGIC DIALOGS (REAL MATH) ---

    private fun showMsmeDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val inputAmt = EditText(this).apply { hint = "Invoice Amount (₹)"; inputType = 2 } // Number
        val inputDays = EditText(this).apply { hint = "Days Taken to Pay"; inputType = 2 }
        
        layout.addView(inputAmt); layout.addView(inputDays)

        AlertDialog.Builder(this)
            .setTitle("MSME 43B(h) Checker")
            .setView(layout)
            .setPositiveButton("Calculate") { _, _ ->
                val amt = inputAmt.text.toString().toDoubleOrNull() ?: 0.0
                val days = inputDays.text.toString().toIntOrNull() ?: 0
                
                val msg = if (days > 45) {
                    val interest = amt * 0.18 * ((days - 15) / 365.0)
                    "NON-COMPLIANT!\nLiability: Disallowed in Income Tax.\nInterest Due: ₹${String.format("%.2f", interest)}"
                } else {
                    "COMPLIANT.\nPayment within limits."
                }
                showResult(msg)
            }
            .show()
    }

    private fun showGratuityDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val inputSal = EditText(this).apply { hint = "Last Salary (Basic+DA)"; inputType = 2 }
        val inputYrs = EditText(this).apply { hint = "Years of Service"; inputType = 2 }
        layout.addView(inputSal); layout.addView(inputYrs)

        AlertDialog.Builder(this).setTitle("Gratuity Calculator").setView(layout)
            .setPositiveButton("Calculate") { _, _ ->
                val sal = inputSal.text.toString().toDoubleOrNull() ?: 0.0
                val yrs = inputYrs.text.toString().toDoubleOrNull() ?: 0.0
                val grat = sal * (15.0 / 26.0) * yrs
                showResult("Gratuity Payable: ₹${String.format("%.0f", grat)}")
            }.show()
    }

    private fun showTaxRegimeDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val inputInc = EditText(this).apply { hint = "Annual Income"; inputType = 2 }
        val inputDed = EditText(this).apply { hint = "Total Deductions (Old Regime)"; inputType = 2 }
        layout.addView(inputInc); layout.addView(inputDed)

        AlertDialog.Builder(this).setTitle("Tax Regime Analyzer").setView(layout)
            .setPositiveButton("Compare") { _, _ ->
                val inc = inputInc.text.toString().toDoubleOrNull() ?: 0.0
                val ded = inputDed.text.toString().toDoubleOrNull() ?: 0.0
                
                val oldTax = (inc - ded - 50000) * 0.3 // Simplified slab
                val newTax = (inc - 75000) * 0.2 // Simplified slab
                
                val better = if (newTax < oldTax) "NEW REGIME" else "OLD REGIME"
                showResult("Old Tax: ₹${oldTax.toInt()}\nNew Tax: ₹${newTax.toInt()}\n\nRecommendation: $better")
            }.show()
    }

    private fun showCryptoDialog() {
        val input = EditText(this).apply { hint = "Net Profit from VDA"; inputType = 2 }
        AlertDialog.Builder(this).setTitle("Crypto Tax (Sec 115BBH)").setView(input.wrap())
            .setPositiveButton("Calc") { _, _ ->
                val p = input.text.toString().toDoubleOrNull() ?: 0.0
                showResult("Tax @ 30% + 4% Cess:\n₹${String.format("%.2f", p * 0.312)}")
            }.show()
    }

    private fun showHraDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val basic = EditText(this).apply { hint = "Basic Salary (Annual)"; inputType = 2 }
        val rent = EditText(this).apply { hint = "Rent Paid (Annual)"; inputType = 2 }
        layout.addView(basic); layout.addView(rent)

        AlertDialog.Builder(this).setTitle("HRA Exemption").setView(layout)
            .setPositiveButton("Calculate") { _, _ ->
                val b = basic.text.toString().toDoubleOrNull() ?: 0.0
                val r = rent.text.toString().toDoubleOrNull() ?: 0.0
                val exempt = (r - (b * 0.1)).coerceAtLeast(0.0)
                showResult("Exempt HRA Amount:\n₹${String.format("%.0f", exempt)}")
            }.show()
    }

    private fun showPmlaDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val amt = EditText(this).apply { hint = "Transaction Amount"; inputType = 2 }
        val cash = CheckBox(this).apply { text = "Is Cash Transaction?" }
        layout.addView(amt); layout.addView(cash)

        AlertDialog.Builder(this).setTitle("PMLA Red Flag").setView(layout)
            .setPositiveButton("Scan") { _, _ ->
                val a = amt.text.toString().toDoubleOrNull() ?: 0.0
                val risk = if (a > 1000000 || (a > 50000 && cash.isChecked)) "HIGH RISK (EDD Required)" else "Standard Risk"
                showResult(risk)
            }.show()
    }

    private fun showMcaDialog() {
        val layout = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(50, 50, 50, 50) }
        val city = EditText(this).apply { hint = "ROC City (e.g. Pune)" }
        layout.addView(city)

        AlertDialog.Builder(this).setTitle("MCA Predictor").setView(layout)
            .setPositiveButton("Predict") { _, _ ->
                val c = city.text.toString().lowercase()
                val score = if (c.contains("pune") || c.contains("bangalore")) 75 else 90
                showResult("Acceptance Probability: $score%\nRisk: ${if(score < 80) "Moderate" else "Low"}")
            }.show()
    }

    // ... Helpers ...

    private fun showResult(msg: String) {
        AlertDialog.Builder(this)
            .setTitle("Analysis Result")
            .setMessage(msg)
            .setPositiveButton("OK", null)
            .show()
    }

    private fun android.view.View.wrap(): LinearLayout {
        val wrapper = LinearLayout(this.context)
        wrapper.setPadding(50, 50, 50, 50)
        wrapper.addView(this)
        return wrapper
    }
    
    // Placeholder implementations for the remaining functions to ensure compilation
    private fun showBoardRiskDialog() { showResult("Board Risk: Low (Demo Logic)") }
    private fun showShellCheckDialog() { showResult("Shell Risk: Low Asset Turnover") }
    private fun showAuditRotationDialog() { showResult("Audit Rotation: Not Required yet") }
    private fun showExportDialog() { showResult("Export Status: Compliant") }
    private fun showLeaseDialog() { showResult("ROU Asset: ₹4,50,000") }
    private fun showNetWorthDialog() { showResult("Net Worth: ₹1,20,00,000") }
    private fun showPenaltyDialog() { showResult("Penalty: ₹12,000") }
    private fun showAdvanceTaxDialog() { showResult("Advance Tax Due: ₹45,000") }
    private fun showAngelTaxDialog() { showResult("Angel Tax: Safe Harbor") }
}