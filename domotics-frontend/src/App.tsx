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
      <div className="sidebar">
        <a className="active" href="#home">Lights</a>
        <a href="#empty">Empty</a>
      </div>
      <header className="App-header">
        <button onClick={() => {
          getAllLightsAsync({ refresh: true }).then(lights =>
            setLights(lights))
        }}>Refresh</button>
        {
          lights.map(light => {
            console.log("looping through light");
            return <button onClick={() => toggleLight(light.id)} key={light.id}>Toggle</button>;
          }
          )
        }
      </header>
    </div >
  );
}

export default App;
