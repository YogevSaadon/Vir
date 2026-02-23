import { useState } from "react";

/*
  UNIQUE ITEMS TRACKER
  Fill this as stories are written. Every named/special item gets an entry.
  Categories: Boss Drops, Quest Rewards, Companion Items, Hidden Treasures, Artifacts
*/

const ITEMS = [
  // EXAMPLE FORMAT — delete these and fill real items during story writing
  {
    name: "Warlord's Dark Shard",
    type: "Accessory",
    subtype: "Artifact Fragment",
    act: "1",
    source: "Bandit Warlord boss fight — choosing to claim the artifact",
    stats: "TBD — dark power boost with corruption cost",
    lore: "A fragment of the artifact that empowered the Bandit Warlord. Pulses with Gorath's leaking influence. Grants power at a price.",
    category: "boss_drop",
    tags_set: ["has_dark_shard"],
    tags_check: ["boss_bandit_warlord"],
  },
  {
    name: "Iron Prosthetic Arm",
    type: "Special",
    subtype: "Companion Equipment",
    act: "1-2",
    source: "One-Armed Warrior personal quest — forged at dwarven fortress",
    stats: "Unlocks: Swimming skill, 2-Handed weapon equip for the Warrior",
    lore: "Dwarven-forged replacement arm. Functional, heavy, beautifully crafted. The warrior can finally fight at full capacity.",
    category: "companion_quest",
    tags_set: ["warrior_has_prosthetic"],
    tags_check: ["companion_one_armed_warrior"],
  },
];

const CATEGORIES = [
  { id: "all", label: "All", color: "#d4a574" },
  { id: "boss_drop", label: "Boss Drops", color: "#bf6a6a" },
  { id: "quest_reward", label: "Quest Rewards", color: "#6abf6a" },
  { id: "companion_quest", label: "Companion Items", color: "#bfa86a" },
  { id: "hidden", label: "Hidden Treasures", color: "#6a6abf" },
  { id: "artifact", label: "Artifacts", color: "#8a6abf" },
  { id: "key_item", label: "Key Items", color: "#6abfbf" },
];

const ACT_COLORS = {
  "0": "#6abf6a", "1": "#bfa86a", "1-2": "#8abf8a",
  "2": "#6a6abf", "3": "#bf6a6a", "4": "#d4a574",
};

export default function UniqueItems() {
  const [filter, setFilter] = useState("all");
  const filtered = filter === "all" ? ITEMS : ITEMS.filter(i => i.category === filter);

  return (
    <div style={{
      minHeight: "100vh", background: "#0a0a0f", color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        <div style={{ textAlign: "center", marginBottom: 20, borderBottom: "1px solid #2a2520", paddingBottom: 16 }}>
          <h1 style={{ fontSize: 22, color: "#d4a574", fontWeight: 400, letterSpacing: 4, textTransform: "uppercase", margin: 0 }}>
            \u2694 Unique Items Tracker \u2694
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            {ITEMS.length} items tracked \u2022 Filled during story writing
          </p>
        </div>

        {/* Category Filter */}
        <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginBottom: 16 }}>
          {CATEGORIES.map(c => (
            <button key={c.id} onClick={() => setFilter(c.id)} style={{
              background: filter === c.id ? "#1a1815" : "transparent",
              border: `1px solid ${filter === c.id ? c.color + "44" : "#1a1815"}`,
              color: filter === c.id ? c.color : "#4a4238",
              padding: "5px 12px", borderRadius: 3, cursor: "pointer",
              fontSize: 10, fontFamily: "inherit",
            }}>{c.label}</button>
          ))}
        </div>

        {/* Items */}
        {filtered.map((item, i) => (
          <div key={i} style={{
            background: "#0c0c10", border: "1px solid #1a1815",
            borderLeft: `3px solid ${ACT_COLORS[item.act] || "#5a5248"}`,
            borderRadius: 4, padding: "12px 16px", marginBottom: 8,
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6, flexWrap: "wrap" }}>
              <span style={{ fontSize: 13, color: "#d4c4b0", fontWeight: 600 }}>{item.name}</span>
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, background: "#08080c", color: ACT_COLORS[item.act] || "#5a5248" }}>Act {item.act}</span>
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, background: "#08080c", color: "#6a6058" }}>{item.type} — {item.subtype}</span>
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, border: `1px solid ${CATEGORIES.find(c => c.id === item.category)?.color || "#5a5248"}33`, color: CATEGORIES.find(c => c.id === item.category)?.color || "#5a5248" }}>
                {item.category.replace(/_/g, " ")}
              </span>
            </div>
            <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.6, marginBottom: 4 }}>
              <span style={{ color: "#6abf6a", fontWeight: 600 }}>Source: </span>{item.source}
            </div>
            <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.6, marginBottom: 4 }}>
              <span style={{ color: "#bfa86a", fontWeight: 600 }}>Stats: </span>{item.stats}
            </div>
            <div style={{ fontSize: 11, color: "#6a6058", lineHeight: 1.6, fontStyle: "italic" }}>{item.lore}</div>
            {(item.tags_set?.length > 0 || item.tags_check?.length > 0) && (
              <div style={{ fontSize: 10, color: "#5a5248", marginTop: 4, display: "flex", gap: 8 }}>
                {item.tags_set?.length > 0 && <span>Sets: {item.tags_set.join(", ")}</span>}
                {item.tags_check?.length > 0 && <span>Requires: {item.tags_check.join(", ")}</span>}
              </div>
            )}
          </div>
        ))}

        <div style={{ textAlign: "center", marginTop: 24, fontSize: 10, color: "#3a3530" }}>
          Add items here as stories introduce them \u2022 Each item needs: name, type, act, source, stats, lore, tags
        </div>
      </div>
    </div>
  );
}
