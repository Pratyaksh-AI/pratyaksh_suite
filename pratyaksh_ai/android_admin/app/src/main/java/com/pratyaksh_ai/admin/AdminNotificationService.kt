package com.pratyaksh_ai.admin

import android.util.Log
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage

class AdminNotificationService : FirebaseMessagingService() {
    override fun onNewToken(token: String) {
        Log.d("AdminFCM", "New Admin Token: $token")
        // In production, save this token to Firestore under 'admins' collection
    }

    override fun onMessageReceived(remoteMessage: RemoteMessage) {
        // Logic to show notification when app is in foreground
        Log.d("AdminFCM", "Payment Notification Received: ${remoteMessage.data}")
    }
}