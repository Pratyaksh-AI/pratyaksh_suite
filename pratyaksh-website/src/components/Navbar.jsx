import React from 'react';
import { Menu, X } from 'lucide-react';
import { THEME } from '../data/constants';

const NavLink = ({ label, target, setPage, mobile }) => (
  <button 
    onClick={() => setPage(target)}
    className={`${mobile ? 'block w-full text-left py-3 text-2xl border-b border-white/10' : 'px-4 py-2'} font-medium hover:underline decoration-2 underline-offset-4 decoration-[#BEF264] transition-all`}
  >
    {label}
  </button>
);

export const Navbar = ({ setPage, mobileMenuOpen, setMobileMenuOpen }) => (
  <nav className={`sticky top-0 w-full z-50 ${THEME.bg} border-b ${THEME.border}`}>
    {/* Top Utility Bar */}
    <div className={`hidden md:flex justify-between items-center px-6 py-2 text-xs font-mono border-b ${THEME.border} ${THEME.bg} text-[#111111]`}>
      <div className="flex items-center gap-2">
        <span className="font-bold">PRATYAKSH AI® v6.0</span>
      </div>
      <div className="flex items-center gap-6">
        <button onClick={() => setPage('news')} className="hover:underline">News</button>
        <button onClick={() => setPage('status')} className="hover:underline">System Status: <span className="text-green-600">● Online</span></button>
        <span className="flex items-center gap-2">Pune, MH <span className="w-2 h-2 bg-[#4FF978] rounded-full animate-pulse"></span> {new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}</span>
      </div>
    </div>

    {/* Main Nav */}
    <div className="flex justify-between items-stretch h-20">
      <div 
        className="flex items-center px-6 border-r border-[#111111]/10 cursor-pointer hover:bg-white/50 transition-colors"
        onClick={() => setPage('home')}
      >
        <div className="w-6 h-6 bg-[#111111] mr-3 skew-x-12"></div>
        <span className="text-xl font-bold tracking-tight">Pratyaksh<span className="text-[#666666]">AI</span></span>
      </div>

      <div className="hidden md:flex flex-1 justify-end items-center px-6 gap-6 text-sm">
        <NavLink label="Modules" target="city_risk" setPage={setPage} />
        {/* UPDATED: Points to the main Tools Catalog now */}
        <NavLink label="Tools" target="tools" setPage={setPage} />
        <NavLink label="Pricing" target="pricing" setPage={setPage} />
        <NavLink label="Resources" target="documentation" setPage={setPage} />
        <NavLink label="Company" target="about" setPage={setPage} />
        
        <button 
          onClick={() => setPage('download')}
          className={`${THEME.accent} hover:bg-[#3dd665] text-[#111111] px-6 py-3 font-bold transition-colors rounded-sm flex items-center gap-2`}
        >
          Download Suite
        </button>
      </div>

      <button 
        className={`md:hidden px-6 flex items-center justify-center border-l ${THEME.border}`}
        onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
      >
        {mobileMenuOpen ? <X /> : <Menu />}
      </button>
    </div>

    {/* Mobile Dropdown */}
    {mobileMenuOpen && (
      <div className={`md:hidden ${THEME.darkBg} text-white p-6 min-h-screen fixed top-20 left-0 w-full z-40 overflow-y-auto`}>
        <div className="flex flex-col gap-2">
          <NavLink label="Home" target="home" setPage={setPage} mobile />
          <NavLink label="Modules" target="city_risk" setPage={setPage} mobile />
          {/* UPDATED: Mobile link also points to main Tools Catalog */}
          <NavLink label="Tools" target="tools" setPage={setPage} mobile />
          <NavLink label="Pricing" target="pricing" setPage={setPage} mobile />
          <NavLink label="Download" target="download" setPage={setPage} mobile />
          <NavLink label="About" target="about" setPage={setPage} mobile />
          <NavLink label="Contact" target="contact" setPage={setPage} mobile />
        </div>
      </div>
    )}
  </nav>
);