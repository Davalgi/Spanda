/**
 * Example Control Center panel bundle for spanda-plugin-control-center-panel.
 *
 * Loaded inside the sandboxed iframe host (see ControlCenterPluginPanel).
 * Expects `#plugin-mount` with data-plugin / data-component attributes.
 */
(function () {
  var mount = document.getElementById("plugin-mount");
  if (!mount) {
    mount = document.createElement("div");
    mount.id = "plugin-mount";
    document.body.appendChild(mount);
  }
  var plugin = mount.dataset.plugin || "spanda-plugin-control-center-panel";
  var component = mount.dataset.component || "FleetOverviewPanel";
  mount.innerHTML =
    "<h4 style='margin:0 0 0.5rem'>" +
    component +
    "</h4>" +
    "<p style='margin:0;opacity:0.85'>Example Control Center panel from <code>" +
    plugin +
    "</code>. Running inside a sandboxed iframe.</p>";
})();
