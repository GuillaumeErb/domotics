import React, { useState, useEffect } from 'react';
import './App.css';
import { toggleLight, getAllLightsAsync, Light } from '../api/lightsApi';

const App = () => {

  const [lights, setLights] = useState<Light[]>([]);

  useEffect(() => {
    getAllLightsAsync().then(lights => {
      console.log("setting lights" + JSON.stringify(lights));
      setLights(lights)
    });
  }, []);

  console.log("light " + lights.length);

  return (
    <div className="App">
      <header className="App-header">
        <button
          className="button"
          onClick={() => {
            getAllLightsAsync({ refresh: true }).then(setLights)
          }}>Refresh</button>
        {
          lights.map(light => {
            console.log("looping through light");
            return <div
              className="button"
              onClick={() => toggleLight(light.id)}
              key={light.id}>{buttonContent(light)}</div>;
          }
          )
        }
      </header>
    </div >
  );
}

const buttonContent = (light: Light): JSX.Element => {
  return (
    <div>
      <div className="button-content-left">{light.name}</div>
      <div className="button-content-right"
        id={light.power ? "on-indicator" : "off-indicator"} />
    </div>
  );
}

export default App;
