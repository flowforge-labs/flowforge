import { useState } from 'react'
import './App.css'
import { reloadLogs } from './api/reload'

function App() {
  const [count, setCount] = useState(0)

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
        <button onClick={handleReload} style={{marginTop: "1rem"}}>
          Reload Logs
        </button>
      </div>
    </>
  )
}

export default App
