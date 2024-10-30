import { getCurrentWindow } from '@tauri-apps/api/window';
import BoxBtn from "../components/BoxBtn";
import { useEffect } from 'react';

const appWindow = getCurrentWindow();

export default () => {
  useEffect(() => {
    document.querySelector("header").addEventListener('mousedown', (e) => {
      if (e.target.id === "close" || e.target.id === "hide" || e.target.id === "fullscreen") {
        return;
      }
      if (e.buttons === 1) {
        e.detail === 2
          ? appWindow.toggleMaximize()
          : appWindow.startDragging();
      }
    });

    document.getElementById("close").addEventListener("click", () => {appWindow.close();});
    document.getElementById("hide").addEventListener("click", () => appWindow.hide());
  }, []);
  return (
    <>
      <main>
        extract...
      </main>
    </>
  );
}