import React, { useEffect, useState } from 'react';
import { doc, getDoc } from 'firebase/firestore';
import { db, appId } from '../lib/firebase';
import { Lock, AlertCircle, RefreshCw } from 'lucide-react';

export default function AccessGuard({ user, children, onDeny }) {
  const [hasAccess, setHasAccess] = useState(null); // null = loading, false = denied, true = granted

  useEffect(() => {
    if (!user) {
      setHasAccess(false);
      return;
    }

    const checkAccess = async () => {
      try {
        // RULE 2: Direct Document Fetch (No complex query)
        // Path must strictly match: artifacts/pratyaksh_ai_suite/public/data/user_access/{uid}
        const docRef = doc(db, 'artifacts', appId, 'public', 'data', 'user_access', user.uid);
        const docSnap = await getDoc(docRef);

        if (docSnap.exists() && docSnap.data().canDownload === true) {
          setHasAccess(true);
        } else {
          setHasAccess(false);
        }
      } catch (err) {
        console.error("Guard Check Failed:", err);
        setHasAccess(false);
      }
    };

    checkAccess();
  }, [user]);

  if (hasAccess === null) {
    return (
      <div className="min-h-[50vh] flex items-center justify-center">
        <RefreshCw className="w-8 h-8 text-[#4FF978] animate-spin" />
      </div>
    );
  }

  if (hasAccess === false) {
    return (
      <div className="flex flex-col items-center justify-center py-24 px-6 text-center animate-in zoom-in-95 duration-300">
        <div className="w-20 h-20 bg-red-500/10 rounded-full flex items-center justify-center mb-6 border border-red-500/20">
          <Lock className="w-10 h-10 text-red-500" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-4">Restricted Area</h2>
        <p className="text-gray-400 max-w-md mb-8">
          You do not have the required cryptographic clearance to view this page. 
          Your payment may still be pending approval or was denied.
        </p>
        <div className="flex gap-4">
          <button 
            onClick={() => window.location.reload()}
            className="px-6 py-3 bg-[#111111] border border-white/10 text-white rounded-lg hover:bg-white/5 transition-colors"
          >
            Check Again
          </button>
          <button 
            onClick={onDeny} // Callback to return to pricing
            className="px-6 py-3 bg-[#4FF978] text-black font-bold rounded-lg hover:bg-[#3DD665] transition-colors"
          >
            Return to Pricing
          </button>
        </div>
      </div>
    );
  }

  // If access is true, render the protected content
  return <>{children}</>;
}