import React, { useState, useEffect } from 'react';
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
        <button
          className="button"
          onClick={() => {
            getAllLightsAsync({ refresh: true }).then(lights =>
              setLights(lights))
          }}>Refresh</button>
        {
          lights.map(light => {
            console.log("looping through light");
            return <button
              className="button"
              onClick={() => toggleLight(light.id)}
              key={light.id}>{light.name}</button>;
          }
          )
        }
      </header>
    </div >
  );
}

export default App;
