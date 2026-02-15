import React from 'react';
import { ArrowRight, MapPin, Users, Award } from 'lucide-react';
import { THEME } from '../data/constants';

export const AboutPage = ({ setPage }) => (
  <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
    <div className="max-w-5xl mx-auto">
      <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
        <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back Home
      </button>

      <div className="grid md:grid-cols-2 gap-16 mb-24">
        <div>
           <h1 className="text-6xl font-medium tracking-tighter mb-8 text-[#111111]">Our Mission</h1>
           <p className="text-xl leading-relaxed text-[#333333]">
             To decode the complexity of Indian regulatory compliance into predictive, actionable intelligence. We believe that Chartered Accountants and Company Secretaries are the architects of the economy, and they deserve tools that are as forward-thinking as they are.
           </p>
        </div>
        <div className="bg-[#111111] p-12 text-white flex flex-col justify-end">
           <div className="text-[#4FF978] text-6xl font-bold mb-4">2024</div>
           <p className="text-gray-400">Founded in Pune, Maharashtra.</p>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-8 mb-24">
        <div className="p-8 bg-white border border-[#111111]/10">
          <MapPin className="w-10 h-10 mb-6 text-[#111111]" />
          <h3 className="text-2xl font-bold mb-4">Local First</h3>
          <p className="text-[#666666]">We don't build generic software. We build tools aware of the specific "Ward Office" reality in Pune, Mumbai, and Bangalore.</p>
        </div>
        <div className="p-8 bg-white border border-[#111111]/10">
          <Users className="w-10 h-10 mb-6 text-[#111111]" />
          <h3 className="text-2xl font-bold mb-4">Community Led</h3>
          <p className="text-[#666666]">Built in direct partnership with the ICAI and ICSI chapters of Maharashtra.</p>
        </div>
        <div className="p-8 bg-white border border-[#111111]/10">
          <Award className="w-10 h-10 mb-6 text-[#111111]" />
          <h3 className="text-2xl font-bold mb-4">Excellence</h3>
          <p className="text-[#666666]">Our risk engines are calibrated to 98% accuracy against historical notice data.</p>
        </div>
      </div>

      <div className="border-t border-[#111111]/10 pt-24">
         <h2 className="text-4xl font-medium tracking-tighter mb-12 text-[#111111]">Leadership</h2>
         <div className="grid md:grid-cols-4 gap-8">
            {["Rajesh Verma (CEO)", "Anjali Desopt (CTO)", "Vikram Seth (Legal)", "Meera K. (Product)"].map((name, i) => (
                <div key={i} className="group cursor-pointer">
                    <div className="aspect-square bg-gray-200 mb-4 overflow-hidden grayscale group-hover:grayscale-0 transition-all">
                        {/* Placeholder for real images */}
                        <div className="w-full h-full bg-[#111111] flex items-center justify-center text-white/20 text-4xl font-bold">{name[0]}</div>
                    </div>
                    <div className="font-bold text-lg">{name.split(' (')[0]}</div>
                    <div className="text-sm text-[#666666]">{name.split('(')[1].replace(')', '')}</div>
                </div>
            ))}
         </div>
      </div>
    </div>
  </div>
);