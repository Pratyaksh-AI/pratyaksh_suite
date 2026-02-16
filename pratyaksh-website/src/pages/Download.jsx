import React from 'react';
import { Download, ArrowRight, Globe, Smartphone } from 'lucide-react';
import { THEME } from '../data/constants';
import AccessGuard from '../components/AccessGuard';

export const DownloadScreen = ({ setPage }) => (
  <AccessGuard onDeny={() => setPage('pricing')}>
    <div className={`min-h-screen ${THEME.bg} pt-32 px-6`}>
      <div className="max-w-5xl mx-auto">
        <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
          <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back to Home
        </button>
        
        <div className="grid lg:grid-cols-2 gap-16">
          <div>
            <h1 className="text-6xl font-medium tracking-tighter mb-8 text-[#111111]">Download Center</h1>
            <p className="text-xl text-[#666666] leading-relaxed mb-12">
              Get the full PratyakshAI suite. Ensure you have your license key ready for activation.<br/><br/>
              Version 6.0.0 (Stable) • Updated Feb 2026
            </p>
            <div className="space-y-6">
              <div className="bg-white p-8 border border-[#111111]/10 hover:border-[#111111] transition-colors group cursor-pointer shadow-sm">
                <div className="flex justify-between items-start mb-6"><Globe className="w-10 h-10 text-[#4353FF]" /><span className="text-xs font-mono border border-[#111111]/20 px-2 py-1">64-BIT</span></div>
                <h3 className="text-2xl font-medium text-[#111111] mb-2">Windows Desktop</h3>
                <p className="text-[#666666] mb-8">The complete control center for your firm.</p>
                <button className="flex items-center gap-3 text-[#111111] font-bold group-hover:underline transition-colors uppercase tracking-wide text-sm">Download .EXE <Download size={16}/></button>
              </div>
              <div className="bg-white p-8 border border-[#111111]/10 hover:border-[#111111] transition-colors group cursor-pointer shadow-sm">
                <div className="flex justify-between items-start mb-6"><Smartphone className="w-10 h-10 text-[#4FF978]" /><span className="text-xs font-mono border border-[#111111]/20 px-2 py-1">ANDROID 10+</span></div>
                <h3 className="text-2xl font-medium text-[#111111] mb-2">Android Companion</h3>
                <p className="text-[#666666] mb-8">Alerts and evidence capture on the go.</p>
                <button className="flex items-center gap-3 text-[#111111] font-bold group-hover:underline transition-colors uppercase tracking-wide text-sm">Download .APK <Download size={16}/></button>
              </div>
            </div>
          </div>
          <div className="bg-[#111111] p-12 flex flex-col justify-between text-white">
            <div>
              <h4 className="text-lg font-medium mb-6">System Requirements</h4>
              <ul className="space-y-4 text-gray-400 text-sm">
                <li className="flex justify-between border-b border-white/10 pb-2"><span>OS</span> <span>Windows 10/11</span></li>
                <li className="flex justify-between border-b border-white/10 pb-2"><span>RAM</span> <span>8GB Minimum</span></li>
                <li className="flex justify-between border-b border-white/10 pb-2"><span>Storage</span> <span>500MB SSD</span></li>
              </ul>
            </div>
            <div className="bg-[#4FF978] p-6 mt-12 text-black">
              <h5 className="font-bold mb-2">Need Enterprise?</h5>
              <p className="text-sm opacity-80 mb-4">On-premise setup available.</p>
              <span className="underline font-bold cursor-pointer hover:opacity-80" onClick={() => setPage('contact')}>Contact Sales</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AccessGuard>
);