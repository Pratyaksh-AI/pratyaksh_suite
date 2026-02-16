import React, { useState } from 'react';
import { ArrowRight, Mail, Phone, MapPin } from 'lucide-react';
import { THEME } from '../data/constants';

export const ContactPage = ({ setPage }) => {
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = (e) => {
    e.preventDefault();
    setSubmitted(true);
  };

  return (
    <div className={`min-h-screen ${THEME.bg} pt-32 pb-20 px-6`}>
      <div className="max-w-7xl mx-auto grid lg:grid-cols-2 gap-20">
        <div>
          <button onClick={() => setPage('home')} className="mb-12 flex items-center gap-2 text-[#666666] hover:text-black group">
            <ArrowRight className="rotate-180 group-hover:-translate-x-1 transition-transform" size={20}/> Back Home
          </button>
          <h1 className="text-6xl font-medium tracking-tighter mb-8 text-[#111111]">Get in touch.</h1>
          <p className="text-xl text-[#666666] leading-relaxed mb-12">
            Whether you need a demo, enterprise pricing, or technical support, our team in Pune is ready to assist.
          </p>

          <div className="space-y-8">
            <div className="flex items-start gap-6">
               <div className="w-12 h-12 bg-[#111111] text-white flex items-center justify-center rounded-full"><Mail size={20}/></div>
               <div>
                  <h4 className="font-bold text-lg">Email Us</h4>
                  <p className="text-[#666666]">support@pratyaksh.ai</p>
                  <p className="text-[#666666]">sales@pratyaksh.ai</p>
               </div>
            </div>
            <div className="flex items-start gap-6">
               <div className="w-12 h-12 bg-[#111111] text-white flex items-center justify-center rounded-full"><Phone size={20}/></div>
               <div>
                  <h4 className="font-bold text-lg">Call Us</h4>
                  <p className="text-[#666666]">+91 98765 43210</p>
                  <p className="text-sm text-gray-400">Mon-Fri, 9am - 7pm IST</p>
               </div>
            </div>
            <div className="flex items-start gap-6">
               <div className="w-12 h-12 bg-[#111111] text-white flex items-center justify-center rounded-full"><MapPin size={20}/></div>
               <div>
                  <h4 className="font-bold text-lg">Visit HQ</h4>
                  <p className="text-[#666666]">405, Tech Park, Baner Road,<br/>Pune, Maharashtra 411045</p>
               </div>
            </div>
          </div>
        </div>

        <div className="bg-white p-10 border border-[#111111]/10 rounded-2xl shadow-sm">
           {submitted ? (
             <div className="h-full flex flex-col items-center justify-center text-center">
                <div className="w-20 h-20 bg-[#4FF978] rounded-full flex items-center justify-center mb-6">
                    <Check size={40} className="text-black" />
                </div>
                <h3 className="text-2xl font-bold mb-4">Message Sent!</h3>
                <p className="text-[#666666]">Thank you for contacting us. A representative will reach out to you within 24 hours.</p>
                <button onClick={() => setSubmitted(false)} className="mt-8 underline">Send another</button>
             </div>
           ) : (
             <form onSubmit={handleSubmit} className="space-y-6">
                <div>
                    <label className="block text-sm font-bold mb-2">Full Name</label>
                    <input type="text" required className="w-full p-4 bg-[#F3F2EC] border border-transparent focus:border-[#111111] outline-none rounded" placeholder="John Doe" />
                </div>
                <div>
                    <label className="block text-sm font-bold mb-2">Work Email</label>
                    <input type="email" required className="w-full p-4 bg-[#F3F2EC] border border-transparent focus:border-[#111111] outline-none rounded" placeholder="name@company.com" />
                </div>
                <div>
                    <label className="block text-sm font-bold mb-2">Company / Firm</label>
                    <input type="text" className="w-full p-4 bg-[#F3F2EC] border border-transparent focus:border-[#111111] outline-none rounded" placeholder="ABC Associates" />
                </div>
                <div>
                    <label className="block text-sm font-bold mb-2">How can we help?</label>
                    <textarea required className="w-full p-4 bg-[#F3F2EC] border border-transparent focus:border-[#111111] outline-none rounded h-40" placeholder="I'm interested in the Enterprise plan..."></textarea>
                </div>
                <button type="submit" className="w-full py-4 bg-[#111111] text-white font-bold rounded hover:bg-black/90 transition-colors">Send Message</button>
             </form>
           )}
        </div>
      </div>
    </div>
  );
};