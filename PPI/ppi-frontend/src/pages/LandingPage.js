import React from 'react';
import './LandingPage.css';

const LandingPage = () => {
    return (
        <div className="landing-container">
            <div className="landing-content">
                <img src="../assets/logo.png" alt="PPI Logo" className="landing-logo"/>
                <h1 className="landing-title">Welcome to Pheonix Package Index</h1>
                <p className="landning-desc">
                    The fastest method to now manage your Packages anywhere anytime (with a computer).
                </p>
                <a href="/packages" className="get-started-btn">
                    Get Started
                </a>
            </div>
        </div>
    );
};