import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import BoxBtn from "./components/BoxBtn";
import IndexPage from "./pages/Index";
import ExtractPage from "./pages/Extract";
import "./App.css";



function App() {
  const [appPath, setAppPath] = useState("");
  const [os, setOs] = useState("");
  

  
  async function setup() {
    setOs(await invoke("get_os"));
    setAppPath(await invoke("select_path"))
  }

  useEffect(() => {
    setup();
  }, []);
  return (
    <>
      {appPath}
      {(!appPath)?
      <IndexPage/>:
        <ExtractPage/>
      }
    </>
  );
}

export default App;
