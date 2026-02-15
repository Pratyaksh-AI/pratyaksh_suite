import { MapPin, Users, Lock, Activity, Activity as McaIcon, FileWarning, ShieldCheck, Book } from 'lucide-react';

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
  home: { title: "Home", type: "custom" },
  download: { title: "Download Center", type: "custom" },
  pricing: { title: "Pricing Plans", type: "custom" },
  
  about: { title: "About Us", subtitle: "Our mission to revolutionize compliance.", type: "text", content: "PratyakshAI was founded in 2024 with a singular mission: to bring predictive intelligence to the Indian regulatory landscape. We believe that Chartered Accountants and Company Secretaries deserve tools that are as proactive as they are." },
  careers: { title: "Careers", subtitle: "Join the intelligence revolution.", type: "text", content: "We are looking for Rust engineers, React developers, and Legal experts to join our Pune HQ. Help us build the operating system for the future of finance." },
  contact: { title: "Contact Us", subtitle: "Get in touch with our team.", type: "form" },
  legal: { title: "Legal Information", subtitle: "Transparency is our core value.", type: "text", content: "PratyakshAI complies with all IT Act 2000 regulations. We prioritize user data sovereignty and employ end-to-end encryption for all client data." },
  privacy: { title: "Privacy Policy", subtitle: "Your data, your control.", type: "text", content: "We do not sell your data. All analytics are performed locally on your device or via encrypted tunnels to our secure servers." },
  terms: { title: "Terms of Service", subtitle: "Usage guidelines.", type: "text", content: "By using PratyakshAI, you agree to our standard EULA. Enterprise licenses are subject to specific SLA agreements." },
  
  city_risk: { title: "City Risk Engine", subtitle: "Predictive Notice Analysis", type: "feature", icon: MapPin },
  client_integrity: { title: "Client Integrity", subtitle: "Automated Due Diligence", type: "feature", icon: Users },
  evidence_locker: { title: "Evidence Locker", subtitle: "Cryptographic Audit Trails", type: "feature", icon: Lock },
  firm_ops: { title: "Firm Operations", subtitle: "Practice Management Intelligence", type: "feature", icon: Activity },
  
  mca_predictor: { title: "MCA Filing Predictor", subtitle: "AI-based rejection probability engine.", type: "tool", toolType: "mca", icon: McaIcon },
  board_risk: { title: "Board Risk Simulator", subtitle: "Analyze resolutions for legal exposure.", type: "tool", toolType: "board", icon: FileWarning },
  trust_score: { title: "Trust Scoring", subtitle: "Client reliability index calculator.", type: "tool", toolType: "trust", icon: ShieldCheck },
  reg_notebook: { title: "Regulator Notebook", subtitle: "Officer intelligence database.", type: "tool", toolType: "regulator", icon: Book },

  penalty_forecast: { title: "Penalty Forecast", subtitle: "Liability Estimation", type: "text", content: "Real-time calculation of potential liabilities based on Companies Act, 2013 delays and specific officer strictness indices." },
  time_leakage: { title: "Time Leakage", subtitle: "Efficiency Analytics", type: "text", content: "Identify exactly which clients and tasks are consuming your non-billable hours with our passive desktop tracker." },
  billing_opt: { title: "Billing Optimizer", subtitle: "Market Rate Intelligence", type: "text", content: "AI suggestions for fee quotes based on the complexity of the specific compliance task and current city market rates." },
  case_law: { title: "Local Case Law", subtitle: "District-level Precedents", type: "text", content: "Instantly find relevant judgments from your specific district court or tribunal to support your replies." },
  
  workflow_auto: { title: "Workflow Automation", subtitle: "End-to-end process handling", type: "text", content: "Connect your email, Tally, and MCA portal to automate the flow of documents and approvals." },
  custom_ai: { title: "Custom AI Models", subtitle: "Private LLMs for Firms", type: "text", content: "Train a private AI model on your firm's historical opinions and drafts to generate consistent, high-quality documents." },
  implementation: { title: "Implementation", subtitle: "Onboarding & Setup", type: "text", content: "Our white-glove service ensures your team is fully trained and your data is securely migrated within 48 hours." },
  strategy: { title: "Digital Strategy", subtitle: "Future-proofing your practice", type: "text", content: "Consulting services to help legacy firms transition to a digital-first, AI-augmented operating model." },
  
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