import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import { toggleLight, getAllLightsAsync, Light } from './api/lightsApi';

function App() {

  const [lights, setLights] = useState<Light[]>([]);

  useEffect(() => {
    getAllLightsAsync().then(lights => {
      console.log("setting lights" + JSON.stringify(lights));
      setLights(lights)
    });
  }, []);

  console.log("light" + lights.length);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        {
          lights.map(light => {
            console.log("looping through light");
            return <button onClick={() => toggleLight(light.id)} key={light.id}>Toggle</button>;
          }
          )
        }
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div >
  );
}

export default App;
