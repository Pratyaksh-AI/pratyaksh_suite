import React, { useState, useEffect } from 'react';
import { Calculator, AlertTriangle, CheckCircle, XCircle } from 'lucide-react';

// --- HELPER COMPONENTS ---
const ResultBox = ({ label, value, isError = false }) => (
  <div className={`p-4 rounded-lg mt-4 border ${isError ? 'bg-red-50 border-red-200 text-red-800' : 'bg-[#F3F2EC] border-[#111111]/10'}`}>
    <div className="text-xs font-bold uppercase tracking-wider opacity-70">{label}</div>
    <div className="text-2xl font-mono font-bold mt-1">{value}</div>
  </div>
);

const InputGroup = ({ label, type = "number", value, onChange, placeholder, options }) => (
  <div className="mb-4">
    <label className="block text-sm font-bold mb-2 text-[#111111]">{label}</label>
    {options ? (
      <select value={value} onChange={onChange} className="w-full p-3 bg-white border border-gray-200 rounded focus:border-[#4FF978] outline-none transition-colors">
        {options.map(opt => <option key={opt} value={opt}>{opt}</option>)}
      </select>
    ) : (
      <input 
        type={type} 
        value={value} 
        onChange={onChange} 
        placeholder={placeholder}
        className="w-full p-3 bg-white border border-gray-200 rounded focus:border-[#4FF978] outline-none transition-colors font-mono text-sm"
      />
    )}
  </div>
);

// --- 1. MSME 45-Day Payment Interest Calculator (Sec 43B-h) ---
export const MsmeCalculator = () => {
  const [amount, setAmount] = useState("");
  const [invoiceDate, setInvoiceDate] = useState("");
  const [paymentDate, setPaymentDate] = useState("");
  const [rbiRate, setRbiRate] = useState("6.50"); // Default Repo Rate
  const [interest, setInterest] = useState(null);

  const calculate = () => {
    const inv = new Date(invoiceDate);
    const pay = new Date(paymentDate);
    if (!amount || isNaN(inv) || isNaN(pay)) return;

    // Logic: 15 days default, 45 days if agreement exists. Assuming 45 for generic tool.
    const diffTime = Math.abs(pay - inv);
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)); 
    
    if (diffDays <= 45) {
      setInterest({ val: "No Interest (Compliant)", error: false });
      return;
    }

    // Sec 16 MSMED Act: 3x Bank Rate compounded monthly
    const delayDays = diffDays - 15; // Strict 15 day liability trigger for calculation base
    const rate = (parseFloat(rbiRate) * 3) / 100;
    const principal = parseFloat(amount);
    
    // Monthly compounding approximation
    const months = delayDays / 30;
    const totalAmount = principal * Math.pow((1 + rate/12), months);
    const interestAmt = totalAmount - principal;

    setInterest({ val: `₹${interestAmt.toFixed(2)}`, error: true });
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Invoice Amount (₹)" value={amount} onChange={e => setAmount(e.target.value)} />
      <div className="grid grid-cols-2 gap-4">
        <InputGroup label="Invoice Date" type="date" value={invoiceDate} onChange={e => setInvoiceDate(e.target.value)} />
        <InputGroup label="Payment Date" type="date" value={paymentDate} onChange={e => setPaymentDate(e.target.value)} />
      </div>
      <InputGroup label="RBI Repo Rate (%)" value={rbiRate} onChange={e => setRbiRate(e.target.value)} />
      <button onClick={calculate} className="w-full py-3 bg-black text-white font-bold rounded">Check Compliance</button>
      {interest && <ResultBox label={interest.error ? "Sec 16 Interest Liability" : "Status"} value={interest.val} isError={interest.error} />}
    </div>
  );
};

