import React, { useEffect, useState } from 'react';
import axios from 'axios';

const PackagesPage = () => {
  const [packages, setPackages] = useState([]);

  useEffect(() => {
    axios.get('https://') // Update this with your backend URL
      .then(response => {
        setPackages(response.data);
      })
      .catch(error => {
        console.error('There was an error fetching packages!', error);
      });
  }, []);

  return (
    <div className="packages-container">
      <h2>Available Packages</h2>
      <ul>
        {packages.map((pkg, index) => (
          <li key={index}>{pkg.name} - {pkg.version}</li>
        ))}
      </ul>
    </div>
  );
};

export default PackagesPage;
