import React from 'react';
import { ArrowRight, Search, Wrench } from 'lucide-react';
import { THEME, PAGE_CONTENT } from '../data/constants';

export const ToolsPage = ({ setPage }) => {
  // Filter only the interactive tools from the content database
  const allTools = Object.entries(PAGE_CONTENT).filter(([_, data]) => data.type === 'tool');

  return (
    <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
      <div className="max-w-7xl mx-auto">
        <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
          <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back to Home
        </button>

        <div className="mb-16">
          <div className="flex items-center gap-3 mb-4">
            <div className={`p-2 ${THEME.accent} rounded-md`}>
              <Wrench size={24} className="text-[#111111]" />
            </div>
            <span className="text-[#111111] font-mono tracking-widest uppercase text-sm">Intelligence Suite</span>
          </div>
          <h1 className="text-5xl md:text-7xl font-medium tracking-tighter mb-6 text-[#111111]">
            Professional Tools
          </h1>
          <p className="text-xl text-[#666666] max-w-2xl font-light">
            Access our complete library of {allTools.length} regulatory calculators, risk engines, and compliance validators. All processing happens locally on your device.
          </p>
        </div>

        {/* Search / Filter Placeholder (Visual) */}
        <div className="flex items-center gap-4 bg-white p-4 border border-[#111111]/10 rounded-lg mb-12 max-w-xl">
          <Search className="text-gray-400" />
          <input 
            type="text" 
            placeholder="Search for a tool (e.g. 'Gratuity', 'Penalty', 'Risk')..." 
            className="w-full outline-none text-[#111111] placeholder-gray-400"
          />
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {allTools.map(([key, tool]) => {
            const ToolIcon = tool.icon;
            return (
              <div 
                key={key}
                onClick={() => setPage(key)}
                className="group bg-white p-8 rounded-xl border border-[#111111]/5 hover:border-[#111111] transition-all cursor-pointer flex flex-col justify-between h-full hover:shadow-xl hover:shadow-black/5"
              >
                <div>
                  <div className="flex justify-between items-start mb-6">
                    <div className="p-3 bg-[#F3F2EC] rounded-lg group-hover:bg-[#4FF978] transition-colors">
                      {ToolIcon && <ToolIcon size={24} className="text-[#111111]" />}
                    </div>
                    <ArrowRight className="text-gray-300 group-hover:text-[#111111] transform group-hover:-rotate-45 transition-all" />
                  </div>
                  
                  <h3 className="text-xl font-bold text-[#111111] mb-2 group-hover:underline decoration-2 underline-offset-4 decoration-[#4FF978]">
                    {tool.title}
                  </h3>
                  <p className="text-[#666666] text-sm leading-relaxed">
                    {tool.subtitle}
                  </p>
                </div>

                <div className="mt-8 pt-6 border-t border-[#111111]/5 flex items-center justify-between text-xs font-mono text-gray-400 uppercase tracking-wide">
                  <span>Launch Tool</span>
                  <span className="opacity-0 group-hover:opacity-100 transition-opacity text-[#111111]">v6.0</span>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};