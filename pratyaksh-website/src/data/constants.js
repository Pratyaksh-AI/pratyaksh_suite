import { 
  MapPin, Users, Lock, Activity, 
  FileWarning, ShieldCheck, Book, Shield, 
  Calculator, Eye, Globe, CheckCircle, RefreshCw, BarChart, DollarSign, Bitcoin, Scale, Clock, Building2, AlertTriangle, Heart, 
  Activity as McaIcon
} from 'lucide-react';

export const THEME = {
  bg: "bg-[#F3F2EC]",
  textMain: "text-[#111111]",
  textMuted: "text-[#666666]",
  accent: "bg-[#4FF978]", 
  accentHover: "hover:bg-[#3DD665]",
  buttonText: "text-[#111111]",
  border: "border-[#111111]/10",
  darkBg: "bg-[#111111]",
  darkText: "text-white"
};

export const PAGE_CONTENT = {
  // --- CORE PAGES ---
  home: { title: "Home", type: "custom" },
  download: { title: "Download Center", type: "custom" },
  pricing: { title: "Pricing Plans", type: "custom" },
  
  // --- COMPANY ---
  about: { title: "About Us", subtitle: "Our mission to revolutionize compliance.", type: "text", content: "PratyakshAI was founded in 2024 with a singular mission: to bring predictive intelligence to the Indian regulatory landscape. We believe that Chartered Accountants and Company Secretaries deserve tools that are as proactive as they are." },
  careers: { title: "Careers", subtitle: "Join the intelligence revolution.", type: "text", content: "We are looking for Rust engineers, React developers, and Legal experts to join our Pune HQ. Help us build the operating system for the future of finance." },
  contact: { title: "Contact Us", subtitle: "Get in touch with our team.", type: "form" },
  legal: { title: "Legal Information", subtitle: "Transparency is our core value.", type: "text", content: "PratyakshAI complies with all IT Act 2000 regulations. We prioritize user data sovereignty and employ end-to-end encryption for all client data." },
  privacy: { title: "Privacy Policy", subtitle: "Your data, your control.", type: "text", content: "We do not sell your data. All analytics are performed locally on your device or via encrypted tunnels to our secure servers." },
  terms: { title: "Terms of Service", subtitle: "Usage guidelines.", type: "text", content: "By using PratyakshAI, you agree to our standard EULA. Enterprise licenses are subject to specific SLA agreements." },
  
  // --- MAIN MODULES ---
  city_risk: { title: "City Risk Engine", subtitle: "Predictive Notice Analysis", type: "feature", icon: MapPin },
  client_integrity: { title: "Client Integrity", subtitle: "Automated Due Diligence", type: "feature", icon: Users },
  evidence_locker: { title: "Evidence Locker", subtitle: "Cryptographic Audit Trails", type: "feature", icon: Lock },
  firm_ops: { title: "Firm Operations", subtitle: "Practice Management Intelligence", type: "feature", icon: Activity },
  
  // --- ORIGINAL CORE TOOLS (4) ---
  mca_predictor: { title: "MCA Filing Predictor", subtitle: "AI-based rejection probability engine.", type: "tool", toolType: "mca", icon: McaIcon },
  board_risk: { title: "Board Risk Simulator", subtitle: "Analyze resolutions for legal exposure.", type: "tool", toolType: "board", icon: FileWarning },
  trust_score: { title: "Trust Scoring", subtitle: "Client reliability index calculator.", type: "tool", toolType: "trust", icon: ShieldCheck },
  reg_notebook: { title: "Regulator Notebook", subtitle: "Officer intelligence database.", type: "tool", toolType: "regulator", icon: Book },

  // --- NEW REAL-TIME TOOLS (20) ---
  msme_calc: { title: "MSME 45-Day Calculator", subtitle: "Sec 43B(h) Interest Computer", type: "tool", toolType: "msme_calc", icon: Calculator },
  rpt_monitor: { title: "RPT Monitor", subtitle: "Related Party Transaction Thresholds", type: "tool", toolType: "rpt_monitor", icon: Users },
  director_risk: { title: "Director Risk Check", subtitle: "Sec 164 Disqualification", type: "tool", toolType: "director_risk", icon: Shield },
  csr_calc: { title: "CSR Obligation", subtitle: "2% Average Net Profit Logic", type: "tool", toolType: "csr_calc", icon: Heart },
  itc_reversal: { title: "ITC Reversal", subtitle: "GST Rule 42/43 Calculator", type: "tool", toolType: "itc_reversal", icon: RefreshCw },
  tax_regime: { title: "Tax Regime Analyzer", subtitle: "Old vs New Regime Break-even", type: "tool", toolType: "tax_regime", icon: Scale },
  advance_tax: { title: "Advance Tax Estimator", subtitle: "Quarterly Liability Calculator", type: "tool", toolType: "advance_tax", icon: Clock },
  lease_calc: { title: "Lease Liability", subtitle: "Ind AS 116 Amortization", type: "tool", toolType: "lease_calc", icon: Building2 },
  angel_tax: { title: "Angel Tax Validator", subtitle: "FMV vs Issue Price Check", type: "tool", toolType: "angel_tax", icon: AlertTriangle },
  buyback_tax: { title: "Buyback Tax", subtitle: "Share Buyback Liability", type: "tool", toolType: "buyback_tax", icon: DollarSign },
  gratuity_calc: { title: "Gratuity Calculator", subtitle: "Payment of Gratuity Act", type: "tool", toolType: "gratuity_calc", icon: Calculator },
  pmla_scanner: { title: "PMLA Scanner", subtitle: "Money Laundering Red Flags", type: "tool", toolType: "pmla_scanner", icon: Eye },
  esg_checker: { title: "ESG Applicability", subtitle: "BRSR Mandate Check", type: "tool", toolType: "esg_checker", icon: Globe },
  udin_valid: { title: "UDIN Validator", subtitle: "Format Verification", type: "tool", toolType: "udin_valid", icon: CheckCircle },
  audit_rot: { title: "Audit Rotation", subtitle: "Cooling Period Tracker", type: "tool", toolType: "audit_rot", icon: RefreshCw },
  net_worth: { title: "Net Worth Calculator", subtitle: "Sec 2(57) Companies Act", type: "tool", toolType: "net_worth", icon: BarChart },
  shell_risk: { title: "Shell Company Index", subtitle: "Turnover/Asset Ratio", type: "tool", toolType: "shell_risk", icon: AlertTriangle },
  export_track: { title: "Export Realization", subtitle: "FEMA Compliance Tracker", type: "tool", toolType: "export_track", icon: Globe },
  partner_diss: { title: "Partnership Dissolution", subtitle: "Settlement Calculator", type: "tool", toolType: "partner_diss", icon: Users },
  crypto_tax: { title: "Crypto Tax", subtitle: "VDA Tax Calculator", type: "tool", toolType: "crypto_tax", icon: Bitcoin },

  // --- SUB-TOOLS (Informational) ---
  penalty_forecast: { title: "Penalty Forecast", subtitle: "Liability Estimation", type: "text", content: "Real-time calculation of potential liabilities based on Companies Act, 2013 delays and specific officer strictness indices." },
  time_leakage: { title: "Time Leakage", subtitle: "Efficiency Analytics", type: "text", content: "Identify exactly which clients and tasks are consuming your non-billable hours with our passive desktop tracker." },
  billing_opt: { title: "Billing Optimizer", subtitle: "Market Rate Intelligence", type: "text", content: "AI suggestions for fee quotes based on the complexity of the specific compliance task and current city market rates." },
  case_law: { title: "Local Case Law", subtitle: "District-level Precedents", type: "text", content: "Instantly find relevant judgments from your specific district court or tribunal to support your replies." },
  
  // --- SOLUTIONS ---
  workflow_auto: { title: "Workflow Automation", subtitle: "End-to-end process handling", type: "text", content: "Connect your email, Tally, and MCA portal to automate the flow of documents and approvals." },
  custom_ai: { title: "Custom AI Models", subtitle: "Private LLMs for Firms", type: "text", content: "Train a private AI model on your firm's historical opinions and drafts to generate consistent, high-quality documents." },
  implementation: { title: "Implementation", subtitle: "Onboarding & Setup", type: "text", content: "Our white-glove service ensures your team is fully trained and your data is securely migrated within 48 hours." },
  strategy: { title: "Digital Strategy", subtitle: "Future-proofing your practice", type: "text", content: "Consulting services to help legacy firms transition to a digital-first, AI-augmented operating model." },
  
  // --- RESOURCES ---
  case_studies: { title: "Case Studies", subtitle: "Success Stories", type: "list", items: ["Kulkarni Associates: 40% less notices", "Mehta & Co: Scaling to 5 cities", "Rao Consultants: Automating due diligence"] },
  documentation: { title: "Documentation", subtitle: "User Guides & Manuals", type: "text", content: "Comprehensive guides for the Windows Desktop Client and Android Companion App." },
  api_ref: { title: "API Reference", subtitle: "Developer Docs", type: "code", content: "GET /api/v1/risk/score?city=pune" },
  status: { title: "System Status", subtitle: "Operational Uptime", type: "text", content: "All Systems Operational. API Latency: 45ms." },
  portfolio: { title: "Portfolio", subtitle: "Our Work", type: "text", content: "Explore how we have transformed over 500 firms across India." },
  experts: { title: "Our Experts", subtitle: "Meet the Team", type: "text", content: "Built by a team of CAs, CSs, and ex-Google Engineers." },
  news: { title: "Newsroom", subtitle: "Latest Updates", type: "list", items: ["v6.0 Released - Enterprise Edition", "New Mumbai Office Opening", "Partnership with ICAI Chapter"] },
};

