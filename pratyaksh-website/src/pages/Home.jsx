import React from 'react';
import { ArrowRight, ArrowUpRight, Check } from 'lucide-react';
import { THEME, PRICING_TIERS } from '../data/constants';

const Hero = ({ setPage }) => (
  <section className={`${THEME.bg} border-b ${THEME.border} overflow-hidden`}>
    <div className="grid lg:grid-cols-2 min-h-[80vh]">
      <div className={`p-8 md:p-16 flex flex-col justify-center border-r ${THEME.border}`}>
        <div className="inline-flex items-center gap-2 mb-8 bg-[#111111] text-white px-3 py-1 text-xs font-mono uppercase w-fit rounded-full">
          <span>New v6.0 Release</span>
          <ArrowRight size={12} />
        </div>
        <h1 className="text-6xl sm:text-7xl md:text-8xl font-medium tracking-tighter leading-[0.9] mb-8 text-[#111111]">
          Designing <br/> intelligent <br/> compliance.
        </h1>
        <p className="text-xl text-[#666666] max-w-md mb-12 leading-relaxed">
          PratyakshAI builds bespoke risk engines tailored to your city, clients, and practice goals.
        </p>
        <div className="flex gap-4">
          <button onClick={() => setPage('download')} className="px-8 py-4 bg-[#111111] text-white font-medium rounded-lg hover:bg-black/80 transition-all flex items-center gap-2 group">
            Start Download <ArrowUpRight size={18} className="group-hover:translate-x-1 transition-transform" />
          </button>
        </div>
      </div>
      <div className="relative bg-[#111111] flex items-center justify-center overflow-hidden p-8">
        {/* Abstract UI Mockup */}
        <div className="w-full max-w-md bg-[#F3F2EC] rounded-xl shadow-2xl overflow-hidden relative">
           <div className="bg-[#111111] p-4 flex justify-between items-center border-b border-white/10">
              <div className="flex gap-2"><div className="w-3 h-3 rounded-full bg-red-500"></div><div className="w-3 h-3 rounded-full bg-yellow-500"></div></div>
              <div className="text-[#4FF978] font-mono text-xs">CONNECTED: PUNE_WARD_05</div>
           </div>
           <div className="p-6 space-y-4">
              <div className="flex justify-between items-end">
                <div><div className="text-xs text-gray-500 uppercase tracking-wide">Risk Score</div><div className="text-4xl font-bold text-red-600">72%</div></div>
                <div className="text-xs bg-red-100 text-red-800 px-2 py-1 rounded">HIGH ALERT</div>
              </div>
              <div className="h-32 bg-gray-100 rounded border border-gray-200 p-2 flex items-end gap-1">
                 {[30, 50, 40, 70, 80, 60, 90, 75, 65, 85].map((h, i) => (
                   <div key={i} className="flex-1 bg-[#111111]" style={{height: `${h}%`, opacity: 0.2 + (i*0.05)}}></div>
                 ))}
              </div>
              <div className="grid grid-cols-2 gap-2">
                 <div className="bg-white p-3 rounded border border-gray-200">
                    <div className="text-xs text-gray-500">Penalty Est.</div>
                    <div className="font-mono font-bold">₹1,24,000</div>
                 </div>
                 <div className="bg-white p-3 rounded border border-gray-200">
                    <div className="text-xs text-gray-500">AO Strictness</div>
                    <div className="font-mono font-bold">9.2/10</div>
                 </div>
              </div>
           </div>
        </div>
      </div>
    </div>
  </section>
);

const StatsBar = () => (
  <section className={`${THEME.bg} border-b ${THEME.border}`}>
    <div className="grid grid-cols-2 md:grid-cols-4">
      {[{ label: "Client Satisfaction", val: "4.9/5" }, { label: "Successful Predictions", val: "50+" }, { label: "Reduction in Notices", val: "40%" }, { label: "AI Modules", val: "12+" }]
      .map((stat, idx) => (
        <div key={idx} className={`p-10 border-r ${THEME.border} hover:bg-white transition-colors`}>
          <div className="text-5xl font-medium tracking-tighter mb-2 text-[#111111]">{stat.val}</div>
          <div className="text-sm text-[#666666]">{stat.label}</div>
        </div>
      ))}
    </div>
  </section>
);

