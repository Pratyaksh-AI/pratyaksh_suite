import React, { useState } from 'react';
import { Search } from 'lucide-react';

export const McaTool = () => {
  const [city, setCity] = useState("Pune");
  const [type, setType] = useState("AOC-4");
  const [result, setResult] = useState(null);

  const calculate = () => {
    let baseScore = 95;
    if (city === "Pune") baseScore -= 15; 
    if (city === "Bangalore") baseScore -= 10;
    if (type === "AOC-4") baseScore -= 5;
    
    setResult({
      score: baseScore,
      risk: baseScore > 80 ? "Low" : baseScore > 60 ? "Medium" : "High",
      msg: baseScore > 80 ? "Likely Acceptance" : "Scrutiny Probability High"
    });
  };

  return (
    <div className="bg-white p-8 rounded-xl border border-[#111111]/10">
      <div className="grid md:grid-cols-2 gap-6 mb-6">
        <div>
          <label className="block text-sm font-bold mb-2">ROC City</label>
          <select value={city} onChange={(e) => setCity(e.target.value)} className="w-full p-3 bg-gray-50 border border-gray-200 rounded outline-none focus:border-[#111111]">
            <option value="Pune">Pune</option>
            <option value="Mumbai">Mumbai</option>
            <option value="Bangalore">Bangalore</option>
            <option value="Ahmedabad">Ahmedabad</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-bold mb-2">Form Type</label>
          <select value={type} onChange={(e) => setType(e.target.value)} className="w-full p-3 bg-gray-50 border border-gray-200 rounded outline-none focus:border-[#111111]">
            <option value="AOC-4">AOC-4 (Financials)</option>
            <option value="MGT-7">MGT-7 (Annual Return)</option>
            <option value="DIR-3">DIR-3 KY</option>
          </select>
        </div>
      </div>
      <button onClick={calculate} className="w-full py-3 bg-[#111111] text-white font-bold rounded hover:bg-black/90 transition-colors">Predict Outcome</button>
      
      {result && (
        <div className="mt-8 p-6 bg-[#F3F2EC] border border-[#111111]/10 rounded-lg animate-in fade-in slide-in-from-bottom-4 duration-500">
          <div className="flex justify-between items-center mb-2">
            <span className="text-sm font-bold uppercase text-gray-500">Probability</span>
            <span className={`text-2xl font-bold ${result.score > 80 ? 'text-green-600' : 'text-red-600'}`}>{result.score}%</span>
          </div>
          <div className="w-full bg-gray-200 h-2 rounded-full mb-4">
            <div className={`h-2 rounded-full ${result.score > 80 ? 'bg-green-500' : 'bg-red-500'}`} style={{width: `${result.score}%`}}></div>
          </div>
          <p className="font-medium text-[#111111]">{result.msg}</p>
        </div>
      )}
    </div>
  );
};

export const BoardRiskTool = () => {
  const [text, setText] = useState("");
  const [analysis, setAnalysis] = useState(null);

  const analyze = () => {
    let risks = [];
    if (text.toLowerCase().includes("loan")) risks.push("Sec 185 Violation Risk (Loan to Director)");
    if (text.toLowerCase().includes("related party")) risks.push("Sec 188 RPT Audit Required");
    if (text.length > 0 && risks.length === 0) risks.push("Standard Resolution - Low Risk");
    setAnalysis(risks);
  };

  return (
    <div className="bg-white p-8 rounded-xl border border-[#111111]/10">
      <label className="block text-sm font-bold mb-2">Draft Resolution Text</label>
      <textarea 
        value={text}
        onChange={(e) => setText(e.target.value)}
        className="w-full p-4 bg-gray-50 border border-gray-200 rounded mb-6 h-32 outline-none focus:border-[#111111]" 
        placeholder="e.g., RESOLVED THAT the company hereby grants a loan of Rs. 50,00,000 to Mr. Sharma..."
      ></textarea>
      <button onClick={analyze} className="w-full py-3 bg-[#111111] text-white font-bold rounded hover:bg-black/90 transition-colors">Scan for Compliance Risks</button>
      {analysis && (
        <div className="mt-6 space-y-2">
          {analysis.map((risk, i) => (
            <div key={i} className={`p-4 rounded border ${risk.includes("Risk") || risk.includes("Violation") ? 'bg-red-50 border-red-100 text-red-700' : 'bg-green-50 border-green-100 text-green-700'}`}>
              {risk}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export const TrustScoreTool = () => {
  const [gst, setGst] = useState("");
  const [bank, setBank] = useState("");
  const [score, setScore] = useState(null);

  const calculate = () => {
    const g = parseFloat(gst) || 0;
    const b = parseFloat(bank) || 0;
    if (g === 0 || b === 0) return;
    const diff = Math.abs(g - b);
    const ratio = diff / b;
    let s = 100 - (ratio * 100);
    setScore(Math.max(0, Math.min(100, Math.round(s))));
  };

  return (
    <div className="bg-white p-8 rounded-xl border border-[#111111]/10">
      <div className="grid md:grid-cols-2 gap-6 mb-6">
        <div>
          <label className="block text-sm font-bold mb-2">GST Turnover (₹)</label>
          <input type="number" value={gst} onChange={(e) => setGst(e.target.value)} className="w-full p-3 bg-gray-50 border border-gray-200 rounded outline-none focus:border-[#111111]" placeholder="5000000" />
        </div>
        <div>
          <label className="block text-sm font-bold mb-2">Bank Credits (₹)</label>
          <input type="number" value={bank} onChange={(e) => setBank(e.target.value)} className="w-full p-3 bg-gray-50 border border-gray-200 rounded outline-none focus:border-[#111111]" placeholder="4800000" />
        </div>
      </div>
      <button onClick={calculate} className="w-full py-3 bg-[#111111] text-white font-bold rounded hover:bg-black/90 transition-colors">Calculate Integrity Score</button>
      {score !== null && (
        <div className="mt-8 text-center">
          <div className="text-6xl font-bold mb-2 text-[#111111]">{score}/100</div>
          <p className="text-gray-500">{score < 80 ? "Mismatch Detected - High Audit Risk" : "Clean Record - Low Risk"}</p>
        </div>
      )}
    </div>
  );
};

export const RegulatorTool = () => {
  const [id, setId] = useState("");
  const [note, setNote] = useState(null);

  const search = () => {
    if (id === "AO-PUNE-05") setNote("Strict on ITC claims. Requires physical invoices.");
    else if (id === "AO-MUM-02") setNote("Accepts digital reconciliations. Delays replies by ~15 days.");
    else setNote("No data found for this Officer ID in the private network.");
  };

  return (
    <div className="bg-white p-8 rounded-xl border border-[#111111]/10">
      <label className="block text-sm font-bold mb-2">Officer ID / Ward Number</label>
      <div className="flex gap-4 mb-6">
        <input type="text" value={id} onChange={(e) => setId(e.target.value)} className="flex-1 p-3 bg-gray-50 border border-gray-200 rounded outline-none focus:border-[#111111]" placeholder="e.g. AO-PUNE-05" />
        <button onClick={search} className="px-6 bg-[#111111] text-white rounded hover:bg-black/90 transition-colors"><Search size={20}/></button>
      </div>
      {note && (
        <div className="p-6 bg-[#F3F2EC] border-l-4 border-[#BEF264] rounded-r-lg">
          <h4 className="font-bold mb-1">Intelligence Note:</h4>
          <p className="text-[#333333]">{note}</p>
        </div>
      )}
    </div>
  );
};