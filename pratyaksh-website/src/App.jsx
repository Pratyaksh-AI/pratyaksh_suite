import React, { useState, useEffect, createContext, useContext } from 'react';
import { Navbar } from './components/Navbar';
import { Footer } from './components/Footer';

// Page Components
import { Home } from './pages/Home';
import { DownloadScreen } from './pages/Download';
import { ToolsPage } from './pages/Tools';
import { PricingPage } from './pages/Pricing';
import { AboutPage } from './pages/About';
import { ContactPage } from './pages/Contact';
import { ModulesPage } from './pages/Modules';
import { ResourcePage } from './pages/Resources';
import { GenericPage } from './pages/GenericPage';

// New Payment & Logic Components
import PaymentPage from './pages/Payment';
import ApprovalWait from './pages/ApprovalWait';

import { THEME } from './data/constants';

// Firebase Imports
import { initializeApp } from 'firebase/app';
import { getAuth, signInAnonymously, onAuthStateChanged, signInWithCustomToken } from 'firebase/auth';
import { getFirestore, doc, onSnapshot } from 'firebase/firestore';

// --- CONTEXT & FIREBASE SETUP ---
// NOTE: In a real deployment, these would be env variables. 
// For this environment, we assume the config is injected or standard.
const firebaseConfig = typeof __firebase_config !== 'undefined' ? JSON.parse(__firebase_config) : {}; 
const app = initializeApp(firebaseConfig);
const auth = getAuth(app);
const db = getFirestore(app);
const appId = typeof __app_id !== 'undefined' ? __app_id : 'pratyaksh_ai_suite';

export const AuthContext = createContext();

export default function App() {
  const [page, setPage] = useState('home');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  // Auth & Access State
  const [user, setUser] = useState(null);
  const [access, setAccess] = useState(null);
  const [loading, setLoading] = useState(true);

  // 1. Initialize Auth
  useEffect(() => {
    const initAuth = async () => {
      try {
        if (typeof __initial_auth_token !== 'undefined' && __initial_auth_token) {
          await signInWithCustomToken(auth, __initial_auth_token);
        } else {
          await signInAnonymously(auth);
        }
      } catch (err) {
        console.error("Auth failed", err);
      }
    };
    initAuth();

    const unsub = onAuthStateChanged(auth, u => {
      setUser(u);
      if (!u) setLoading(false);
    });
    return () => unsub();
  }, []);

  // 2. Listen for Admin Approval (Real-Time)
  useEffect(() => {
    if (!user) return;
    
    // Strict Path: artifacts/{appId}/public/data/user_access/{uid}
    const ref = doc(db, 'artifacts', appId, 'public', 'data', 'user_access', user.uid);
    
    const unsub = onSnapshot(ref, (snap) => {
      if (snap.exists()) {
        const data = snap.data();
        setAccess(data);
        
        // Auto-redirect if approved while waiting
        if (data.canDownload && page === 'wait') {
          setPage('download');
        }
      } else {
        setAccess({ canDownload: false });
      }
      setLoading(false);
    });
    return () => unsub();
  }, [user, page]);

  useEffect(() => { window.scrollTo(0, 0); }, [page]);

  // Router Logic to select the correct component based on state
  const renderPage = () => {
    switch(page) {
      // Core Pages
      case 'home': return <Home setPage={setPage} />;
      
      // Protected Download Flow
      case 'payment': return <PaymentPage user={user} onPaymentComplete={() => setPage('wait')} />;
      case 'wait': return <ApprovalWait user={user} onApproved={() => setPage('download')} />;
      case 'download': return <DownloadScreen setPage={setPage} />; // AccessGuard inside component handles restriction
      
      case 'tools': return <ToolsPage setPage={setPage} />;
      
      // Feature-Rich Custom Layouts
      case 'pricing': return <PricingPage setPage={setPage} />;
      case 'about': return <AboutPage setPage={setPage} />;
      case 'contact': return <ContactPage setPage={setPage} />;
      
      // Module Overview
      case 'city_risk':
      case 'firm_ops':
      case 'client_integrity':
      case 'evidence_locker':
         return <ModulesPage setPage={setPage} />; 
      
      // Resource Pages (Text-heavy docs)
      case 'legal':
      case 'privacy':
      case 'terms':
      case 'documentation':
      case 'api_ref':
      case 'status':
      case 'news':
      case 'careers':
      case 'case_studies':
         return <ResourcePage pageKey={page} setPage={setPage} />;

      // Fallback to GenericPage for Tools & Calculators
      default: return <GenericPage pageKey={page} setPage={setPage} />;
    }
  };

  return (
    <AuthContext.Provider value={{ user, access, setPage }}>
      <div className={`font-sans antialiased ${THEME.bg} ${THEME.textMain} selection:bg-[#4FF978] selection:text-black`}>
        <Navbar setPage={setPage} mobileMenuOpen={mobileMenuOpen} setMobileMenuOpen={setMobileMenuOpen} />
        
        <main>
          {renderPage()}
        </main>

        <Footer setPage={setPage} />
      </div>
    </AuthContext.Provider>
  );
}