export const PRICING_TIERS = [
  {
    name: "Launch",
    price: "₹1,199",
    period: "/ Month",
    desc: "For independent CAs starting their AI automation journey.",
    features: ["Android App Access", "City Risk Alerts (1 City)", "Basic Evidence Locker", "1 User License"],
    bg: "bg-[#F3F2EC]",
    btnBg: "bg-[#111111]",
    btnText: "text-white"
  },
  {
    name: "Scale",
    price: "₹2,399",
    period: "/ Month",
    desc: "For growing firms expanding across multiple wards and cities.",
    features: ["Windows Desktop Control", "All City Risk Data", "Unlimited Evidence Hashing", "Client Integrity Engine"],
    bg: "bg-[#111111]",
    textColor: "text-white",
    descColor: "text-gray-400",
    btnBg: "bg-[#4FF978]",
    btnText: "text-black",
    highlight: true
  },
  {
    name: "Enterprise",
    price: "₹4,999",
    period: "/ Month",
    desc: "For teams with 50+ clients requiring advanced automation.",
    features: ["Multi-City Dashboard", "API Access for ERP", "Dedicated Account Manager", "Custom Regulator Notes"],
    bg: "bg-[#F3F2EC]",
    btnBg: "bg-[#111111]",
    btnText: "text-white"
  }
];

// --- DATABASE CONFIGURATION ---
export const DB_PATHS = {
  // Base Firestore Path Structure
  // artifacts/{appId}/public/data/{collection}
  BASE: "artifacts",
  VISIBILITY: "public",
  DATA: "data",
  
  // Collections
  COLLECTION_PAYMENTS: "payments",       // Stores transaction requests
  COLLECTION_USER_ACCESS: "user_access"  // Stores approval status
};