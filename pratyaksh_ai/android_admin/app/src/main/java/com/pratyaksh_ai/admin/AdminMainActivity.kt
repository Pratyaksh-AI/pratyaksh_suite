package com.pratyaksh_ai.admin

import android.os.Bundle
import android.widget.ListView
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.google.firebase.FirebaseApp
import com.google.firebase.auth.FirebaseAuth
import com.google.firebase.firestore.FirebaseFirestore
import com.google.firebase.firestore.ListenerRegistration
import com.google.firebase.firestore.SetOptions

class AdminMainActivity : AppCompatActivity() {

    private lateinit var db: FirebaseFirestore
    private lateinit var auth: FirebaseAuth
    private var paymentListener: ListenerRegistration? = null
    
    // Config
    private val appId = "pratyaksh_ai_suite" // Must match user app
    private val paymentRequests = mutableListOf<Map<String, Any>>()
    
    // UI
    private lateinit var listView: ListView
    private lateinit var statusText: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_admin_main)

        listView = findViewById(R.id.listView)
        statusText = findViewById(R.id.statusText)

        FirebaseApp.initializeApp(this)
        auth = FirebaseAuth.getInstance()
        db = FirebaseFirestore.getInstance()

        authenticateAdmin()
    }

    private fun authenticateAdmin() {
        // In real world, use Email/Password. For MVP, Anon + RLS checks
        auth.signInAnonymously().addOnSuccessListener {
            statusText.text = "🟢 Live. Watching for transactions..."
            listenForPayments()
        }.addOnFailureListener {
            statusText.text = "🔴 Connection Failed: ${it.message}"
        }
    }

    private fun listenForPayments() {
        // Strict Path: artifacts/{appId}/public/data/payments
        val path = "artifacts/$appId/public/data/payments"
        
        paymentListener = db.collection(path)
            .addSnapshotListener { snapshot, e ->
                if (e != null) {
                    statusText.text = "Sync Error: ${e.message}"
                    return@addSnapshotListener
                }

                if (snapshot != null) {
                    paymentRequests.clear()
                    for (doc in snapshot.documents) {
                        val data = doc.data?.toMutableMap() ?: mutableMapOf()
                        data["id"] = doc.id
                        // Only show pending
                        if (data["status"] == "pending") {
                            paymentRequests.add(data)
                        }
                    }
                    refreshList()
                }
            }
    }

    private fun refreshList() {
        val adapter = PaymentRequestAdapter(this, paymentRequests, 
            onApprove = { docId, userId -> approveUser(docId, userId) },
            onDeny = { docId -> denyUser(docId) }
        )
        listView.adapter = adapter
    }

    private fun approveUser(docId: String, userId: String) {
        val batch = db.batch()
        
        // 1. Update Payment Status
        val paymentRef = db.collection("artifacts/$appId/public/data/payments").document(docId)
        batch.update(paymentRef, "status", "approved")
        
        // 2. Create User Access Record (This unlocks the download page for the user)
        val accessRef = db.collection("artifacts/$appId/public/data/user_access").document(userId)
        val accessData = hashMapOf(
            "canDownload" to true,
            "plan" to "Enterprise",
            "grantedAt" to System.currentTimeMillis()
        )
        batch.set(accessRef, accessData, SetOptions.merge())

        batch.commit().addOnSuccessListener {
            Toast.makeText(this, "Access Granted", Toast.LENGTH_SHORT).show()
        }.addOnFailureListener {
            Toast.makeText(this, "Failed: ${it.message}", Toast.LENGTH_SHORT).show()
        }
    }

    private fun denyUser(docId: String) {
        db.collection("artifacts/$appId/public/data/payments").document(docId)
            .update("status", "denied")
            .addOnSuccessListener {
                Toast.makeText(this, "Request Denied", Toast.LENGTH_SHORT).show()
            }
    }

    override fun onDestroy() {
        paymentListener?.remove()
        super.onDestroy()
    }
}