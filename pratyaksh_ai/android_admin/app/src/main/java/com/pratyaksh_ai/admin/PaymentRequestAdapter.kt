package com.pratyaksh_ai.admin

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.pratyaksh_ai.admin.model.PaymentRequest

class PaymentRequestAdapter(
    private val requests: List<PaymentRequest>,
    private val onApproveClick: (PaymentRequest) -> Unit,
    private val onDenyClick: (PaymentRequest) -> Unit
) : RecyclerView.Adapter<PaymentRequestAdapter.PaymentViewHolder>() {

    class PaymentViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        val tvUserName: TextView = view.findViewById(R.id.tvUserName)
        val tvTransactionId: TextView = view.findViewById(R.id.tvTransactionId)
        val tvPlanDetails: TextView = view.findViewById(R.id.tvPlanDetails)
        val btnApprove: Button = view.findViewById(R.id.btnApprove)
        val btnDeny: Button = view.findViewById(R.id.btnDeny)
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): PaymentViewHolder {
        val view = LayoutInflater.from(parent.context)
            .inflate(R.layout.item_payment_request, parent, false)
        return PaymentViewHolder(view)
    }

    override fun onBindViewHolder(holder: PaymentViewHolder, position: Int) {
        val request = requests[position]

        holder.tvUserName.text = request.userName
        holder.tvTransactionId.text = "Txn ID: ${request.transactionId}"
        holder.tvPlanDetails.text = "Plan: ${request.planName} | ₹${request.amount}"

        holder.btnApprove.setOnClickListener {
            onApproveClick(request)
        }

        holder.btnDeny.setOnClickListener {
            onDenyClick(request)
        }
    }

    override fun getItemCount() = requests.size
}