import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ControlCenterPanel } from "./ControlCenterPanel";
import "./index.css";

const apiBase =
  import.meta.env.VITE_CONTROL_CENTER_URL ??
  (typeof window !== "undefined" ? window.location.origin : "http://127.0.0.1:8080");

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <div className="app cc-desktop-app">
      <ControlCenterPanel apiBase={apiBase} />
    </div>
  </StrictMode>,
);
