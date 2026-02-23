import { useState } from "react";

/*
  UNIQUE NPCs TRACKER
  Every named character in the game beyond companions and traitor parties.
  Kings, merchants, quest givers, recurring characters, village NPCs.
  Fill as stories are written.
*/

const NPCS = [
  // SEED — known NPCs from lore. Expand during story writing.
  {
    name: "The King of Caelmund",
    realName: "TBD",
    race: "Human",
    role: "Ruler",
    acts: ["0", "1", "1.5", "3", "4"],
    personality: "Desperate but dignified. Summoned the Godsents as a last resort. Aging, burdened, but still commands respect. Trusts too easily — which is how the traitor party gained power.",
    quests: ["The Awakening (Act 0.1)", "Traitor foreshadow quests", "Act 3 — may be under traitor influence"],
    relationships: "Trusts the traitor leader. Skeptical of player in Act 3 when player returns making accusations.",
    notes: "Central to Act 3 political drama. The traitor's grip on him determines how hard the exposure is.",
  },
  {
    name: "Guild Master",
    realName: "TBD",
    race: "TBD",
    role: "King's Guild Leader",
    acts: ["0", "1"],
    personality: "TBD — manages the Godsent parties, assigns missions, practical and no-nonsense.",
    quests: ["Proving Ground (Act 0.2)", "Mission briefings throughout Act 1"],
    relationships: "Direct superior to player. May suspect traitor party but lacks evidence.",
    notes: "Quest hub NPC for Acts 0-1. Possible ally in Act 3 if still alive/free.",
  },
  {
    name: "Shadow Hero",
    realName: "TBD",
    race: "TBD",
    role: "Mysterious Rogue / Continuation Quest NPC",
    acts: ["1", "2", "3", "4"],
    personality: "Enigmatic, appears and disappears. Seems to know more than they should. Helps the player at key moments but has their own agenda.",
    quests: ["Continuation quest chain across all acts — meet in Act 1, reappears in Act 2-4"],
    relationships: "Unknown connection to Gorath's seal. May be a previous Godsent from an earlier summoning.",
    notes: "Major continuation quest NPC. Their true identity is a late-game revelation.",
  },
];

const ROLE_COLORS = {
  "Ruler": "#d4a574",
  "King's Guild Leader": "#bfa86a",
  "Mysterious Rogue / Continuation Quest NPC": "#8a6abf",
  "Merchant": "#6abf6a",
  "Quest Giver": "#6a8abf",
  "Village NPC": "#8a8078",
  "Military": "#bf8a6a",
  "Religious": "#bfbf6a",
  "Scholar": "#6abfbf",
};

export default function UniqueNPCs() {
  const [filterAct, setFilterAct] = useState("all");
  const filtered = filterAct === "all" ? NPCS : NPCS.filter(n => n.acts.includes(filterAct));

  return (
    <div style={{
      minHeight: "100vh", background: "#0a0a0f", color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        <div style={{ textAlign: "center", marginBottom: 20, borderBottom: "1px solid #2a2520", paddingBottom: 16 }}>
          <h1 style={{ fontSize: 22, color: "#6abf6a", fontWeight: 400, letterSpacing: 4, textTransform: "uppercase", margin: 0 }}>
            \u{1F464} Unique NPCs Tracker \u{1F464}
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            {NPCS.length} NPCs tracked \u2022 Filled during story writing \u2022 Companions and traitors tracked separately
          </p>
        </div>

        <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginBottom: 16 }}>
          {["all", "0", "1", "1.5", "2", "3", "4"].map(a => (
            <button key={a} onClick={() => setFilterAct(a)} style={{
              background: filterAct === a ? "#1a1815" : "transparent",
              border: `1px solid ${filterAct === a ? "#2a2520" : "#1a1815"}`,
              color: filterAct === a ? "#d4a574" : "#4a4238",
              padding: "5px 12px", borderRadius: 3, cursor: "pointer",
              fontSize: 10, fontFamily: "inherit",
            }}>{a === "all" ? "All Acts" : `Act ${a}`}</button>
          ))}
        </div>

        {filtered.map((npc, i) => (
          <div key={i} style={{
            background: "#0c0c10", border: "1px solid #1a1815",
            borderLeft: "3px solid #6abf6a",
            borderRadius: 4, padding: "12px 16px", marginBottom: 8,
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6, flexWrap: "wrap" }}>
              <span style={{ fontSize: 13, color: "#d4c4b0", fontWeight: 600 }}>{npc.name}</span>
              {npc.realName !== "TBD" && <span style={{ fontSize: 10, color: "#6a6058" }}>({npc.realName})</span>}
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, background: "#08080c", color: "#8a8078" }}>{npc.race}</span>
              <span style={{ fontSize: 9, padding: "2px 6px", borderRadius: 3, background: "#08080c", color: "#6abf6a" }}>{npc.role}</span>
              <span style={{ fontSize: 9, color: "#5a5248" }}>Acts: {npc.acts.join(", ")}</span>
            </div>
            <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.6 }}>
              <div><span style={{ color: "#6abf6a", fontWeight: 600 }}>Personality: </span>{npc.personality}</div>
              <div><span style={{ color: "#bfa86a", fontWeight: 600 }}>Appears in: </span>{npc.quests.join(", ")}</div>
              <div><span style={{ color: "#6a6abf", fontWeight: 600 }}>Relationships: </span>{npc.relationships}</div>
              {npc.notes && <div style={{ fontStyle: "italic", color: "#6a6058", marginTop: 4 }}>{npc.notes}</div>}
            </div>
          </div>
        ))}

        <div style={{ textAlign: "center", marginTop: 24, fontSize: 10, color: "#3a3530" }}>
          Add NPCs as stories create them \u2022 Companions in traitor-parties.jsx \u2022 Player companions in companion roster
        </div>
      </div>
    </div>
  );
}