// --- 2. Related Party Transaction Monitor (Sec 188) ---
export const RptMonitor = () => {
  const [turnover, setTurnover] = useState("");
  const [transValue, setTransValue] = useState("");
  const [type, setType] = useState("Sale/Purchase of Goods");
  const [status, setStatus] = useState(null);

  const check = () => {
    const to = parseFloat(turnover);
    const val = parseFloat(transValue);
    if (!to || !val) return;

    let limit = 0;
    let limitPercent = 0.10; // 10% of turnover is standard trigger

    if (type === "Sale/Purchase of Goods") limit = to * 0.10;
    else if (type === "Selling Property") limit = to * 0.10;
    else if (type === "Leasing Property") limit = to * 0.10;
    else if (type === "Availing Services") limit = to * 0.10;

    if (val >= limit) {
      setStatus({ msg: "Shareholder Approval Required (OR passed)", color: "red" });
    } else {
      setStatus({ msg: "Board Resolution Sufficient", color: "green" });
    }
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Audited Turnover (Prev FY)" value={turnover} onChange={e => setTurnover(e.target.value)} />
      <InputGroup label="Transaction Value" value={transValue} onChange={e => setTransValue(e.target.value)} />
      <InputGroup label="Transaction Type" options={["Sale/Purchase of Goods", "Selling Property", "Leasing Property", "Availing Services"]} value={type} onChange={e => setType(e.target.value)} />
      <button onClick={check} className="w-full py-3 bg-black text-white font-bold rounded">Check Threshold</button>
      {status && <div className={`mt-4 p-4 font-bold ${status.color === 'red' ? 'text-red-600 bg-red-50' : 'text-green-600 bg-green-50'}`}>{status.msg}</div>}
    </div>
  );
};

// --- 3. Director Disqualification Risk (Sec 164) ---
export const DirectorRisk = () => {
  const [years, setYears] = useState("0");
  const [defaults, setDefaults] = useState("No");
  const [risk, setRisk] = useState(null);

  const analyze = () => {
    const y = parseInt(years);
    if (y >= 3) {
      setRisk("CRITICAL: Immediate Disqualification (Sec 164(2)(a))");
    } else if (defaults === "Yes") {
      setRisk("HIGH: Check Deposits/Debentures Interest (Sec 164(2)(b))");
    } else {
      setRisk("LOW: Compliant");
    }
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Consecutive Years Financials Not Filed" type="number" value={years} onChange={e => setYears(e.target.value)} />
      <InputGroup label="Defaulted on Deposit/Dividend > 1 Year?" options={["No", "Yes"]} value={defaults} onChange={e => setDefaults(e.target.value)} />
      <button onClick={analyze} className="w-full py-3 bg-black text-white font-bold rounded">Analyze Risk</button>
      {risk && <ResultBox label="Director Status" value={risk} isError={risk.includes("CRITICAL")} />}
    </div>
  );
};

