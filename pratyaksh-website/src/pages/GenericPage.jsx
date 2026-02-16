import React from 'react';
import { ArrowRight } from 'lucide-react';
import { THEME, PAGE_CONTENT } from '../data/constants';
import * as Tools from '../components/Tools';
import * as Calculators from '../components/Calculators';

export const GenericPage = ({ pageKey, setPage }) => {
  const data = PAGE_CONTENT[pageKey];
  if (!data) return <div>Page Not Found</div>;

  const FeatureIcon = data.icon;

  const renderTool = () => {
    switch (data.toolType) {
      // --- ORIGINAL CORE TOOLS ---
      case 'mca': return <Tools.McaTool />;
      case 'board': return <Tools.BoardRiskTool />;
      case 'trust': return <Tools.TrustScoreTool />;
      case 'regulator': return <Tools.RegulatorTool />;

      // --- NEW 20 REAL-TIME CALCULATORS ---
      case 'msme_calc': return <Calculators.MsmeCalculator />;
      case 'rpt_monitor': return <Calculators.RptMonitor />;
      case 'director_risk': return <Calculators.DirectorRisk />;
      case 'csr_calc': return <Calculators.CsrCalculator />;
      case 'itc_reversal': return <Calculators.ItcReversal />;
      case 'tax_regime': return <Calculators.TaxRegimeAnalyzer />;
      case 'advance_tax': return <Calculators.AdvanceTaxEstimator />;
      case 'lease_calc': return <Calculators.LeaseCalculator />;
      case 'angel_tax': return <Calculators.AngelTaxValidator />;
      case 'buyback_tax': return <Calculators.BuybackTax />;
      case 'gratuity_calc': return <Calculators.GratuityCalc />;
      case 'pmla_scanner': return <Calculators.PmlaScanner />;
      case 'esg_checker': return <Calculators.EsgChecker />;
      case 'udin_valid': return <Calculators.UdinValidator />;
      case 'audit_rot': return <Calculators.AuditRotation />;
      case 'net_worth': return <Calculators.NetWorthCalc />;
      case 'shell_risk': return <Calculators.ShellRisk />;
      case 'export_track': return <Calculators.ExportTracker />;
      case 'partner_diss': return <Calculators.PartnershipCalc />;
      case 'crypto_tax': return <Calculators.CryptoTax />;
      
      default: return <div className="p-4 bg-red-100 text-red-800 rounded">Tool component not found for {data.toolType}</div>;
    }
  };

  return (
    <div className={`min-h-screen ${THEME.bg} pt-12 pb-20 px-6`}>
      <div className="max-w-4xl mx-auto">
        <button onClick={() => setPage('home')} className="mb-8 flex items-center gap-2 text-[#666666] hover:text-black group">
          <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back Home
        </button>
        
        <div className="border-l-4 border-[#4FF978] pl-8 mb-12">
          <h1 className="text-5xl md:text-6xl font-medium tracking-tighter mb-4 text-[#111111]">{data.title}</h1>
          <p className="text-xl text-[#666666] font-light">{data.subtitle}</p>
        </div>

        <div className="prose prose-lg max-w-none text-[#333333]">
          {data.type === 'text' && <p className="text-xl leading-relaxed">{data.content}</p>}
          
          {data.type === 'code' && (
            <div className="bg-[#111111] p-6 rounded-lg text-white font-mono text-sm overflow-x-auto">
              <code>{data.content}</code>
            </div>
          )}

          {data.type === 'list' && (
            <ul className="space-y-4">
              {data.items.map((item, i) => (
                <li key={i} className="flex items-center gap-3 p-4 bg-white border border-[#111111]/5 rounded-lg">
                  <div className="w-2 h-2 bg-[#4FF978] rounded-full"></div>
                  {item}
                </li>
              ))}
            </ul>
          )}

          {data.type === 'form' && (
            <form className="space-y-6 max-w-lg" onSubmit={(e) => e.preventDefault()}>
              <div>
                <label className="block text-sm font-bold mb-2">Email Address</label>
                <input type="email" className="w-full p-4 bg-white border border-gray-300 focus:border-[#111111] outline-none" placeholder="name@firm.com" />
              </div>
              <div>
                <label className="block text-sm font-bold mb-2">Message</label>
                <textarea className="w-full p-4 bg-white border border-gray-300 focus:border-[#111111] outline-none h-32" placeholder="How can we help?"></textarea>
              </div>
              <button className="px-8 py-4 bg-[#111111] text-white font-bold hover:bg-black/90">Send Message</button>
            </form>
          )}

          {data.type === 'feature' && FeatureIcon && (
            <div className="grid md:grid-cols-2 gap-8">
              <div className="bg-[#111111] text-white p-8 rounded-xl flex flex-col justify-between min-h-[300px]">
                <FeatureIcon size={48} className="text-[#4FF978] mb-6" />
                <div>
                  <h3 className="text-2xl font-bold mb-4">Core Capability</h3>
                  <p className="text-gray-400">This module is part of the Enterprise Suite. It connects directly to your local database.</p>
                </div>
                <button onClick={() => setPage('download')} className="mt-8 text-[#4FF978] hover:underline flex items-center gap-2">
                  Download Software <ArrowRight size={16}/>
                </button>
              </div>
              <div className="space-y-4">
                <div className="p-6 bg-white border border-gray-200 rounded-xl">
                  <h4 className="font-bold mb-2">Real-time Sync</h4>
                  <p className="text-sm text-gray-600">Changes made on the desktop app reflect instantly on mobile.</p>
                </div>
                <div className="p-6 bg-white border border-gray-200 rounded-xl">
                  <h4 className="font-bold mb-2">Offline Mode</h4>
                  <p className="text-sm text-gray-600">Access your data even without an internet connection.</p>
                </div>
              </div>
            </div>
          )}

          {/* RENDER REAL TOOLS */}
          {data.type === 'tool' && (
            <div className="mt-8 animate-in fade-in slide-in-from-bottom-8 duration-700">
              {renderTool()}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};