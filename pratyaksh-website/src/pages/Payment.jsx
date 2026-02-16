import React, { useState } from 'react';
import { doc, setDoc, serverTimestamp } from 'firebase/firestore';
import { db, appId } from '../lib/firebase';
import { CreditCard, Lock, Loader2, CheckCircle2 } from 'lucide-react';

export default function Payment({ user, plan, onPaymentComplete }) {
  const [isProcessing, setIsProcessing] = useState(false);
  const [email, setEmail] = useState("");
  const [error, setError] = useState("");

  const handleTransaction = async (e) => {
    e.preventDefault();
    if (!user) return;
    setIsProcessing(true);
    setError("");

    try {
      // 1. Generate Transaction ID
      const txnId = `TXN-${Math.random().toString(36).substr(2, 9).toUpperCase()}`;

      // 2. Write to 'payments' collection (Admin APK Listens Here)
      // RULE 1: Strict Path Compliance
      const paymentRef = doc(db, 'artifacts', appId, 'public', 'data', 'payments', user.uid);

      await setDoc(paymentRef, {
        userId: user.uid,
        userEmail: email,
        amount: plan?.price || "₹19,999", // Dynamic based on selected plan
        plan: plan?.name || "Enterprise Suite",
        status: "pending", // Critical: This status triggers the Admin Notification
        txnId: txnId,
        createdAt: serverTimestamp(),
        device: "Web Client"
      });

      // 3. Handover control to the "Wait" page
      onPaymentComplete();

    } catch (err) {
      console.error("Transaction Write Failed:", err);
      setError("Secure connection failed. Please retry.");
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="max-w-md mx-auto mt-12 animate-in fade-in slide-in-from-bottom-8 duration-500">
      
      {/* Secure Header */}
      <div className="bg-[#1A1A1A] border border-white/10 rounded-t-2xl p-6 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-[#4FF978]/10 rounded-lg">
            <CreditCard className="w-5 h-5 text-[#4FF978]" />
          </div>
          <div>
            <h3 className="font-bold text-white">Secure Checkout</h3>
            <p className="text-xs text-gray-500 font-mono">SSL ENCRYPTED</p>
          </div>
        </div>
        <Lock className="w-4 h-4 text-gray-600" />
      </div>

      {/* Payment Form */}
      <div className="bg-[#111111] border-x border-b border-white/10 rounded-b-2xl p-8">
        <form onSubmit={handleTransaction} className="space-y-6">
          
          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-400">Order Summary</label>
            <div className="flex justify-between items-center p-4 bg-black rounded-xl border border-white/5">
              <span className="text-white font-bold">{plan?.name || "Enterprise Suite"}</span>
              <span className="text-[#4FF978] font-mono">{plan?.price || "₹19,999"}</span>
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-400">Billing Email</label>
            <input 
              type="email" 
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="name@firm.com"
              className="w-full bg-black border border-white/10 text-white p-4 rounded-xl outline-none focus:border-[#4FF978] transition-all"
            />
          </div>

          {error && (
            <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-red-500 text-sm">
              {error}
            </div>
          )}

          <button 
            type="submit" 
            disabled={isProcessing}
            className="w-full py-4 bg-[#4FF978] hover:bg-[#3DD665] text-black font-bold rounded-xl transition-all flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isProcessing ? (
              <Loader2 className="w-5 h-5 animate-spin" />
            ) : (
              <>Complete Transaction <CheckCircle2 className="w-5 h-5" /></>
            )}
          </button>

          <p className="text-xs text-gray-600 text-center leading-relaxed">
            By proceeding, a payment verification request will be sent to the PratyakshAI Admin Console. Access is granted instantly upon verification.
          </p>
        </form>
      </div>
    </div>
  );
}