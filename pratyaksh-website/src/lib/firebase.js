import { initializeApp } from 'firebase/app';
import { getAuth } from 'firebase/auth';
import { getFirestore } from 'firebase/firestore';

// --- FIREBASE CONFIGURATION ---
// Configuration for 'Pratyaksh Website' project
const firebaseConfig = {
  apiKey: "AIzaSyAyU8Xw9ZqRts_xsEtJISNvcHSq4kg0Wko",
  authDomain: "pratyakshai-website.firebaseapp.com",
  projectId: "pratyakshai-website",
  storageBucket: "pratyakshai-website.firebasestorage.app",
  messagingSenderId: "801069212447",
  appId: "1:801069212447:web:c8a7f60632744d04ba462a"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);

// Export Services for use in the app
export const auth = getAuth(app);
export const db = getFirestore(app);

// Export App ID for consistent path usage
// Uses the injected __app_id if available, otherwise defaults to the suite ID
export const appId = typeof __app_id !== 'undefined' ? __app_id : 'pratyaksh_ai_suite';

export default app;