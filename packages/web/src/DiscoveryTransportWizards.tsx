import { useState } from "react";
import { CcSection } from "./controlCenterUi";

type Wizard = {
  id: string;
  title: string;
  summary: string;
  steps: string[];
  env: string[];
  cli: string;
};

const WIZARDS: Wizard[] = [
  {
    id: "opcua",
    title: "OPC-UA",
    summary: "Connect to industrial OPC-UA servers on the plant floor.",
    steps: [
      "Confirm the OPC-UA server endpoint (opc.tcp://host:4840).",
      "Set security policy to match the server (None, Basic256Sha256, …).",
      "Enable the opcua transport in your discovery scan.",
    ],
    env: ["SPANDA_OPCUA_ENDPOINT", "SPANDA_OPCUA_SECURITY_POLICY"],
    cli: "spanda discover --transport opcua --endpoint opc.tcp://127.0.0.1:4840",
  },
  {
    id: "modbus",
    title: "Modbus TCP",
    summary: "Probe Modbus TCP devices on subnet or static hosts.",
    steps: [
      "Identify unit IDs and holding-register maps from device docs.",
      "Allow TCP/502 from the Control Center host to targets.",
      "Run discovery with the modbus transport selected.",
    ],
    env: ["SPANDA_MODBUS_HOST", "SPANDA_MODBUS_PORT", "SPANDA_MODBUS_UNIT_ID"],
    cli: "spanda discover --transport modbus --host 192.168.1.50 --port 502",
  },
  {
    id: "mqtt",
    title: "MQTT",
    summary: "Attach to a broker and enumerate device topics.",
    steps: [
      "Point SPANDA_MQTT_BROKER at your broker URL.",
      "Configure credentials/TLS if the broker requires them.",
      "Select mqtt in the transport grid and run discovery.",
    ],
    env: ["SPANDA_MQTT_BROKER", "SPANDA_MQTT_USERNAME", "SPANDA_MQTT_TLS"],
    cli: "spanda discover --transport mqtt --broker mqtt://localhost:1883",
  },
  {
    id: "can",
    title: "CAN / SocketCAN",
    summary: "Discover devices on a Linux SocketCAN interface.",
    steps: [
      "Bring up can0 (or your interface) with correct bitrate.",
      "Install spanda-discovery-can when using the package backend.",
      "Select can transport and run discovery from a CAN-capable host.",
    ],
    env: ["SPANDA_CAN_INTERFACE", "SPANDA_CAN_BITRATE"],
    cli: "spanda discover --transport can --interface can0",
  },
];

type Props = {
  onSelectTransport: (transportId: string) => void;
};

export function DiscoveryTransportWizards({ onSelectTransport }: Props) {
  const [openId, setOpenId] = useState<string | null>("opcua");

  return (
    <CcSection
      title="Transport setup wizards"
      hint="Guided steps for industrial and field-bus transports. Selecting a wizard enables that transport in the scan grid."
    >
      <div className="cc-wizard-list">
        {WIZARDS.map((wizard) => {
          const open = openId === wizard.id;
          return (
            <article key={wizard.id} className={`cc-wizard-card${open ? " open" : ""}`}>
              <header className="cc-wizard-header">
                <button
                  type="button"
                  className="cc-wizard-toggle"
                  onClick={() => setOpenId(open ? null : wizard.id)}
                >
                  {wizard.title}
                </button>
                <button
                  type="button"
                  className="secondary"
                  onClick={() => onSelectTransport(wizard.id)}
                >
                  Use transport
                </button>
              </header>
              {open && (
                <div className="cc-wizard-body">
                  <p className="cc-section-hint">{wizard.summary}</p>
                  <ol className="cc-wizard-steps">
                    {wizard.steps.map((step) => (
                      <li key={step}>{step}</li>
                    ))}
                  </ol>
                  <p className="cc-section-hint">Environment variables</p>
                  <ul className="cc-env-list">
                    {wizard.env.map((name) => (
                      <li key={name}>
                        <code>{name}</code>
                      </li>
                    ))}
                  </ul>
                  <pre className="cc-cli-example">{wizard.cli}</pre>
                </div>
              )}
            </article>
          );
        })}
      </div>
    </CcSection>
  );
}
