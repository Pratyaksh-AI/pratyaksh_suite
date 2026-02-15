package com.pratyaksh_ai.admin

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout
import com.pratyaksh_ai.admin.model.PaymentRequest
import com.pratyaksh_ai.admin.network.ApiClient
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response

class AdminMainActivity : AppCompatActivity() {

    private lateinit var recyclerView: RecyclerView
    private lateinit var swipeRefreshLayout: SwipeRefreshLayout
    private lateinit var adapter: PaymentRequestAdapter
    private val paymentRequests = mutableListOf<PaymentRequest>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_admin_main)

        // Initialize Views
        recyclerView = findViewById(R.id.recyclerViewPayments)
        swipeRefreshLayout = findViewById(R.id.swipeRefresh)

        // Setup RecyclerView
        recyclerView.layoutManager = LinearLayoutManager(this)
        adapter = PaymentRequestAdapter(paymentRequests, this::onApproveClicked, this::onDenyClicked)
        recyclerView.adapter = adapter

        // Load Data initially
        fetchPendingPayments()

        // Setup Pull-to-Refresh
        swipeRefreshLayout.setOnRefreshListener {
            fetchPendingPayments()
        }
    }

    private fun fetchPendingPayments() {
        swipeRefreshLayout.isRefreshing = true
        
        ApiClient.instance.getPendingPayments().enqueue(object : Callback<List<PaymentRequest>> {
            override fun onResponse(call: Call<List<PaymentRequest>>, response: Response<List<PaymentRequest>>) {
                swipeRefreshLayout.isRefreshing = false
                if (response.isSuccessful && response.body() != null) {
                    paymentRequests.clear()
                    paymentRequests.addAll(response.body()!!)
                    adapter.notifyDataSetChanged()
                } else {
                    Toast.makeText(this@AdminMainActivity, "Failed to load data", Toast.LENGTH_SHORT).show()
                }
            }

            override fun onFailure(call: Call<List<PaymentRequest>>, t: Throwable) {
                swipeRefreshLayout.isRefreshing = false
                Toast.makeText(this@AdminMainActivity, "Error: ${t.message}", Toast.LENGTH_SHORT).show()
            }
        })
    }

    private fun onApproveClicked(request: PaymentRequest) {
        updatePaymentStatus(request.userId, "approved")
    }

    private fun onDenyClicked(request: PaymentRequest) {
        updatePaymentStatus(request.userId, "denied")
    }

    private fun updatePaymentStatus(userId: String, status: String) {
        // Optimistic update: Remove from list immediately for better UI feel
        val position = paymentRequests.indexOfFirst { it.userId == userId }
        if (position != -1) {
            paymentRequests.removeAt(position)
            adapter.notifyItemRemoved(position)
        }

        // Call API to update status in Backend
        ApiClient.instance.updatePaymentStatus(userId, status).enqueue(object : Callback<Void> {
            override fun onResponse(call: Call<Void>, response: Response<Void>) {
                if (response.isSuccessful) {
                    val message = if (status == "approved") "User Approved" else "User Denied"
                    Toast.makeText(this@AdminMainActivity, message, Toast.LENGTH_SHORT).show()
                } else {
                    // Revert list if API fails (optional, simply showing error here)
                    Toast.makeText(this@AdminMainActivity, "Failed to update status", Toast.LENGTH_SHORT).show()
                    fetchPendingPayments() // Reload list to sync with server
                }
            }

            override fun onFailure(call: Call<Void>, t: Throwable) {
                Toast.makeText(this@AdminMainActivity, "Network Error", Toast.LENGTH_SHORT).show()
                fetchPendingPayments() // Reload list
            }
        })
    }
}