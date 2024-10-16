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
      <header>
        <nav>
          <div id="close"></div>
          <div id="hide"></div>
          <div id="fullscreen"></div>
        </nav>
      </header>
      <main>
        <article>
          <BoxBtn name={"zip"} />
          <BoxBtn name={"7zip"} />
          <BoxBtn name={"tar"} />
        </article>

        <article>
          <BoxBtn name={"tar.gz"} />
          <BoxBtn name={"tar.xz"} />
          <BoxBtn name={"tar.xyz"} />
        </article>
      </main>
    </>
  );
}