// --- 4. CSR Spending Calculator ---
export const CsrCalculator = () => {
  const [p1, setP1] = useState("");
  const [p2, setP2] = useState("");
  const [p3, setP3] = useState("");
  const [obligation, setObligation] = useState(null);

  const calc = () => {
    const avg = (parseFloat(p1) + parseFloat(p2) + parseFloat(p3)) / 3;
    const csr = avg * 0.02;
    setObligation(csr.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <div className="grid grid-cols-3 gap-2">
        <InputGroup label="Net Profit FY-1" value={p1} onChange={e => setP1(e.target.value)} />
        <InputGroup label="Net Profit FY-2" value={p2} onChange={e => setP2(e.target.value)} />
        <InputGroup label="Net Profit FY-3" value={p3} onChange={e => setP3(e.target.value)} />
      </div>
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate 2% Obligation</button>
      {obligation && <ResultBox label="Minimum CSR Spend Required" value={`₹${obligation}`} />}
    </div>
  );
};

// --- 5. ITC Reversal (Rule 42/43) ---
export const ItcReversal = () => {
  const [totalItc, setTotalItc] = useState("");
  const [exemptTurnover, setExemptTurnover] = useState("");
  const [totalTurnover, setTotalTurnover] = useState("");
  const [reversal, setReversal] = useState(null);

  const calc = () => {
    const t = parseFloat(totalItc);
    const e = parseFloat(exemptTurnover);
    const f = parseFloat(totalTurnover);
    if (!f) return;
    
    // Formula: (Exempt / Total) * Common Credit. Simplified here as Total ITC * Ratio
    const rev = t * (e / f);
    setReversal(rev.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Common Input Tax Credit" value={totalItc} onChange={e => setTotalItc(e.target.value)} />
      <div className="grid grid-cols-2 gap-4">
        <InputGroup label="Exempt Turnover" value={exemptTurnover} onChange={e => setExemptTurnover(e.target.value)} />
        <InputGroup label="Total Turnover" value={totalTurnover} onChange={e => setTotalTurnover(e.target.value)} />
      </div>
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Reversal</button>
      {reversal && <ResultBox label="ITC to be Reversed (D1)" value={`₹${reversal}`} isError={true} />}
    </div>
  );
};

// --- 6. New vs Old Tax Regime Analyzer ---
export const TaxRegimeAnalyzer = () => {
  const [income, setIncome] = useState("");
  const [deductions, setDeductions] = useState(""); // 80C, 80D, HRA etc
  const [result, setResult] = useState(null);

  const calc = () => {
    const inc = parseFloat(income);
    const ded = parseFloat(deductions);
    
    // Old Regime (Simplified Slabs 2024-25)
    const taxableOld = inc - ded - 50000; // Std Ded
    let taxOld = 0;
    if (taxableOld > 1000000) taxOld += (taxableOld - 1000000) * 0.3 + 112500;
    else if (taxableOld > 500000) taxOld += (taxableOld - 500000) * 0.2 + 12500;
    
    // New Regime (FY 2025-26 Slabs)
    const taxableNew = inc - 75000; // New Std Ded
    let taxNew = 0;
    if (taxableNew > 1500000) taxNew += (taxableNew - 1500000) * 0.3 + 150000; // Approx
    else if (taxableNew > 1200000) taxNew += (taxableNew - 1200000) * 0.2 + 90000;
    
    const diff = taxOld - taxNew;
    setResult({
      old: taxOld.toFixed(0),
      new: taxNew.toFixed(0),
      recommendation: taxNew < taxOld ? "NEW REGIME" : "OLD REGIME",
      savings: Math.abs(diff).toFixed(0)
    });
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Gross Salary / Income" value={income} onChange={e => setIncome(e.target.value)} />
      <InputGroup label="Total Deductions (80C, HRA, Home Loan)" value={deductions} onChange={e => setDeductions(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Compare Regimes</button>
      {result && (
        <div className="mt-4 grid grid-cols-2 gap-4">
          <ResultBox label="Old Tax" value={`₹${result.old}`} />
          <ResultBox label="New Tax" value={`₹${result.new}`} />
          <div className="col-span-2 p-3 bg-green-100 text-green-900 font-bold text-center rounded">
            Switch to {result.recommendation} (Save ₹{result.savings})
          </div>
        </div>
      )}
    </div>
  );
};

// --- 7. Advance Tax Estimator ---
export const AdvanceTaxEstimator = () => {
  const [estTax, setEstTax] = useState("");
  const [paid, setPaid] = useState("");
  const [quarter, setQuarter] = useState("Q1 (Jun 15)");
  const [liability, setLiability] = useState(null);

  const calc = () => {
    const tax = parseFloat(estTax);
    let targetPerc = 0;
    if (quarter.includes("Jun")) targetPerc = 0.15;
    if (quarter.includes("Sep")) targetPerc = 0.45;
    if (quarter.includes("Dec")) targetPerc = 0.75;
    if (quarter.includes("Mar")) targetPerc = 1.00;

    const targetAmount = tax * targetPerc;
    const due = targetAmount - parseFloat(paid);
    setLiability(due > 0 ? due : 0);
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Estimated Annual Tax Liability" value={estTax} onChange={e => setEstTax(e.target.value)} />
      <InputGroup label="TDS/Tax Already Paid" value={paid} onChange={e => setPaid(e.target.value)} />
      <InputGroup label="Current Quarter" options={["Q1 (Jun 15)", "Q2 (Sep 15)", "Q3 (Dec 15)", "Q4 (Mar 15)"]} value={quarter} onChange={e => setQuarter(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Installment</button>
      {liability !== null && <ResultBox label="Payable Now" value={`₹${liability.toFixed(2)}`} />}
    </div>
  );
};

// --- 8. Ind AS 116 Lease Calculator ---
export const LeaseCalculator = () => {
  const [payment, setPayment] = useState("");
  const [rate, setRate] = useState("10");
  const [years, setYears] = useState("5");
  const [rou, setRou] = useState(null);

  const calc = () => {
    const pmt = parseFloat(payment);
    const r = parseFloat(rate) / 100;
    const n = parseFloat(years);
    
    // PV of Annuity Formula
    const pv = pmt * ((1 - Math.pow(1 + r, -n)) / r);
    setRou(pv.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Annual Lease Payment" value={payment} onChange={e => setPayment(e.target.value)} />
      <div className="grid grid-cols-2 gap-4">
        <InputGroup label="Discount Rate (%)" value={rate} onChange={e => setRate(e.target.value)} />
        <InputGroup label="Lease Term (Years)" value={years} onChange={e => setYears(e.target.value)} />
      </div>
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate ROU Asset</button>
      {rou && <ResultBox label="Right of Use (ROU) Asset Value" value={`₹${rou}`} />}
    </div>
  );
};

// --- 9. Angel Tax Validator ---
export const AngelTaxValidator = () => {
  const [issuePrice, setIssuePrice] = useState("");
  const [fmv, setFmv] = useState("");
  const [status, setStatus] = useState(null);

  const calc = () => {
    const ip = parseFloat(issuePrice);
    const f = parseFloat(fmv);
    if (ip > f) {
      setStatus({ msg: `Taxable Income: ₹${ip - f}`, color: "red" });
    } else {
      setStatus({ msg: "Safe Harbor (No Angel Tax)", color: "green" });
    }
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Share Issue Price" value={issuePrice} onChange={e => setIssuePrice(e.target.value)} />
      <InputGroup label="Fair Market Value (FMV)" value={fmv} onChange={e => setFmv(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Check Sec 56(2)(viib)</button>
      {status && <ResultBox label="Assessment" value={status.msg} isError={status.color === 'red'} />}
    </div>
  );
};

// --- 10. Buyback Tax Calculator ---
export const BuybackTax = () => {
  const [shares, setShares] = useState("");
  const [price, setPrice] = useState("");
  const [tax, setTax] = useState(null);

  const calc = () => {
    const total = parseFloat(shares) * parseFloat(price);
    // Flat 20% + 12% Surcharge + 4% Cess = ~23.296%
    const liability = total * 0.23296; 
    setTax(liability.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Number of Shares" value={shares} onChange={e => setShares(e.target.value)} />
      <InputGroup label="Buyback Price" value={price} onChange={e => setPrice(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Liability</button>
      {tax && <ResultBox label="Buyback Tax Payable" value={`₹${tax}`} />}
    </div>
  );
};

// --- 11. Gratuity Calculator (New Code) ---
export const GratuityCalc = () => {
  const [salary, setSalary] = useState("");
  const [years, setYears] = useState("");
  const [grat, setGrat] = useState(null);

  const calc = () => {
    // Formula: (Basic + DA) * 15/26 * Years
    const res = parseFloat(salary) * (15/26) * parseFloat(years);
    setGrat(res.toFixed(0));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Last Drawn Monthly Salary (Basic + DA)" value={salary} onChange={e => setSalary(e.target.value)} />
      <InputGroup label="Years of Service" value={years} onChange={e => setYears(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Gratuity</button>
      {grat && <ResultBox label="Gratuity Payable" value={`₹${grat}`} />}
    </div>
  );
};

// --- 12. PMLA Red Flag Scanner ---
export const PmlaScanner = () => {
  const [txn, setTxn] = useState("");
  const [cash, setCash] = useState("No");
  const [alert, setAlert] = useState(null);

  const scan = () => {
    const amount = parseFloat(txn);
    if (amount > 1000000 || (amount > 50000 && cash === "Yes")) {
      setAlert("HIGH RISK: Enhanced Due Diligence (EDD) Required");
    } else {
      setAlert("Standard Risk");
    }
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Transaction Value" value={txn} onChange={e => setTxn(e.target.value)} />
      <InputGroup label="Is Cash Involved?" options={["No", "Yes"]} value={cash} onChange={e => setCash(e.target.value)} />
      <button onClick={scan} className="w-full py-3 bg-black text-white font-bold rounded">Scan Transaction</button>
      {alert && <ResultBox label="PMLA Status" value={alert} isError={alert.includes("HIGH")} />}
    </div>
  );
};

// --- 13. ESG Applicability ---
export const EsgChecker = () => {
  const [mcap, setMcap] = useState("");
  const [rank, setRank] = useState(null);

  const check = () => {
    // Determine rank based on MCap input (Simplified simulation)
    const m = parseFloat(mcap);
    if (m > 5000) {
      setRank("Mandatory BRSR (Business Responsibility & Sustainability Report)");
    } else {
      setRank("Voluntary Disclosure");
    }
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Market Capitalization (Crores)" value={mcap} onChange={e => setMcap(e.target.value)} />
      <button onClick={check} className="w-full py-3 bg-black text-white font-bold rounded">Check Applicability</button>
      {rank && <ResultBox label="ESG Requirement" value={rank} />}
    </div>
  );
};

// --- 14. UDIN Validator ---
export const UdinValidator = () => {
  const [udin, setUdin] = useState("");
  const [valid, setValid] = useState(null);

  const validate = () => {
    // Format: 18 digits (First 6 Mem No + Next 6 Doc Date + Last 6 Serial)
    const regex = /^[0-9]{18}$/;
    if (regex.test(udin)) setValid("Format Valid");
    else setValid("Invalid Format (Must be 18 digits)");
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Enter UDIN" value={udin} onChange={e => setUdin(e.target.value)} />
      <button onClick={validate} className="w-full py-3 bg-black text-white font-bold rounded">Validate Format</button>
      {valid && <ResultBox label="Result" value={valid} isError={valid.includes("Invalid")} />}
    </div>
  );
};

// --- 15. Audit Rotation Tracker ---
export const AuditRotation = () => {
  const [years, setYears] = useState("");
  const [status, setStatus] = useState(null);

  const check = () => {
    const y = parseInt(years);
    if (y >= 10) setStatus("Mandatory Rotation Required (Cooling Period Starts)");
    else setStatus(`${10 - y} Years Remaining`);
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Years Auditor Appointed" type="number" value={years} onChange={e => setYears(e.target.value)} />
      <button onClick={check} className="w-full py-3 bg-black text-white font-bold rounded">Check Status</button>
      {status && <ResultBox label="Rotation Status" value={status} isError={status.includes("Required")} />}
    </div>
  );
};

// --- 16. Net Worth Calculator (Sec 2(57)) ---
export const NetWorthCalc = () => {
  const [shareCap, setShareCap] = useState("");
  const [reserves, setReserves] = useState("");
  const [losses, setLosses] = useState("");
  const [nw, setNw] = useState(null);

  const calc = () => {
    const res = parseFloat(shareCap) + parseFloat(reserves) - parseFloat(losses);
    setNw(res.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Paid-up Share Capital" value={shareCap} onChange={e => setShareCap(e.target.value)} />
      <InputGroup label="Reserves & Surplus" value={reserves} onChange={e => setReserves(e.target.value)} />
      <InputGroup label="Accumulated Losses / Def Exp" value={losses} onChange={e => setLosses(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Net Worth</button>
      {nw && <ResultBox label="Net Worth" value={`₹${nw}`} />}
    </div>
  );
};

// --- 17. Shell Company Risk Index ---
export const ShellRisk = () => {
  const [turnover, setTurnover] = useState("");
  const [assets, setAssets] = useState("");
  const [risk, setRisk] = useState(null);

  const analyze = () => {
    const t = parseFloat(turnover);
    const a = parseFloat(assets);
    if (a === 0) return;
    
    // Low turnover relative to high assets is a flag
    const ratio = t / a;
    if (ratio < 0.05) setRisk("HIGH: Potential Shell Entity (Low Asset Turnover)");
    else setRisk("LOW: Active Business");
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Annual Turnover" value={turnover} onChange={e => setTurnover(e.target.value)} />
      <InputGroup label="Total Assets" value={assets} onChange={e => setAssets(e.target.value)} />
      <button onClick={analyze} className="w-full py-3 bg-black text-white font-bold rounded">Analyze Indicators</button>
      {risk && <ResultBox label="Risk Level" value={risk} isError={risk.includes("HIGH")} />}
    </div>
  );
};

// --- 18. Export Realization (FEMA) ---
export const ExportTracker = () => {
  const [date, setDate] = useState("");
  const [status, setStatus] = useState(null);

  const check = () => {
    const exportDate = new Date(date);
    const today = new Date();
    // 9 months limit
    const deadline = new Date(exportDate.setMonth(exportDate.getMonth() + 9));
    
    if (today > deadline) setStatus("OVERDUE: FEMA Violation Risk");
    else setStatus("Compliant (Within 9 Months)");
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Date of Export" type="date" value={date} onChange={e => setDate(e.target.value)} />
      <button onClick={check} className="w-full py-3 bg-black text-white font-bold rounded">Check Deadline</button>
      {status && <ResultBox label="Realization Status" value={status} isError={status.includes("OVERDUE")} />}
    </div>
  );
};

// --- 19. Partnership Dissolution ---
export const PartnershipCalc = () => {
  const [assets, setAssets] = useState("");
  const [liab, setLiab] = useState("");
  const [partners, setPartners] = useState("2");
  const [share, setShare] = useState(null);

  const calc = () => {
    const net = parseFloat(assets) - parseFloat(liab);
    const p = parseInt(partners);
    setShare((net / p).toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Realized Assets Value" value={assets} onChange={e => setAssets(e.target.value)} />
      <InputGroup label="External Liabilities" value={liab} onChange={e => setLiab(e.target.value)} />
      <InputGroup label="Number of Partners" type="number" value={partners} onChange={e => setPartners(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Settlement</button>
      {share && <ResultBox label="Per Partner Share" value={`₹${share}`} />}
    </div>
  );
};

// --- 20. Crypto Tax Calculator ---
export const CryptoTax = () => {
  const [profit, setProfit] = useState("");
  const [tax, setTax] = useState(null);

  const calc = () => {
    // Flat 30% + 4% Cess
    const t = parseFloat(profit) * 0.312;
    setTax(t.toFixed(2));
  };

  return (
    <div className="bg-white p-6 rounded-xl border border-gray-200">
      <InputGroup label="Net Profit from VDA (Crypto)" value={profit} onChange={e => setProfit(e.target.value)} />
      <button onClick={calc} className="w-full py-3 bg-black text-white font-bold rounded">Calculate Sec 115BBH Tax</button>
      {tax && <ResultBox label="Tax Liability" value={`₹${tax}`} />}
    </div>
  );
};