const ServicesGrid = ({ setPage }) => {
  const SERVICES = [
    { num: "01", title: "Risk Engine", desc: "Predict notices before they happen.", link: "city_risk", bg: "bg-[#F3F2EC]", text: "text-[#111111]" },
    { num: "02", title: "Integrity", desc: "Trust scores for every client.", link: "client_integrity", bg: "bg-[#4FF978]", text: "text-[#111111]" },
    { num: "03", title: "Evidence", desc: "Immutable cryptographic audit trails.", link: "evidence_locker", bg: "bg-[#4353FF]", text: "text-white" },
    { num: "04", title: "Ops Growth", desc: "Optimize billing and stop leakage.", link: "firm_ops", bg: "bg-[#111111]", text: "text-[#F3F2EC]" }
  ];
  return (
    <section className="grid lg:grid-cols-4 min-h-[600px] border-b border-[#111111]/10">
      {SERVICES.map((item, idx) => (
        <div key={idx} onClick={() => setPage(item.link)} className={`${item.bg} p-10 flex flex-col justify-between group border-r border-[#111111]/10 cursor-pointer hover:opacity-95 transition-opacity`}>
          <div>
            <div className={`text-6xl font-medium mb-6 opacity-30 ${item.text}`}>{item.num}</div>
            <h3 className={`text-3xl font-medium tracking-tight mb-4 ${item.text}`}>{item.title}</h3>
          </div>
          <div>
            <p className={`text-lg leading-relaxed mb-12 opacity-80 ${item.text}`}>{item.desc}</p>
            <div className={`w-12 h-12 rounded-full border border-current flex items-center justify-center ${item.text} group-hover:rotate-45 transition-transform`}><ArrowUpRight /></div>
          </div>
        </div>
      ))}
    </section>
  );
};

const Pricing = ({ setPage }) => (
  <section className={`${THEME.bg} py-32 px-6 border-b ${THEME.border}`}>
    <div className="max-w-[1400px] mx-auto">
      <div className="flex flex-col md:flex-row justify-between items-end mb-20 gap-8">
        <h2 className="text-6xl sm:text-7xl font-medium tracking-tighter leading-none text-[#111111]">The right plan to <br/> power your progress.</h2>
      </div>
      <div className="grid lg:grid-cols-3 gap-8">
        {PRICING_TIERS.map((tier, idx) => (
          <div key={idx} className={`p-10 flex flex-col ${tier.bg} ${tier.textColor || 'text-[#111111]'} ${tier.highlight ? 'relative overflow-hidden' : 'border border-[#111111]/10'}`}>
            {tier.highlight && <div className="absolute top-6 right-6 bg-[#4FF978] text-black text-xs font-bold px-3 py-1 uppercase">Popular</div>}
            <h3 className="text-2xl font-medium mb-4">{tier.name}</h3>
            <p className={`mb-8 text-sm ${tier.descColor || 'text-[#666666]'}`}>{tier.desc}</p>
            <div className="mb-8"><span className="text-6xl font-medium tracking-tighter">{tier.price}</span><span className={`text-sm ${tier.descColor || 'text-[#666666]'}`}>{tier.period}</span></div>
            <ul className="space-y-4 mb-12 flex-1">
              {tier.features.map((feat, i) => (
                <li key={i} className="flex items-center gap-3 text-sm font-medium"><Check size={16} className={tier.highlight ? "text-[#4FF978]" : "text-[#111111]"} />{feat}</li>
              ))}
            </ul>
            <button onClick={() => setPage('download')} className={`w-full py-4 font-bold text-sm rounded-lg transition-colors ${tier.btnBg} ${tier.btnText} hover:opacity-90`}>Choose Plan</button>
          </div>
        ))}
      </div>
    </div>
  </section>
);

export const Home = ({ setPage }) => (
  <main>
    <Hero setPage={setPage} />
    <StatsBar />
    <ServicesGrid setPage={setPage} />
    <Pricing setPage={setPage} />
  </main>
);