import React, { useEffect, useState } from 'react';
import { doc, onSnapshot } from 'firebase/firestore';
import { db, appId } from '../lib/firebase'; // Assuming centralized config
import { Loader2, ShieldCheck, Lock, Clock } from 'lucide-react';

export default function ApprovalWait({ user, onApproved }) {
  const [status, setStatus] = useState("waiting"); // waiting, approved, denied

  useEffect(() => {
    if (!user) return;

    // RULE 1: Strict Path - Listening to the exact document the Admin APK updates
    const accessRef = doc(db, 'artifacts', appId, 'public', 'data', 'user_access', user.uid);

    // Real-time listener
    const unsubscribe = onSnapshot(accessRef, (docSnap) => {
      if (docSnap.exists()) {
        const data = docSnap.data();
        
        if (data.canDownload === true) {
          setStatus("approved");
          // Add a small delay for UI transition before redirecting
          setTimeout(() => {
            onApproved(); 
          }, 1500);
        } else if (data.status === "denied") {
          setStatus("denied");
        }
      }
    }, (error) => {
      console.error("Approval listener error:", error);
    });

    return () => unsubscribe();
  }, [user]);

  if (status === "approved") {
    return (
      <div className="flex flex-col items-center justify-center h-[60vh] animate-in fade-in zoom-in duration-500">
        <div className="w-24 h-24 bg-green-500/10 rounded-full flex items-center justify-center mb-6">
          <ShieldCheck className="w-12 h-12 text-green-500" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">Access Granted</h2>
        <p className="text-gray-400">Redirecting to Secure Download Center...</p>
      </div>
    );
  }

  return (
    <div className="max-w-xl mx-auto py-20 px-6 text-center animate-in slide-in-from-bottom-4 duration-700">
      <div className="relative inline-block mb-10">
        <div className="absolute inset-0 bg-[#4FF978]/20 blur-3xl rounded-full animate-pulse"></div>
        <Clock className="w-20 h-20 text-[#4FF978] relative" />
      </div>

      <h1 className="text-4xl font-bold text-white mb-6">Verification Pending</h1>
      
      <div className="bg-[#111111] border border-white/10 rounded-2xl p-8 mb-8 text-left space-y-4">
        <div className="flex items-start gap-4">
          <div className="p-2 bg-blue-500/10 rounded-lg">
            <Lock className="w-5 h-5 text-blue-400" />
          </div>
          <div>
            <h4 className="text-white font-bold">Secure Hold</h4>
            <p className="text-sm text-gray-400">Your payment is being verified by the Super Admin via the secure Android terminal.</p>
          </div>
        </div>
        
        <div className="flex items-start gap-4">
          <div className="p-2 bg-yellow-500/10 rounded-lg">
            <Loader2 className="w-5 h-5 text-yellow-400 animate-spin" />
          </div>
          <div>
            <h4 className="text-white font-bold">Awaiting Signal</h4>
            <p className="text-sm text-gray-400">Do not close this window. It will automatically refresh once approved.</p>
          </div>
        </div>
      </div>

      <p className="text-xs text-gray-600 font-mono">
        SESSION ID: {user?.uid || "UNKNOWN"} <br/>
        LISTENING ON CHANNEL: artifacts/{appId}/user_access
      </p>
    </div>
  );
}