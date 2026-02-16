import React from 'react';
import { Check, ArrowRight, X } from 'lucide-react';
import { THEME, PRICING_TIERS } from '../data/constants';

const ComparisonTable = () => {
  const features = [
    { name: "City Risk Analysis", basic: true, pro: true, ent: true },
    { name: "Evidence Locker", basic: "5GB", pro: "Unlimited", ent: "Unlimited" },
    { name: "Client Integrity Score", basic: false, pro: true, ent: true },
    { name: "MCA Predictor", basic: false, pro: true, ent: true },
    { name: "Billing Optimizer", basic: false, pro: true, ent: true },
    { name: "Multi-City Support", basic: false, pro: false, ent: true },
    { name: "API Access", basic: false, pro: false, ent: true },
    { name: "On-Premise Setup", basic: false, pro: false, ent: true },
  ];

  return (
    <div className="mt-20 overflow-x-auto">
      <table className="w-full text-left border-collapse">
        <thead>
          <tr className="border-b border-[#111111]/10">
            <th className="py-6 px-4 text-sm font-bold uppercase tracking-wider text-[#666666]">Feature</th>
            <th className="py-6 px-4 text-sm font-bold uppercase tracking-wider text-[#111111]">Launch</th>
            <th className="py-6 px-4 text-sm font-bold uppercase tracking-wider text-[#4FF978] bg-[#111111]">Scale</th>
            <th className="py-6 px-4 text-sm font-bold uppercase tracking-wider text-[#111111]">Enterprise</th>
          </tr>
        </thead>
        <tbody>
          {features.map((feat, i) => (
            <tr key={i} className="border-b border-[#111111]/5 hover:bg-white transition-colors">
              <td className="py-4 px-4 font-medium text-[#111111]">{feat.name}</td>
              <td className="py-4 px-4 text-[#666666]">{feat.basic === true ? <Check size={18} /> : feat.basic === false ? <X size={18} className="opacity-20"/> : feat.basic}</td>
              <td className="py-4 px-4 font-bold text-[#111111] bg-[#111111]/5">{feat.pro === true ? <Check size={18} /> : feat.pro === false ? <X size={18} className="opacity-20"/> : feat.pro}</td>
              <td className="py-4 px-4 text-[#111111]">{feat.ent === true ? <Check size={18} /> : feat.ent === false ? <X size={18} className="opacity-20"/> : feat.ent}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export const PricingPage = ({ setPage }) => (
  <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
    <div className="max-w-7xl mx-auto">
      <div className="text-center mb-24">
        <h1 className="text-6xl md:text-8xl font-medium tracking-tighter mb-8 text-[#111111]">
          Invest in foresight.
        </h1>
        <p className="text-xl text-[#666666] max-w-2xl mx-auto">
          Choose the plan that fits your firm's size. All plans include the core Android App and basic risk alerts.
        </p>
      </div>

      <div className="grid lg:grid-cols-3 gap-8">
        {PRICING_TIERS.map((tier, idx) => (
          <div key={idx} className={`p-10 flex flex-col ${tier.bg} ${tier.textColor || 'text-[#111111]'} ${tier.highlight ? 'relative overflow-hidden shadow-2xl scale-105 z-10' : 'border border-[#111111]/10'}`}>
            {tier.highlight && <div className="absolute top-6 right-6 bg-[#4FF978] text-black text-xs font-bold px-3 py-1 uppercase">Most Popular</div>}
            <h3 className="text-3xl font-medium mb-4">{tier.name}</h3>
            <p className={`mb-8 text-sm ${tier.descColor || 'text-[#666666]'}`}>{tier.desc}</p>
            <div className="mb-8"><span className="text-6xl font-medium tracking-tighter">{tier.price}</span><span className={`text-sm ${tier.descColor || 'text-[#666666]'}`}>{tier.period}</span></div>
            <ul className="space-y-4 mb-12 flex-1">
              {tier.features.map((feat, i) => (
                <li key={i} className="flex items-center gap-3 text-sm font-medium"><Check size={16} className={tier.highlight ? "text-[#4FF978]" : "text-[#111111]"} />{feat}</li>
              ))}
            </ul>
            {/* UPDATED: Route to 'payment' page instead of 'download' */}
            <button onClick={() => setPage('payment')} className={`w-full py-4 font-bold text-sm rounded-lg transition-colors ${tier.btnBg} ${tier.btnText} hover:opacity-90`}>Choose Plan</button>
          </div>
        ))}
      </div>

      <ComparisonTable />

      <div className="mt-24 p-12 bg-[#111111] text-white rounded-2xl flex flex-col md:flex-row justify-between items-center gap-8">
        <div>
            <h3 className="text-3xl font-bold mb-4">Need a custom enterprise setup?</h3>
            <p className="text-gray-400">We offer on-premise deployment, whitelabeling, and dedicated support engineering.</p>
        </div>
        <button onClick={() => setPage('contact')} className="px-8 py-4 bg-[#4FF978] text-black font-bold rounded-lg hover:bg-white transition-colors flex items-center gap-2">
            Contact Sales <ArrowRight size={18}/>
        </button>
      </div>
    </div>
  </div>
);