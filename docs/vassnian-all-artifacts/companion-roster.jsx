import { useState } from "react";

const COMPANIONS = [
  // ACT 0
  { id: 1, act: 0, class: "Knight", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 2, act: 0, class: "Warrior", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 3, act: 0, class: "Paladin", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 4, act: 0, class: "Mage", num: 1, race: "Elf", gender: "Male", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 5, act: 0, class: "Priest", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  // ACT 1
  { id: 6, act: 1, class: "Warrior", num: 2, race: "Human", gender: "Male", worldSkill: "Swimming (LOCKED)", concept: "One-Armed Warrior — former champion, lost his arm. Cannot use 2H weapons or Swimming until prosthetic arm quest (dwarven fortress forge).", handicap: "One-Armed — no 2H weapons, no Swimming until prosthetic" },
  { id: 7, act: 1, class: "Knight", num: 2, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 8, act: 1, class: "Priest", num: 2, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 9, act: 1, class: "Bard", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 10, act: 1, class: "Ranger", num: 1, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  { id: 11, act: 1, class: "Rogue", num: 1, race: "Gnome", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
  // ACT 2
  { id: 12, act: 2, class: "Mage", num: 2, race: "Crystal Deepkin", gender: "TBD", worldSkill: "TBD", concept: "Battle Mage — energy sword (INT-scaling melee). Crystal-based sorcery. Can fight front or back row.", handicap: "Surface Disorientation — above-ground is overwhelming" },
  { id: 13, act: 2, class: "Paladin", num: 2, race: "TBD", gender: "TBD", worldSkill: "TBD", concept: "TBD", handicap: null },
];

const WORLD_SKILLS = [
  "Acrobatics", "Lockpicking", "Swimming", "Arcane Knowledge", "History",
  "Religion", "Nature", "Perception", "Search", "Tracking", "Stealth",
  "Persuasion", "Disguise"
];

const ACT_COLORS = { 0: "#6abf6a", 1: "#bfa86a", 2: "#6a6abf" };

export default function CompanionRoster() {
  const assigned = COMPANIONS.filter(c => c.worldSkill !== "TBD" && !c.worldSkill.includes("LOCKED")).map(c => c.worldSkill);
  const unassigned = WORLD_SKILLS.filter(s => !assigned.includes(s) && s !== "Swimming");

  return (
    <div style={{
      minHeight: "100vh", background: "#0a0a0f", color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        <div style={{ textAlign: "center", marginBottom: 16, borderBottom: "1px solid #2a2520", paddingBottom: 12 }}>
          <h1 style={{ fontSize: 20, color: "#d4a574", fontWeight: 400, letterSpacing: 3, textTransform: "uppercase", margin: 0 }}>
            Companion Roster — 13 Companions
          </h1>
          <p style={{ color: "#6a6058", fontSize: 10, marginTop: 4 }}>
            5 classes get 2 (Knight, Warrior, Paladin, Mage, Priest) • 3 classes get 1 (Bard, Ranger, Rogue)
          </p>
        </div>

        {[0, 1, 2].map(act => (
          <div key={act} style={{ marginBottom: 16 }}>
            <div style={{ fontSize: 12, color: ACT_COLORS[act], fontWeight: 600, marginBottom: 6, borderBottom: `1px solid ${ACT_COLORS[act]}33`, paddingBottom: 4 }}>
              Act {act} — {act === 0 ? "King's Guild (5)" : act === 1 ? "Surface World (6)" : "Shadow World (2)"}
            </div>
            {COMPANIONS.filter(c => c.act === act).map(c => (
              <div key={c.id} style={{
                background: "#0c0c10", border: "1px solid #1a1815",
                borderLeft: `3px solid ${ACT_COLORS[act]}`,
                borderRadius: 4, padding: "10px 14px", marginBottom: 4,
              }}>
                <div style={{ display: "flex", alignItems: "center", gap: 8, flexWrap: "wrap" }}>
                  <span style={{ fontSize: 12, color: "#d4c4b0", fontWeight: 600, minWidth: 80 }}>
                    {c.class} #{c.num}
                  </span>
                  <span style={{ fontSize: 10, color: c.race === "TBD" ? "#3a3530" : "#8a8078" }}>
                    {c.race}
                  </span>
                  <span style={{ fontSize: 10, color: c.gender === "TBD" ? "#3a3530" : "#6a6058" }}>
                    {c.gender}
                  </span>
                  <span style={{ fontSize: 9, color: c.worldSkill === "TBD" ? "#3a3530" : "#8a6abf", background: "#0a0a14", padding: "2px 6px", borderRadius: 3 }}>
                    Skill: {c.worldSkill}
                  </span>
                  {c.handicap && (
                    <span style={{ fontSize: 9, color: "#bf6a6a", background: "#1a0808", padding: "2px 6px", borderRadius: 3 }}>
                      {c.handicap}
                    </span>
                  )}
                </div>
                {c.concept !== "TBD" && (
                  <div style={{ fontSize: 11, color: "#a09888", marginTop: 6, lineHeight: 1.6 }}>
                    {c.concept}
                  </div>
                )}
              </div>
            ))}
          </div>
        ))}

        {/* Unassigned Skills */}
        <div style={{ marginTop: 16, padding: 12, background: "#0f0e0c", border: "1px solid #1a1815", borderRadius: 4 }}>
          <div style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 2, marginBottom: 6 }}>
            World Skills to Assign ({unassigned.length} remaining)
          </div>
          <div style={{ fontSize: 11, color: "#8a6abf", lineHeight: 1.8 }}>
            {unassigned.join(" • ")}
          </div>
          <div style={{ fontSize: 10, color: "#4a8a4a", marginTop: 4 }}>
            Swimming → One-Armed Warrior (locked until prosthetic)
          </div>
        </div>
      </div>
    </div>
  );
}
