import { useMemo, useState } from "react";
import {
  CONTROL_CENTER_NAV_GROUPS,
  tabLabel,
  type NavGroup,
} from "./controlCenterNavConfig";
import { type ControlCenterTab } from "./controlCenterRbac";

type PluginPanel = {
  plugin: string;
  id: string;
  title: string;
};

type Props = {
  activeTab: ControlCenterTab | null;
  pluginTab: string | null;
  pluginPanels: PluginPanel[];
  isTabAllowed: (tab: ControlCenterTab) => boolean;
  onSelectTab: (tab: ControlCenterTab) => void;
  onSelectPlugin: (key: string) => void;
};

export function ControlCenterNav({
  activeTab,
  pluginTab,
  pluginPanels,
  isTabAllowed,
  onSelectTab,
  onSelectPlugin,
}: Props) {
  const [query, setQuery] = useState("");
  const [collapsed, setCollapsed] = useState<Record<string, boolean>>({});

  const normalizedQuery = query.trim().toLowerCase();

  const visibleGroups = useMemo(() => {
    const groups: NavGroup[] = [];
    for (const group of CONTROL_CENTER_NAV_GROUPS) {
      const tabs = group.tabs.filter((tab) => {
        if (!isTabAllowed(tab)) return false;
        if (!normalizedQuery) return true;
        const label = tabLabel(tab).toLowerCase();
        return label.includes(normalizedQuery) || tab.includes(normalizedQuery);
      });
      if (tabs.length > 0) {
        groups.push({ ...group, tabs });
      }
    }
    return groups;
  }, [isTabAllowed, normalizedQuery]);

  const visiblePlugins = useMemo(() => {
    if (!normalizedQuery) return pluginPanels;
    return pluginPanels.filter((panel) =>
      panel.title.toLowerCase().includes(normalizedQuery),
    );
  }, [normalizedQuery, pluginPanels]);

  const toggleGroup = (groupId: string) => {
    setCollapsed((prev) => ({ ...prev, [groupId]: !prev[groupId] }));
  };

  return (
    <nav className="cc-nav" aria-label="Control Center sections">
      <div className="cc-nav-search">
        <input
          type="search"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Find a view…"
          aria-label="Filter views"
        />
      </div>

      <div className="cc-nav-groups">
        {visibleGroups.map((group) => {
          const isOpen = normalizedQuery ? true : !collapsed[group.id];
          const groupHasActive =
            !pluginTab && activeTab != null && group.tabs.includes(activeTab);
          return (
            <section key={group.id} className="cc-nav-group">
              <button
                type="button"
                className={`cc-nav-group-toggle${groupHasActive ? " has-active" : ""}`}
                onClick={() => toggleGroup(group.id)}
                aria-expanded={isOpen ? "true" : "false"}
              >
                <span>{group.label}</span>
                <span className="cc-nav-chevron" aria-hidden>
                  {isOpen ? "▾" : "▸"}
                </span>
              </button>
              {isOpen && (
                <ul className="cc-nav-list">
                  {group.tabs.map((tab) => (
                    <li key={tab}>
                      <button
                        type="button"
                        className={
                          activeTab === tab && !pluginTab ? "cc-nav-item active" : "cc-nav-item"
                        }
                        onClick={() => onSelectTab(tab)}
                      >
                        {tabLabel(tab)}
                      </button>
                    </li>
                  ))}
                </ul>
              )}
            </section>
          );
        })}

        {visiblePlugins.length > 0 && (
          <section className="cc-nav-group">
            <div className="cc-nav-group-label">Plugins</div>
            <ul className="cc-nav-list">
              {visiblePlugins.map((panel) => {
                const key = `${panel.plugin}:${panel.id}`;
                return (
                  <li key={key}>
                    <button
                      type="button"
                      className={pluginTab === key ? "cc-nav-item active" : "cc-nav-item"}
                      onClick={() => onSelectPlugin(key)}
                    >
                      {panel.title}
                    </button>
                  </li>
                );
              })}
            </ul>
          </section>
        )}
      </div>
    </nav>
  );
}
