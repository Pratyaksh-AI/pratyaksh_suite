import React, { useState } from 'react';
import { doc, setDoc, serverTimestamp } from 'firebase/firestore';
import { db, appId } from '../lib/firebase';
import { CreditCard, Lock, Loader2, CheckCircle2, Copy, Globe, QrCode } from 'lucide-react';

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

  const copyToClipboard = (text) => {
    navigator.clipboard.writeText(text);
    alert("Copied to clipboard: " + text);
  };

  return (
    <div className="max-w-xl mx-auto mt-12 animate-in fade-in slide-in-from-bottom-8 duration-500 mb-20">
      
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
        <form onSubmit={handleTransaction} className="space-y-8">
          
          {/* 1. Order Details */}
          <div className="space-y-2">
            <label className="text-sm font-medium text-gray-400">Order Summary</label>
            <div className="flex justify-between items-center p-4 bg-black rounded-xl border border-white/5">
              <span className="text-white font-bold">{plan?.name || "Enterprise Suite"}</span>
              <span className="text-[#4FF978] font-mono">{plan?.price || "₹19,999"}</span>
            </div>
          </div>

          {/* 2. Payment Instructions Section */}
          <div className="space-y-4 border-t border-white/10 pt-4">
            <h4 className="text-white font-bold flex items-center gap-2">
              <Globe className="w-4 h-4 text-[#4FF978]" /> Payment Details
            </h4>
            <p className="text-xs text-gray-500">Please complete the transfer using one of the methods below before confirming.</p>

            {/* UPI Section (India) */}
            <div className="bg-[#1A1A1A] p-5 rounded-xl border border-white/5">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <span className="text-xs font-bold text-[#4FF978] bg-[#4FF978]/10 px-2 py-1 rounded">🇮🇳 INDIA (UPI)</span>
                  <div className="mt-2 flex items-center gap-2">
                     <code className="text-white text-lg">918329004424@waicici</code>
                     <button type="button" onClick={() => copyToClipboard('918329004424@waicici')} className="text-gray-400 hover:text-white"><Copy size={14}/></button>
                  </div>
                  <div className="text-xs text-gray-500 mt-1">Beneficiary: Arun Ammisetty</div>
                </div>
                {/* QR Placeholder */}
                <div className="w-16 h-16 bg-white p-1 rounded flex items-center justify-center">
                    <QrCode className="text-black w-full h-full" />
                </div>
              </div>
              <div className="flex gap-2 mt-2">
                {['PhonePe', 'GPay', 'BHIM', 'WhatsApp Pay'].map(app => (
                    <div key={app} className="px-2 py-1 bg-white/10 rounded text-[10px] text-gray-300">{app}</div>
                ))}
              </div>
            </div>

            {/* International Bank Transfer Section */}
            <div className="space-y-3">
               <div className="text-xs font-bold text-blue-400 bg-blue-400/10 px-2 py-1 rounded w-fit">🌍 INTERNATIONAL WIRE</div>
               
               {/* UAE */}
               <div className="bg-[#1A1A1A] p-4 rounded-xl border border-white/5 text-sm space-y-1">
                 <div className="font-bold text-white mb-2 flex items-center gap-2">🇦🇪 UAE <span className="text-xs font-normal text-gray-500">(United Arab Emirates)</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Bank:</span> <span className="col-span-2 text-white">Standard Chartered</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Address:</span> <span className="col-span-2">Standard Chartered Tower, Emaar Square Dubai, UAE</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">IBAN:</span> <span className="col-span-2 text-mono text-white">AE550446420010001414704</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">SWIFT:</span> <span className="col-span-2 text-mono text-white">SCBLAEADXXX</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Name:</span> <span className="col-span-2 text-white">Arun Ammisetty</span></div>
               </div>

               {/* Australia */}
               <div className="bg-[#1A1A1A] p-4 rounded-xl border border-white/5 text-sm space-y-1">
                 <div className="font-bold text-white mb-2 flex items-center gap-2">🇦🇺 Australia</div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Bank:</span> <span className="col-span-2 text-white">Citibank</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Address:</span> <span className="col-span-2">2 Park Street, Sydney NSW 2000</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">BSB:</span> <span className="col-span-2 text-mono text-white">248024</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Account:</span> <span className="col-span-2 text-mono text-white">10516966</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Name:</span> <span className="col-span-2 text-white">Arun Ammisetty</span></div>
               </div>

               {/* Japan */}
               <div className="bg-[#1A1A1A] p-4 rounded-xl border border-white/5 text-sm space-y-1">
                 <div className="font-bold text-white mb-2 flex items-center gap-2">🇯🇵 Japan</div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Bank:</span> <span className="col-span-2 text-white">MUFG Bank, Ltd. (0005)</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Branch:</span> <span className="col-span-2">869</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Account:</span> <span className="col-span-2 text-mono text-white">4674430 (Savings)</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Name:</span> <span className="col-span-2 text-white">ﾍﾟｲｵﾆｱ ｼﾞﾔﾊﾟﾝ(ｶ</span></div>
               </div>

               {/* Eurozone */}
               <div className="bg-[#1A1A1A] p-4 rounded-xl border border-white/5 text-sm space-y-1">
                 <div className="font-bold text-white mb-2 flex items-center gap-2">🇪🇺 Eurozone</div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Bank:</span> <span className="col-span-2 text-white">Banking Circle S.A.</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Address:</span> <span className="col-span-2">2, Boulevard de la Foire L-1528 LUXEMBOURG</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">IBAN:</span> <span className="col-span-2 text-mono text-white">LU744080000045726924</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">BIC:</span> <span className="col-span-2 text-mono text-white">BCIRLULL</span></div>
                 <div className="grid grid-cols-3 gap-2 text-gray-400"><span className="text-gray-600">Name:</span> <span className="col-span-2 text-white">Arun Ammisetty</span></div>
               </div>
               
               <div className="flex justify-end pt-2">
                 <div className="flex items-center gap-2 px-3 py-1 bg-white rounded">
                    <span className="text-black font-bold text-xs">PayPal</span>
                    <QrCode className="w-4 h-4 text-black"/>
                 </div>
               </div>
            </div>
          </div>

          {/* 3. User Details & Confirm */}
          <div className="space-y-4 pt-4 border-t border-white/10">
            <div className="space-y-2">
                <label className="text-sm font-medium text-gray-400">Your Registered Email</label>
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
                <>I Have Completed Payment <CheckCircle2 className="w-5 h-5" /></>
                )}
            </button>

            <p className="text-xs text-gray-600 text-center leading-relaxed">
                By clicking above, a verification request will be sent to the PratyakshAI Admin Console. Access is granted instantly upon manual admin approval.
            </p>
          </div>
        </form>
      </div>
    </div>
  );
}