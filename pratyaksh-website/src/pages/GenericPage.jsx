import React from 'react';
import { ArrowRight, ArrowUpRight } from 'lucide-react';
import { THEME, PAGE_CONTENT } from '../data/constants';
import { McaTool, BoardRiskTool, TrustScoreTool, RegulatorTool } from '../components/Tools';

export const GenericPage = ({ pageKey, setPage }) => {
  const data = PAGE_CONTENT[pageKey];
  if (!data) return <div>Page Not Found</div>;

  const FeatureIcon = data.icon;

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
            <div className="bg-[#111111] p-6 rounded-lg text-white font-mono text-sm overflow-x-auto"><code>{data.content}</code></div>
          )}

          {data.type === 'list' && (
            <ul className="space-y-4">
              {data.items.map((item, i) => (
                <li key={i} className="flex items-center gap-3 p-4 bg-white border border-[#111111]/5 rounded-lg">
                  <div className="w-2 h-2 bg-[#4FF978] rounded-full"></div>{item}
                </li>
              ))}
            </ul>
          )}

          {data.type === 'form' && (
            <form className="space-y-6 max-w-lg" onSubmit={(e) => e.preventDefault()}>
              <div><label className="block text-sm font-bold mb-2">Email</label><input type="email" className="w-full p-4 bg-white border border-gray-300 focus:border-[#111111] outline-none" placeholder="name@firm.com" /></div>
              <div><label className="block text-sm font-bold mb-2">Message</label><textarea className="w-full p-4 bg-white border border-gray-300 focus:border-[#111111] outline-none h-32" placeholder="How can we help?"></textarea></div>
              <button className="px-8 py-4 bg-[#111111] text-white font-bold hover:bg-black/90">Send Message</button>
            </form>
          )}

          {data.type === 'feature' && FeatureIcon && (
            <div className="grid md:grid-cols-2 gap-8">
              <div className="bg-[#111111] text-white p-8 rounded-xl flex flex-col justify-between min-h-[300px]">
                <FeatureIcon size={48} className="text-[#4FF978] mb-6" />
                <div><h3 className="text-2xl font-bold mb-4">Core Capability</h3><p className="text-gray-400">This module is part of the Enterprise Suite. It connects directly to your local database.</p></div>
                <button onClick={() => setPage('download')} className="mt-8 text-[#4FF978] hover:underline flex items-center gap-2">Download Software <ArrowRight size={16}/></button>
              </div>
              <div className="space-y-4">
                <div className="p-6 bg-white border border-gray-200 rounded-xl"><h4 className="font-bold mb-2">Real-time Sync</h4><p className="text-sm text-gray-600">Changes reflect instantly on mobile.</p></div>
                <div className="p-6 bg-white border border-gray-200 rounded-xl"><h4 className="font-bold mb-2">Offline Mode</h4><p className="text-sm text-gray-600">Access data without internet.</p></div>
              </div>
            </div>
          )}

          {data.type === 'tool' && (
            <div className="mt-8">
              {data.toolType === 'mca' && <McaTool />}
              {data.toolType === 'board' && <BoardRiskTool />}
              {data.toolType === 'trust' && <TrustScoreTool />}
              {data.toolType === 'regulator' && <RegulatorTool />}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};