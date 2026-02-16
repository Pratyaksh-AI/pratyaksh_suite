import React from 'react';
import { ArrowUpRight } from 'lucide-react';
import { THEME, PAGE_CONTENT } from '../data/constants';

export const Footer = ({ setPage }) => {
  const footerLinks = {
    Company: ['about', 'careers', 'news', 'contact'],
    Resources: ['documentation', 'api_ref', 'status', 'case_studies'],
    Legal: ['privacy', 'terms', 'legal'],
    Tools: ['mca_predictor', 'board_risk', 'trust_score', 'reg_notebook']
  };

  return (
    <footer className={`${THEME.bg} pt-24 pb-12 px-6 border-t border-[#111111]/10`}>
      <div className="max-w-[1400px] mx-auto">
        <div className="grid lg:grid-cols-5 gap-12 mb-20">
          <div className="lg:col-span-2">
            <h2 className="text-4xl font-medium tracking-tighter leading-none text-[#111111] mb-8">
              Pratyaksh — <br/> Smart Compliance.
            </h2>
            <div className="flex w-full max-w-sm border-b border-black pb-2">
              <input type="email" placeholder="Email Address *" className="bg-transparent w-full outline-none placeholder-black/50 text-lg"/>
              <ArrowUpRight />
            </div>
          </div>
          
          {Object.entries(footerLinks).map(([cat, links]) => (
            <div key={cat}>
              <h4 className="font-bold mb-6">{cat}</h4>
              <ul className="space-y-3 text-[#666666] text-sm">
                {links.map(linkKey => {
                  const label = PAGE_CONTENT[linkKey]?.title || linkKey;
                  return (
                    <li key={linkKey}>
                      <button onClick={() => setPage(linkKey)} className="hover:text-black hover:underline underline-offset-4 text-left decoration-[#BEF264] decoration-2">
                        {label}
                      </button>
                    </li>
                  );
                })}
              </ul>
            </div>
          ))}
        </div>
        <div className="pt-8 border-t border-[#111111]/10 text-sm text-[#666666] flex justify-between">
          <div>© 2026 PratyakshAI.</div>
          <div>Pune • Mumbai • Bangalore</div>
        </div>
      </div>
    </footer>
  );
};