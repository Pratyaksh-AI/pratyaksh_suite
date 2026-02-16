import React from 'react';
import { ArrowRight, MapPin, Users, Lock, Activity } from 'lucide-react';
import { THEME } from '../data/constants';

export const ModulesPage = ({ setPage }) => {
  const modules = [
    { id: 'city_risk', title: 'City Risk Engine', icon: MapPin, color: 'bg-red-100', text: 'text-red-800' },
    { id: 'client_integrity', title: 'Client Integrity', icon: Users, color: 'bg-blue-100', text: 'text-blue-800' },
    { id: 'evidence_locker', title: 'Evidence Locker', icon: Lock, color: 'bg-green-100', text: 'text-green-800' },
    { id: 'firm_ops', title: 'Firm Operations', icon: Activity, color: 'bg-purple-100', text: 'text-purple-800' },
  ];

  return (
    <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
      <div className="max-w-7xl mx-auto">
        <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
          <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back Home
        </button>
        <h1 className="text-6xl font-medium tracking-tighter mb-16 text-[#111111]">Platform Modules</h1>

        <div className="grid gap-12">
           {modules.map((mod, i) => (
             <div key={i} className="group bg-white border border-[#111111]/10 p-12 rounded-2xl hover:shadow-xl transition-all cursor-pointer" onClick={() => setPage(mod.id)}>
                <div className="flex flex-col md:flex-row gap-12 items-start">
                   <div className={`p-6 rounded-2xl ${mod.color} ${mod.text}`}>
                      <mod.icon size={48} />
                   </div>
                   <div className="flex-1">
                      <h2 className="text-4xl font-bold mb-6">{mod.title}</h2>
                      <p className="text-xl text-[#666666] mb-8 leading-relaxed">
                         Access deep intelligence specific to this domain. Click to launch the dedicated toolset and explore specific calculators.
                      </p>
                      <span className="inline-flex items-center gap-2 font-bold underline underline-offset-4 group-hover:text-[#4FF978] transition-colors">
                        Launch Module <ArrowRight size={20} />
                      </span>
                   </div>
                </div>
             </div>
           ))}
        </div>
      </div>
    </div>
  );
};