import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
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
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>FlowForge Dashboard</h1>
      <div className="card">
        <button onClick={handleReload} style={{marginTop: "1rem"}}>
          Reload Logs
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
