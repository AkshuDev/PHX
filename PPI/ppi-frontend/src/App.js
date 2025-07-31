import React from 'react';

import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import logo from './logo.svg';
import './App.css';

import LandingPage from './pages/LandingPage';
import PackagesPage from './pages/PackagesPage';

function App() {
  // return (
  //   <div className="App">
  //     <LandingPage />
  //   </div>
  // );

  return (
    <Router>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/packages" element={<PackagesPage />} />
      </Routes>
    </Router>
  );
}

export default App;
