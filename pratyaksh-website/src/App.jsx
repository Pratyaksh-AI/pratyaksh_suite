import React, { useState, useEffect } from 'react';
import { Navbar } from './components/Navbar';
import { Footer } from './components/Footer';
import { Home } from './pages/Home';
import { DownloadScreen } from './pages/Download';
import { GenericPage } from './pages/GenericPage';
import { ToolsPage } from './pages/Tools'; // Import the new Tools Page
import { THEME } from './data/constants';

export default function App() {
  const [page, setPage] = useState('home');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  useEffect(() => { window.scrollTo(0, 0); }, [page]);

  return (
    <div className={`font-sans antialiased ${THEME.bg} ${THEME.textMain} selection:bg-[#4FF978] selection:text-black`}>
      <Navbar setPage={setPage} mobileMenuOpen={mobileMenuOpen} setMobileMenuOpen={setMobileMenuOpen} />
      
      {/* Route: Home */}
      {page === 'home' && <Home setPage={setPage} />}
      
      {/* Route: Download */}
      {page === 'download' && <DownloadScreen setPage={setPage} />}
      
      {/* Route: Tools Catalog (New) */}
      {page === 'tools' && <ToolsPage setPage={setPage} />}
      
      {/* Route: Generic Pages (About, Legal, Specific Calculators) */}
      {!['home', 'download', 'tools'].includes(page) && <GenericPage pageKey={page} setPage={setPage} />}

      <Footer setPage={setPage} />
    </div>
  );
}