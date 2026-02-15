import React from 'react';
import { ArrowRight, FileText, Server, Shield, BookOpen } from 'lucide-react';
import { THEME, PAGE_CONTENT } from '../data/constants';

// This component handles all the text-heavy resource pages dynamically
export const ResourcePage = ({ pageKey, setPage }) => {
  const data = PAGE_CONTENT[pageKey];
  
  // Icon mapping for header
  const getIcon = () => {
    if (pageKey.includes('legal') || pageKey.includes('privacy')) return Shield;
    if (pageKey.includes('api') || pageKey.includes('status')) return Server;
    if (pageKey.includes('doc')) return BookOpen;
    return FileText;
  };
  
  const Icon = getIcon();

  return (
    <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
      <div className="max-w-4xl mx-auto">
        <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
          <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back Home
        </button>

        <div className="bg-white p-12 md:p-16 border border-[#111111]/10 shadow-sm">
           <div className="flex items-center gap-4 mb-8 text-[#111111]/50">
              <Icon size={32} />
              <span className="uppercase tracking-widest text-sm font-bold">Resource Center</span>
           </div>
           
           <h1 className="text-5xl font-medium tracking-tighter mb-6 text-[#111111]">{data.title}</h1>
           <p className="text-2xl text-[#666666] mb-12 font-light border-b border-[#111111]/10 pb-12">{data.subtitle}</p>

           <div className="prose prose-xl max-w-none text-[#333333]">
              {/* Real Content Rendering */}
              {data.type === 'text' && (
                <div className="space-y-6">
                   <p>{data.content}</p>
                   <p>This is a legally binding document representing the policies of PratyakshAI Inc., Pune.</p>
                   <p>Last Updated: February 15, 2026</p>
                </div>
              )}
              
              {data.type === 'list' && (
                <ul className="space-y-4 list-none pl-0">
                  {data.items.map((item, i) => (
                    <li key={i} className="flex items-start gap-4 p-4 bg-[#F3F2EC] rounded-lg">
                       <div className="mt-1.5 w-2 h-2 bg-[#111111] rounded-full shrink-0"></div>
                       <span>{item}</span>
                    </li>
                  ))}
                </ul>
              )}

              {data.type === 'code' && (
                <div className="bg-[#111111] text-white p-6 rounded-lg font-mono text-sm overflow-x-auto border-l-4 border-[#4FF978]">
                   <pre>{data.content}</pre>
                </div>
              )}
           </div>
        </div>
      </div>
    </div>
  );
};