import { useState } from 'react'
import './App.css'
import {invoke} from "@tauri-apps/api/core";
import {open} from "@tauri-apps/plugin-dialog";

function App() {
  const [count, setCount] = useState(0)

  async function handleSelectFolder() {
    try{
      const selected = await open({
        directory: true,
        multiple: true,
        recursive: true
      });
      console.log("Directory selected");

      if(selected) {
        const paths = Array.isArray(selected) ? selected : [selected];

        for (const p of paths) {
          await invoke("ingest_path", {path: p});
          console.log("Path sent to backend: ", p);
        }
      }
    } catch (err) {
      alert("Core is not connected");
      console.log(err);
    }
  }


  return (
    <>
      <h1>Click to select a folder</h1>
      <div className="card">
        <button onClick={handleSelectFolder} style={{marginTop: "1rem"}}>
          Select a Folder
        </button>
      </div>
    </>
  )
}

export default App
