import { useState } from "react";

/*
  UNIQUE ENEMIES TRACKER
  Named/special enemies beyond the basic bestiary.
  Mini-bosses, quest-specific encounters, named villains, rare creatures.
  Fill as stories are written.
*/

const ENEMIES = [
  // EXAMPLES — replace with real entries during story writing
  {
    name: "The Goblin King",
    family: "Goblin",
    rank: "Act Boss",
    act: "1",
    source: "Goblin King boss path — Act 1.3 'The Goblin Throne'",
    hp: "80",
    abilities: "Formation commands (repositions guards), terrain exploitation, adaptive tactics. Multi-phase: guards first, then personal combat.",
    behavior: "Tactical — commands army, exploits terrain, adapts to player strategy mid-fight.",
    lore: "United the goblin tribes through intelligence, not brute force. Wears a crude iron crown. Fights with terrifying cunning.",
    drops: "TBD",
  },
  {
    name: "The Corrupted Beast (Phase 1)",
    family: "Corrupted",
    rank: "Scripted Encounter",
    act: "1",
    source: "Corrupted Beast path — Act 1.1 'The Hunter's Mark'",
    hp: "Unkillable (scripted escape)",
    abilities: "Massive physical damage, corruption aura, tracks Godsents specifically.",
    behavior: "Aggressive pursuit. Player CANNOT win — scripted to flee.",
    lore: "Once a normal creature, twisted by Gorath's leaking influence into a hunter of Godsents. Grows stronger across 3 encounters.",
    drops: "None (Act 1.1 version)",
  },
];

const RANK_COLORS = {
  "Act Boss": "#d4a574",
  "Mini-Boss": "#bf8a6a",
  "Elite": "#8a6abf",
  "Named": "#6abf6a",
  "Scripted Encounter": "#bf6a8a",
  "Rare": "#6a8abf",
};

export default function UniqueEnemies() {
  const [filterAct, setFilterAct] = useState("all");
  const filtered = filterAct === "all" ? ENEMIES : ENEMIES.filter(e => e.act === filterAct);

  return (
    <div style={{
      minHeight: "100vh", background: "#0a0a0f", color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        <div style={{ textAlign: "center", marginBottom: 20, borderBottom: "1px solid #2a2520", paddingBottom: 16 }}>
          <h1 style={{ fontSize: 22, color: "#bf6a6a", fontWeight: 400, letterSpacing: 4, textTransform: "uppercase", margin: 0 }}>
            \u2620 Unique Enemies Tracker \u2620
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            {ENEMIES.length} unique enemies \u2022 Filled during story writing
          </p>
        </div>

        <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginBottom: 16 }}>
          {["all", "1", "2", "3", "4"].map(a => (
            <button key={a} onClick={() => setFilterAct(a)} style={{
              background: filterAct === a ? "#1a1815" : "transparent",
              border: `1px solid ${filterAct === a ? "#2a2520" : "#1a1815"}`,
              color: filterAct === a ? "#d4a574" : "#4a4238",
              padding: "5px 12px", borderRadius: 3, cursor: "pointer",
              fontSize: 10, fontFamily: "inherit",
            }}>{a === "all" ? "All Acts" : `Act ${a}`}</button>
          ))}
        </div>

        {filtered.map((e, i) => (
          <div key={i} style={{
            background: "#0c0c10", border: "1px solid #1a1815",
            borderLeft: `3px solid ${RANK_COLORS[e.rank] || "#5a5248"}`,
            borderRadius: 4, padding: "12px 16px", marginBottom: 8,
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6, flexWrap: "wrap" }}>
              <span style={{ fontSize: 13, color: "#d4c4b0", fontWeight: 600 }}>{e.name}</span>
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, background: RANK_COLORS[e.rank] + "22", color: RANK_COLORS[e.rank], border: `1px solid ${RANK_COLORS[e.rank]}44` }}>{e.rank}</span>
              <span style={{ fontSize: 9, color: "#6a6058" }}>Act {e.act} — {e.family}</span>
            </div>
            <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.6 }}>
              <div><span style={{ color: "#bf6a6a", fontWeight: 600 }}>HP: </span>{e.hp}</div>
              <div><span style={{ color: "#d4a574", fontWeight: 600 }}>Abilities: </span>{e.abilities}</div>
              <div><span style={{ color: "#6abf6a", fontWeight: 600 }}>Behavior: </span>{e.behavior}</div>
              <div><span style={{ color: "#6a6abf", fontWeight: 600 }}>Source: </span>{e.source}</div>
              <div style={{ fontStyle: "italic", color: "#6a6058", marginTop: 4 }}>{e.lore}</div>
            </div>
          </div>
        ))}

        <div style={{ textAlign: "center", marginTop: 24, fontSize: 10, color: "#3a3530" }}>
          Add enemies as stories create them \u2022 Basic bestiary enemies are in bestiary.jsx
        </div>
      </div>
    </div>
  );
}
