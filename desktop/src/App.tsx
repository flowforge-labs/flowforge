import { useState } from 'react'
import './App.css'
import { reloadLogs } from './api/reload'
import {invoke} from "@tauri-apps/api/core";

function App() {
  const [count, setCount] = useState(0)

  async function testCore() {
    try{
      const result = await invoke<string>("core_ping");
      console.log(result);
      alert("Core connected!");
    } catch (err) {
      alert("Core is not connected");
    }
  }
  async function handleReload() {
    try {
      const result = await reloadLogs();
      console.log("Reload result: ", result);
      alert("Reload successful");
    } catch (err) {
      console.error(err);
      alert("Reload failed.");
    }
  }

  return (
    <>
      <h1>FlowForge Dashboard</h1>
      <div className="card">
        <button onClick={testCore} style={{marginTop: "1rem"}}>
          Reload Logs
        </button>
      </div>
    </>
  )
}

export default App
