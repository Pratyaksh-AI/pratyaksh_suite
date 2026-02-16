import React, { useState } from 'react';
import { doc, setDoc, serverTimestamp } from 'firebase/firestore';
import { db, appId } from '../lib/firebase';
import { 
  CreditCard, Lock, Loader2, CheckCircle2, Copy, 
  Globe, QrCode, Smartphone, ShieldCheck, 
  Wallet, ArrowRightLeft, AlertTriangle, Landmark 
} from 'lucide-react';

export default function Payment({ user, plan, onPaymentComplete }) {
  const [isProcessing, setIsProcessing] = useState(false);
  const [email, setEmail] = useState("");
  const [error, setError] = useState("");
  const [selectedUpiApp, setSelectedUpiApp] = useState("PhonePe");

  const handleTransaction = async (e) => {
    e.preventDefault();
    if (!user) return;
    setIsProcessing(true);
    setError("");

    try {
      const txnId = `TXN-${Math.random().toString(36).substr(2, 9).toUpperCase()}`;
      
      // RULE 1: Strict Path Compliance
      const paymentRef = doc(db, 'artifacts', appId, 'public', 'data', 'payments', user.uid);

      // DATA STRUCTURE MATCHING RUST ADMIN MODEL STRICTLY
      await setDoc(paymentRef, {
        userId: user.uid,
        userEmail: email,
        amount: plan?.price || "₹19,999",
        plan: plan?.name || "Enterprise Suite",
        status: "pending",
        txnId: txnId,
        device: "Web Client", // Required by Rust model
        createdAt: serverTimestamp() // Audit trail (ignored by Rust struct serialization but good for DB)
      });

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
    alert("Copied: " + text);
  };

  // Logic to switch images based on selection
  // Ensure these files exist in public/assets/
  const getQrImage = (app) => {
    switch (app) {
        case "PhonePe": return "https://a-amm.vercel.app/assets/ph.png";
        case "GPay": return "https://a-amm.vercel.app/assets/gpay.png";
        case "BHIM": return "https://a-amm.vercel.app/assets/bhim.jpeg";
        case "WhatsApp Pay": return "https://a-amm.vercel.app/assets/wa.png";
        default: return "/assets/upi.png";
    }
  };

  return (
    <div className="max-w-6xl mx-auto mt-8 mb-20 animate-in fade-in slide-in-from-bottom-4 duration-700 font-sans text-[#111111]">
      
      <div className="grid lg:grid-cols-12 gap-12">
        
        {/* LEFT COLUMN: Payment Methods */}
        <div className="lg:col-span-7 space-y-8">
          
          {/* Section Header */}
          <div className="flex items-center gap-4 border-b border-black/10 pb-6">
            <div className="p-3 bg-[#111111] rounded-lg">
              <Wallet className="w-6 h-6 text-white" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-[#111111]">Payment Gateway</h2>
              <p className="text-sm text-gray-500">Select a secure method to transfer funds.</p>
            </div>
          </div>

          {/* 1. UPI Payment Card */}
          <div className="bg-white border border-gray-200 rounded-xl overflow-hidden shadow-sm hover:shadow-md transition-shadow">
            <div className="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-center">
              <div className="flex items-center gap-2">
                <Smartphone className="w-4 h-4 text-[#111111]" />
                <span className="font-bold text-sm tracking-wide text-[#111111]">UPI / QR CODE</span>
              </div>
              <span className="px-2 py-1 rounded bg-green-100 text-green-800 text-[10px] font-bold border border-green-200">INSTANT</span>
            </div>
            
            <div className="p-6 flex flex-col sm:flex-row gap-8">
               {/* Controls */}
               <div className="flex-1 space-y-6">
                  <div>
                    <div className="flex items-center gap-2 mb-3">
                        <span className="text-xs font-bold text-white bg-[#111111] px-2 py-1 rounded">IN</span>
                        <label className="text-xs text-gray-500 font-bold uppercase tracking-wider">Select App</label>
                    </div>
                    <div className="grid grid-cols-2 gap-3">
                      {['PhonePe', 'GPay', 'BHIM', 'WhatsApp Pay'].map(app => (
                        <button
                          key={app}
                          type="button"
                          onClick={() => setSelectedUpiApp(app)}
                          className={`px-4 py-3 rounded-lg text-xs font-bold transition-all border ${
                            selectedUpiApp === app 
                              ? 'bg-[#111111] text-white border-[#111111] shadow-lg' 
                              : 'bg-white text-gray-600 border-gray-200 hover:border-gray-400 hover:text-black'
                          }`}
                        >
                          {app}
                        </button>
                      ))}
                    </div>
                  </div>

                  <div>
                    <label className="text-xs text-gray-500 font-bold uppercase tracking-wider mb-2 block">VPA / UPI ID</label>
                    <div className="flex items-center gap-3 bg-gray-50 p-3 rounded-lg border border-gray-200 hover:border-gray-400 transition-colors">
                      <div className="p-2 bg-white rounded-md border border-gray-200 shadow-sm">
                        <ArrowRightLeft className="w-4 h-4 text-[#111111]" />
                      </div>
                      <code className="flex-1 text-[#111111] font-mono text-sm font-bold">918329004424@waicici</code>
                      <button onClick={() => copyToClipboard('918329004424@waicici')} className="p-2 hover:bg-gray-200 rounded-md text-gray-500 hover:text-black transition-colors">
                        <Copy size={16} />
                      </button>
                    </div>
                    <div className="text-[10px] text-gray-500 mt-2 ml-1">Beneficiary: Arun Ammisetty</div>
                  </div>
               </div>

               {/* QR Display */}
               <div className="flex flex-col items-center justify-center">
                  <div className="w-40 h-40 bg-white p-2 rounded-xl border border-gray-200 shadow-xl relative">
                    <img 
                      src={getQrImage(selectedUpiApp)} 
                      alt={`${selectedUpiApp} QR`}
                      className="w-full h-full object-contain"
                      onError={(e) => {e.target.src = "https://placehold.co/400x400/F3F2EC/111?text=QR+Code"}}
                    />
                    <div className="absolute -bottom-3 -right-3 bg-[#111111] text-white text-[10px] font-bold px-2 py-1 rounded shadow-lg">
                      SCAN ME
                    </div>
                  </div>
                  <span className="mt-4 text-xs text-gray-500 font-medium">Scanning via {selectedUpiApp}</span>
               </div>
            </div>
          </div>

          {/* 2. International Wire Card */}
          <div className="bg-white border border-gray-200 rounded-2xl overflow-hidden shadow-sm">
            <div className="p-5 border-b border-gray-100 bg-gray-50 flex items-center gap-2">
              <Globe className="w-4 h-4 text-[#111111]" />
              <span className="font-bold text-sm tracking-wide text-[#111111]">INTERNATIONAL WIRE</span>
            </div>
            
            <div className="p-6 grid gap-8">
               {/* UAE */}
               <div className="space-y-2">
                  <div className="flex items-center gap-2 text-[#111111] font-bold text-xs uppercase tracking-wider">
                    <Landmark className="w-3 h-3 text-gray-400" /> UAE (United Arab Emirates)
                  </div>
                  <div className="bg-gray-50 p-5 rounded-lg border border-gray-200 grid gap-2 text-xs">
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Bank</span> <span className="text-[#111111] font-bold">Standard Chartered</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Address</span> <span className="text-[#111111] text-right">Standard Chartered Tower, Emaar Square Dubai</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">IBAN</span> <span className="font-mono text-[#111111] select-all">AE550446420010001414704</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">SWIFT</span> <span className="font-mono text-[#111111] select-all">SCBLAEADXXX</span></div>
                     <div className="flex justify-between pt-1"><span className="text-gray-500">Beneficiary</span> <span className="text-[#111111] font-bold">Arun Ammisetty</span></div>
                  </div>
               </div>

               {/* Australia */}
               <div className="space-y-2">
                  <div className="flex items-center gap-2 text-[#111111] font-bold text-xs uppercase tracking-wider">
                    <Landmark className="w-3 h-3 text-gray-400" /> Australia
                  </div>
                  <div className="bg-gray-50 p-5 rounded-lg border border-gray-200 grid gap-2 text-xs">
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Bank</span> <span className="text-[#111111] font-bold">Citibank</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Address</span> <span className="text-[#111111] text-right">2 Park Street, Sydney NSW 2000</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">BSB</span> <span className="font-mono text-[#111111] select-all">248024</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Account</span> <span className="font-mono text-[#111111] select-all">10516966</span></div>
                     <div className="flex justify-between pt-1"><span className="text-gray-500">Beneficiary</span> <span className="text-[#111111] font-bold">Arun Ammisetty</span></div>
                  </div>
               </div>

               {/* Japan */}
               <div className="space-y-2">
                  <div className="flex items-center gap-2 text-[#111111] font-bold text-xs uppercase tracking-wider">
                    <Landmark className="w-3 h-3 text-gray-400" /> Japan
                  </div>
                  <div className="bg-gray-50 p-5 rounded-lg border border-gray-200 grid gap-2 text-xs">
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Bank</span> <span className="text-[#111111] font-bold">MUFG Bank, Ltd. (0005)</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Branch</span> <span className="font-mono text-[#111111]">869</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Account</span> <span className="font-mono text-[#111111] select-all">4674430 (Savings)</span></div>
                     <div className="flex justify-between pt-1"><span className="text-gray-500">Beneficiary</span> <span className="text-[#111111] font-bold">ﾍﾟｲｵﾆｱ ｼﾞﾔﾊﾟﾝ(ｶ</span></div>
                  </div>
               </div>

               {/* Eurozone */}
               <div className="space-y-2">
                  <div className="flex items-center gap-2 text-[#111111] font-bold text-xs uppercase tracking-wider">
                    <Landmark className="w-3 h-3 text-gray-400" /> Eurozone
                  </div>
                  <div className="bg-gray-50 p-5 rounded-lg border border-gray-200 grid gap-2 text-xs">
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Bank</span> <span className="text-[#111111] font-bold">Banking Circle S.A.</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">Address</span> <span className="text-[#111111] text-right">L-1528 LUXEMBOURG</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">IBAN</span> <span className="font-mono text-[#111111] select-all">LU744080000045726924</span></div>
                     <div className="flex justify-between border-b border-gray-200 pb-2"><span className="text-gray-500">BIC</span> <span className="font-mono text-[#111111] select-all">BCIRLULL</span></div>
                     <div className="flex justify-between pt-1"><span className="text-gray-500">Beneficiary</span> <span className="text-[#111111] font-bold">Arun Ammisetty</span></div>
                  </div>
               </div>

               {/* PayPal */}
               <div className="flex justify-end pt-4">
                 <div className="flex flex-col items-center gap-3 p-6 bg-white rounded-xl shadow-sm border border-gray-200">
                    <span className="text-[#003087] font-extrabold text-xs flex items-center gap-1"><Globe size={14}/> PayPal</span>
                    <div className="w-24 h-24 bg-gray-50 flex items-center justify-center overflow-hidden rounded-md border border-gray-200">
                        <img 
                          src="https://a-amm.vercel.app/assets/paypal.png" 
                          alt="PayPal" 
                          className="w-full h-full object-contain"
                          onError={(e) => {e.target.src = "https://placehold.co/100x100/003087/white?text=PP"}}
                        />
                    </div>
                    <span className="text-[10px] text-gray-500 font-medium">Global Support</span>
                 </div>
               </div>
            </div>
          </div>
        </div>

        {/* RIGHT COLUMN: Summary & Confirm */}
        <div className="lg:col-span-5">
           <div className="sticky top-24 space-y-6">
              
              {/* Order Summary */}
              <div className="bg-white border border-gray-200 rounded-2xl p-8 shadow-xl shadow-black/5">
                 <h3 className="text-[#111111] font-bold mb-6 flex items-center gap-2 pb-6 border-b border-gray-100">
                    <ShieldCheck className="w-5 h-5 text-black" /> Purchase Summary
                 </h3>
                 
                 <div className="space-y-4 mb-8">
                    <div className="flex justify-between items-center">
                       <span className="text-gray-500 text-sm">Plan Selected</span>
                       <span className="text-[#111111] font-bold">{plan?.name || "Enterprise Suite"}</span>
                    </div>
                    <div className="flex justify-between items-center">
                       <span className="text-gray-500 text-sm">Billing Cycle</span>
                       <span className="text-[#111111] font-medium">One-time / Lifetime</span>
                    </div>
                    <div className="flex justify-between items-center">
                       <span className="text-gray-500 text-sm">Platform Fee</span>
                       <span className="text-green-600 font-medium">Waived</span>
                    </div>
                 </div>

                 <div className="flex justify-between items-end mb-8 pt-6 border-t border-dashed border-gray-200">
                    <span className="text-gray-500 text-sm font-medium">Total Due</span>
                    <span className="text-4xl font-bold text-[#111111] tracking-tighter">{plan?.price || "₹19,999"}</span>
                 </div>

                 {/* Confirmation Form */}
                 <form onSubmit={handleTransaction} className="space-y-5">
                    <div className="space-y-2">
                        <label className="text-xs font-bold text-gray-500 uppercase tracking-wider">Registered Email</label>
                        <input 
                          type="email" 
                          required
                          value={email}
                          onChange={(e) => setEmail(e.target.value)}
                          placeholder="name@firm.com"
                          className="w-full bg-gray-50 border border-gray-200 text-black p-4 rounded-lg outline-none focus:border-black focus:ring-1 focus:ring-black transition-all text-sm placeholder:text-gray-400"
                        />
                    </div>

                    {error && (
                        <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-red-600 text-xs flex items-center gap-2 font-medium">
                          <AlertTriangle size={14} /> {error}
                        </div>
                    )}

                    <button 
                        type="submit" 
                        disabled={isProcessing}
                        className="w-full py-4 bg-[#111111] hover:bg-black text-white font-bold rounded-xl transition-all flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
                    >
                        {isProcessing ? (
                          <Loader2 className="w-5 h-5 animate-spin" />
                        ) : (
                          <>Confirm Transaction <CheckCircle2 className="w-5 h-5" /></>
                        )}
                    </button>
                    
                    <div className="flex items-center justify-center gap-2 text-[10px] text-gray-400 mt-4">
                       <Lock className="w-3 h-3" />
                       <span>256-bit SSL Secure Verification</span>
                    </div>
                 </form>
              </div>

           </div>
        </div>
      </div>
    </div>
  );
}