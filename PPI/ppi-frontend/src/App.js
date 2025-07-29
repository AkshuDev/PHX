import React from 'react';

import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

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
      <Switch>
        <Route path="/" exact component={LandingPage} />
        <Route path="/packages" component={PackagesPage} />
      </Switch>
    </Router>
  );
}

export default App;
