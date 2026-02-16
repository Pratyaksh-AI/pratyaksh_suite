package com.pratyaksh_ai.admin

import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.TextView

class PaymentRequestAdapter(
    context: Context,
    private val requests: List<Map<String, Any>>,
    private val onApprove: (String, String) -> Unit,
    private val onDeny: (String) -> Unit
) : ArrayAdapter<Map<String, Any>>(context, 0, requests) {

    override fun getView(position: Int, convertView: View?, parent: ViewGroup): View {
        val view = convertView ?: LayoutInflater.from(context).inflate(R.layout.item_payment_request, parent, false)
        val data = getItem(position)!!

        val txtEmail = view.findViewById<TextView>(R.id.txtEmail)
        val txtPlan = view.findViewById<TextView>(R.id.txtPlan)
        val btnApprove = view.findViewById<Button>(R.id.btnApprove)
        val btnDeny = view.findViewById<Button>(R.id.btnDeny)

        txtEmail.text = data["userEmail"] as? String ?: "Unknown"
        txtPlan.text = "${data["plan"]} • ${data["amount"]}"

        val docId = data["id"] as String
        val userId = data["userId"] as String

        btnApprove.setOnClickListener { onApprove(docId, userId) }
        btnDeny.setOnClickListener { onDeny(docId) }

        return view
    }
}