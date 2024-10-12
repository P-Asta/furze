import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { runtimeDir } from '@tauri-apps/api/path';

import "./App.css";



function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setName(await invoke("zip_path"))
  }

  useEffect(() => {
    greet();
  }, []);
  return (
    <>{name}</>
  );
}

export default App;