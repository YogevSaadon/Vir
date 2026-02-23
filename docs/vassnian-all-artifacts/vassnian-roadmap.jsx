import { useState } from "react";

const PHASES = [
  {
    id: "done",
    label: "COMPLETED",
    color: "#4a8a4a",
    icon: "\u2705",
    items: [
      {
        name: "Lore Bible v3",
        detail: "Kingdom of Caelmund, 5 gods (Auron, Maetha, Luminar, Gorath, Umbrath), races, Shadow World, Deepkin, Godsent summoning, seal mythology. LOCKED.",
        artifact: "vassnian-lore-v3.md",
      },
      {
        name: "Core Systems",
        detail: "3 stats (STR/DEX/INT range 1-20), 8 classes, 6 actives + 4 passives per class, cross-class Godsent learning, formation (2 front + 2 back), mana (no regen in combat), no crits, no injury system. Skill gaps: 8 actives + 4 passives remain across 6 classes.",
        artifact: "vassnian-master-session.md",
      },
      {
        name: "Numbers & Formulas",
        detail: "HP = 6 + STR\u00d7Level. Mana = INT + Level. ATB = 10 + DEX. Armor = LoL diminishing returns. Equipment ranges per act. Enemy power bands per sub-act. Sample combat validated. \u26A0\uFE0F Boss count says 16, needs update to 13.",
        artifact: "vassnian-numbers.md",
        needsFix: true,
      },
      {
        name: "World Skills (13)",
        detail: "Acrobatics, Lockpicking, Swimming, Arcane Knowledge, History, Religion, Nature, Perception, Search, Tracking, Stealth, Persuasion, Disguise. Binary (have it or don't). 1 per companion.",
        artifact: "vassnian-world-skills.md",
      },
      {
        name: "Quest System Template",
        detail: "5 quest types (Main, Recruitment, Companion, Continuation, One-shot) with priority system. 10 tag types for conditional logic. Dynamic array filling per sub-act. Displacement rules.",
        artifact: "quest-system.jsx",
      },
      {
        name: "Act 1 Bestiary",
        detail: "53 enemies across 19 families and 5 ranks (Fodder/Common/Tough/Elite/Mini-Boss). Tier-based stats (HP/DMG/Armor/Speed 0-6). Full stat tables for sub-acts 1.1-1.3.",
        artifact: "bestiary.jsx",
      },
      {
        name: "13 Boss Designs",
        detail: "4 Act 1 (Goblin King, Bandit Warlord, Corrupted Beast, Cult High Priest), 4 Act 2 (Shadow Dragon, Deepkin Tyrant, Exiled Archmage, Ancient Construct), 4 Act 3 Traitor Parties, Gorath. 1 boss per act randomly selected per run = 4 fights per playthrough.",
        artifact: "main-quest-table.jsx",
      },
      {
        name: "4 Traitor Party Compositions",
        detail: "Iron Vanguard (Commander), Shadow Circle (Prodigy), Gilded Saints (Showman), Hollow Mercy (Shepherd). Each: Godsent leader + 3 members with combat roles, personalities, Act 0-3 arc, betrayal style.",
        artifact: "traitor-parties.jsx",
      },
      {
        name: "Main Quest Table",
        detail: "Every main quest from Act 0 through Gorath. Universal quests + boss-specific variants + traitor foreshadow/bonding/betrayal. ~19 quests per run, 64 possible boss combinations (4\u00d74\u00d74).",
        artifact: "main-quest-table.jsx",
      },
    ],
  },
  {
    id: "fix",
    label: "NEEDS FIXING",
    color: "#bf8a4a",
    icon: "\u26A0\uFE0F",
    items: [
      {
        name: "Numbers Doc — Boss Count & Act Structure",
        detail: "Says 16 bosses, should say 13. Doesn't include Act 1.5 (betrayal) or Act 2.5 (emergence). Level ranges need adjustment for the new act flow: Act 0 (Lv1) \u2192 Act 1 (Lv1-5) \u2192 Act 1.5 \u2192 Act 2 (Lv6-10) \u2192 Act 2.5 \u2192 Act 3 (Lv11-15) \u2192 Act 4 (Lv16-20).",
        artifact: "vassnian-numbers.md",
        priority: "medium",
      },
      {
        name: "Companion Roster — Full Rebuild",
        detail: "Current draft says 16 companions with 2 per class and Act 3 recruits. Should be 13 companions: 5 classes get 2 (Knight, Warrior, Paladin, Mage, Priest), 3 get 1 (Bard, Ranger, Rogue). Distribution: 5 Act 0, 6 Act 1, 2 Act 2, 0 Act 3. Need: names, races, personalities, world skills, handicaps, recruitment concepts.",
        artifact: "vassnian-companions-draft.md",
        priority: "high",
      },
    ],
  },
  {
    id: "pre_story",
    label: "PHASE 1: PRE-STORY FOUNDATIONS",
    color: "#6a8abf",
    icon: "\u{1F527}",
    description: "Everything needed BEFORE we can write story descriptions. These are the databases that stories reference.",
    items: [
      {
        name: "1A. Companion Roster v2",
        detail: "13 companions fully designed. For each: name, race, gender, class, personality (2-3 sentences), world skill, recruitment act, handicaps/special traits, personal quest concept. Must include One-Armed Warrior (can't swim/use 2H until prosthetic), Battle Mage (energy sword), Crystal Deepkin Mage, Umbrath Shadow Priest.",
        depends: [],
        priority: "critical",
      },
      {
        name: "1B. NPC Roster",
        detail: "Recurring named NPCs across acts. King of Caelmund, guild master, merchants, quest givers, village elders, Shadow World contacts. Each needs: name, race, role, personality, which acts they appear in. The traitor party members are already done — this is everyone ELSE.",
        depends: [],
        priority: "high",
      },
      {
        name: "1C. Basic Items Catalog",
        detail: "Non-unique equipment available in shops/drops per act. Weapons (by type: sword, axe, bow, staff, dagger, 2H), Armor (light/medium/heavy per act), Shields (per act), Accessories, Potions (HP/mana/buff/cure). Price ranges. Stat values per tier. This is the 'baseline' that unique items contrast against.",
        depends: [],
        priority: "high",
      },
      {
        name: "1D. Acts 2-4 Bestiary",
        detail: "Enemy families for Shadow World (Act 2), surface Act 3 (traitor-aligned soldiers, corrupted creatures, Gorath's minions), and Gorath's Domain (Act 4). Same format as Act 1: families, ranks, stat tiers. Unique enemies get added during story writing.",
        depends: [],
        priority: "medium",
      },
      {
        name: "1E. Story Format Spec",
        detail: "The exact format the writer agent receives and produces. Each story/quest needs: scene description, implications/consequences, tags (set/check), image tags with naming convention (e.g. 'forest_path_01', 'cave_entrance_01'). Define: how image tags map to scenes, how transitions work, tag format, what metadata each quest carries.",
        depends: ["1A"],
        priority: "critical",
      },
    ],
  },
  {
    id: "story_phase",
    label: "PHASE 2: STORY DESCRIPTIONS",
    color: "#bf6a8a",
    icon: "\u{1F4DC}",
    description: "Write short story descriptions for ALL quests. Each description feeds the writer agent who expands it into full narrative. While writing these, we populate the tracking artifacts in PARALLEL.",
    items: [
      {
        name: "2A. Main Quest Descriptions",
        detail: "Expand the main quest table entries into writer-ready briefs. Each main quest gets: scene description (what happens), implications (what this means for the story), tags (set/checked), image tags (named scene images for visual novel moments). All 4 boss paths per act need variants.",
        depends: ["1E"],
        priority: "critical",
        parallel: ["2E", "2F", "2G"],
      },
      {
        name: "2B. Companion Quest Descriptions",
        detail: "13 recruitment quests + 13 personal quests = 26 quests. Each companion has a personal arc: recruitment story \u2192 bonding \u2192 personal quest \u2192 resolution. Multi-act arcs for deeper companions.",
        depends: ["1A", "1E"],
        priority: "high",
        parallel: ["2E", "2F", "2G"],
      },
      {
        name: "2C. Continuation Quest Descriptions",
        detail: "Multi-act side quest arcs. Examples: Shadow Hero thread (meet Act 1, appears Act 2-4), merchant caravan protection chain, cult infiltration subplot, Deepkin diplomatic arc. These span multiple acts and add depth. NOT one-shots.",
        depends: ["1A", "1B", "1E"],
        priority: "high",
        parallel: ["2E", "2F", "2G"],
      },
      {
        name: "2D. One-Shot Quest Descriptions",
        detail: "Self-contained side quests per act. Village defense, monster hunts, mystery investigations, escort missions, dungeon crawls. Pool quests that get randomly selected. 5-10 per act.",
        depends: ["1E"],
        priority: "medium",
        parallel: ["2E", "2F", "2G"],
      },
      {
        name: "2E. Unique Items Tracker",
        detail: "FILLED IN PARALLEL while writing stories. Every time a story introduces a named/special item (boss drops, quest rewards, companion quest items, hidden treasures), it gets added here. Tracks: item name, type, stats, how obtained, which quest, lore.",
        depends: [],
        priority: "parallel",
      },
      {
        name: "2F. Unique Enemies Tracker",
        detail: "FILLED IN PARALLEL while writing stories. Named/special enemies beyond the basic bestiary. Mini-bosses, quest-specific encounters, named villains, rare creatures. Tracks: name, type, stats, abilities, where encountered, lore.",
        depends: [],
        priority: "parallel",
      },
      {
        name: "2G. Unique NPCs Tracker",
        detail: "FILLED IN PARALLEL while writing stories. Every named NPC that appears. Extends the NPC roster from 1B. Tracks: name, race, role, personality, which quests they appear in, relationship to player.",
        depends: [],
        priority: "parallel",
      },
    ],
  },
  {
    id: "post_story",
    label: "PHASE 3: REFINEMENT & HANDOFF",
    color: "#8a6abf",
    icon: "\u{1F3AF}",
    description: "After all descriptions are written, refine and prepare for the writer agent and coding agent.",
    items: [
      {
        name: "3A. Consistency Pass",
        detail: "Cross-reference all story descriptions against: tag usage, item references, NPC appearances, companion availability per act, boss path logic. Fix contradictions.",
        depends: ["2A", "2B", "2C", "2D"],
        priority: "high",
      },
      {
        name: "3B. Balance Review",
        detail: "Check unique items against basic items (no power gaps). Check unique enemies against bestiary power bands. Verify level curve still holds. Adjust numbers doc if needed.",
        depends: ["2E", "2F", "3A"],
        priority: "medium",
      },
      {
        name: "3C. Writer Agent Briefing Package",
        detail: "Compile everything the storyteller AI needs: lore bible, companion roster, NPC roster, story format spec, all quest descriptions, image tag library, tag dictionary. One handoff package.",
        depends: ["3A", "3B"],
        priority: "critical",
      },
      {
        name: "3D. Coding Agent Architecture Package",
        detail: "Compile everything the coding agent needs: quest data structures, tag system implementation, image loading specs, combat encounter formats, item/enemy JSON schemas.",
        depends: ["3A", "3B"],
        priority: "critical",
      },
      {
        name: "3E. Fill Remaining Skill Gaps",
        detail: "8 actives + 4 passives still missing across 6 classes. Can be designed last since story writing doesn't depend on specific skill names — only class roles matter for narrative.",
        depends: [],
        priority: "low",
      },
    ],
  },
];

const PRIORITY_COLORS = {
  critical: { bg: "#3a1a1a", border: "#6a2a2a", text: "#d4a0a0" },
  high: { bg: "#2a2a1a", border: "#5a5a2a", text: "#d4d4a0" },
  medium: { bg: "#1a2a2a", border: "#2a5a5a", text: "#a0d4d4" },
  low: { bg: "#1a1a2a", border: "#2a2a5a", text: "#a0a0d4" },
  parallel: { bg: "#2a1a2a", border: "#5a2a5a", text: "#d4a0d4" },
};

function PhaseSection({ phase, isExpanded, onToggle }) {
  const doneCount = phase.items.filter(i => i.done).length;

  return (
    <div style={{
      background: "#0c0c10",
      border: `1px solid ${phase.color}33`,
      borderRadius: 6,
      marginBottom: 12,
      overflow: "hidden",
    }}>
      <div
        onClick={onToggle}
        style={{
          padding: "14px 18px",
          cursor: "pointer",
          display: "flex",
          alignItems: "center",
          gap: 10,
          borderBottom: isExpanded ? `1px solid ${phase.color}22` : "none",
        }}
      >
        <span style={{ fontSize: 16 }}>{phase.icon}</span>
        <span style={{ fontSize: 14, color: phase.color, fontWeight: 600, flex: 1 }}>
          {phase.label}
        </span>
        <span style={{ fontSize: 10, color: "#5a5248" }}>
          {phase.items.length} items
        </span>
        <span style={{ color: "#4a4238", fontSize: 14 }}>
          {isExpanded ? "\u25B2" : "\u25BC"}
        </span>
      </div>

      {isExpanded && (
        <div style={{ padding: "8px 18px 18px" }}>
          {phase.description && (
            <div style={{
              fontSize: 11, color: "#6a6058", fontStyle: "italic",
              marginBottom: 12, lineHeight: 1.6,
            }}>
              {phase.description}
            </div>
          )}

          {phase.items.map((item, i) => {
            const p = PRIORITY_COLORS[item.priority] || PRIORITY_COLORS.medium;
            return (
              <div key={i} style={{
                background: item.needsFix ? "#1a1208" : p?.bg || "#0a0a0f",
                border: `1px solid ${item.needsFix ? "#3a2a0a" : p?.border || "#1a1815"}`,
                borderLeft: `3px solid ${phase.color}`,
                borderRadius: 4,
                padding: "10px 14px",
                marginBottom: 6,
              }}>
                <div style={{
                  display: "flex", alignItems: "center", gap: 8,
                  marginBottom: 6, flexWrap: "wrap",
                }}>
                  <span style={{ fontSize: 12, color: "#d4c4b0", fontWeight: 600 }}>
                    {item.name}
                  </span>
                  {item.priority && (
                    <span style={{
                      fontSize: 9, padding: "2px 6px", borderRadius: 3,
                      background: p.bg, border: `1px solid ${p.border}`,
                      color: p.text, textTransform: "uppercase",
                    }}>
                      {item.priority}
                    </span>
                  )}
                  {item.needsFix && (
                    <span style={{
                      fontSize: 9, padding: "2px 6px", borderRadius: 3,
                      background: "#3a2a0a", color: "#d4a574",
                    }}>NEEDS UPDATE</span>
                  )}
                  {item.artifact && (
                    <span style={{
                      fontSize: 9, color: "#4a6a4a",
                      fontFamily: "monospace",
                    }}>{item.artifact}</span>
                  )}
                </div>
                <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.6 }}>
                  {item.detail}
                </div>
                {item.depends && item.depends.length > 0 && (
                  <div style={{
                    fontSize: 10, color: "#5a5248", marginTop: 4,
                  }}>
                    Depends on: {item.depends.join(", ")}
                  </div>
                )}
                {item.parallel && item.parallel.length > 0 && (
                  <div style={{
                    fontSize: 10, color: "#8a6abf", marginTop: 4,
                  }}>
                    Fills in parallel: {item.parallel.join(", ")}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

export default function Roadmap() {
  const [expanded, setExpanded] = useState("pre_story");

  return (
    <div style={{
      minHeight: "100vh",
      background: "#0a0a0f",
      color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        {/* Header */}
        <div style={{
          textAlign: "center", marginBottom: 20,
          borderBottom: "1px solid #2a2520", paddingBottom: 16,
        }}>
          <h1 style={{
            fontSize: 22, color: "#d4a574", fontWeight: 400,
            letterSpacing: 4, textTransform: "uppercase", margin: 0,
          }}>
            Vassnian Development Roadmap
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            From systems to story descriptions \u2022 Updated Feb 20, 2026
          </p>
        </div>

        {/* Flow Diagram */}
        <div style={{
          background: "#0c0c10", border: "1px solid #1a1815",
          borderRadius: 6, padding: "16px 20px", marginBottom: 20,
          textAlign: "center",
        }}>
          <div style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 2, marginBottom: 10 }}>
            Development Flow
          </div>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "center", gap: 6, flexWrap: "wrap", fontSize: 11 }}>
            {[
              { label: "Systems \u2705", color: "#4a8a4a" },
              { label: "\u2192", color: "#3a3530" },
              { label: "Fix Docs \u26A0\uFE0F", color: "#bf8a4a" },
              { label: "\u2192", color: "#3a3530" },
              { label: "Pre-Story \u{1F527}", color: "#6a8abf" },
              { label: "\u2192", color: "#3a3530" },
              { label: "Story Descriptions \u{1F4DC}", color: "#bf6a8a" },
              { label: "\u2192", color: "#3a3530" },
              { label: "Refinement \u{1F3AF}", color: "#8a6abf" },
            ].map((s, i) => (
              <span key={i} style={{ color: s.color, fontWeight: s.label.includes("\u2192") ? 400 : 600 }}>
                {s.label}
              </span>
            ))}
          </div>
          <div style={{ fontSize: 10, color: "#4a4238", marginTop: 8 }}>
            During story descriptions: Unique Items + Unique Enemies + Unique NPCs filled in parallel
          </div>
        </div>

        {/* Story Description Format Preview */}
        <div style={{
          background: "#0f0e0c", border: "1px solid #2a2520",
          borderRadius: 6, padding: "14px 18px", marginBottom: 20,
        }}>
          <div style={{
            fontSize: 11, color: "#d4a574", fontWeight: 600,
            textTransform: "uppercase", letterSpacing: 1, marginBottom: 8,
          }}>
            Target: What Each Story Description Contains
          </div>
          <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.8 }}>
            <div><span style={{ color: "#6abf6a" }}>\u2022 Scene Description</span> — What happens, who's involved, where it takes place</div>
            <div><span style={{ color: "#bfa86a" }}>\u2022 Implications</span> — Story consequences, what this sets up for later</div>
            <div><span style={{ color: "#bf6a6a" }}>\u2022 Tags</span> — Tags checked (prerequisites) and tags set (consequences)</div>
            <div><span style={{ color: "#6a6abf" }}>\u2022 Image Tags</span> — Named scene images (e.g. forest_path_01, cave_entrance_01, throne_room_dark_01)</div>
            <div><span style={{ color: "#8a6abf" }}>\u2022 Unique Refs</span> — Any unique items, enemies, or NPCs introduced (added to trackers)</div>
          </div>
          <div style={{
            fontSize: 10, color: "#5a5248", marginTop: 8,
            borderTop: "1px solid #1a1815", paddingTop: 6,
          }}>
            Writer agent receives this brief \u2192 Expands into full visual novel narrative with dialogue, choices, and branching
          </div>
        </div>

        {/* Side Quest Clarification */}
        <div style={{
          background: "#0c1014", border: "1px solid #1a2a30",
          borderRadius: 6, padding: "14px 18px", marginBottom: 20,
        }}>
          <div style={{
            fontSize: 11, color: "#6a8abf", fontWeight: 600,
            textTransform: "uppercase", letterSpacing: 1, marginBottom: 8,
          }}>
            Quest Types Reminder (5 Types)
          </div>
          <div style={{ fontSize: 11, color: "#8a8078", lineHeight: 1.8 }}>
            <div><span style={{ color: "#d4a574", fontWeight: 600 }}>Main</span> — Boss-path stories, always present (priority 1)</div>
            <div><span style={{ color: "#6abf6a", fontWeight: 600 }}>Recruitment</span> — Companion join quests (priority 2)</div>
            <div><span style={{ color: "#bfa86a", fontWeight: 600 }}>Companion</span> — Personal companion arcs (priority 3)</div>
            <div><span style={{ color: "#bf6a8a", fontWeight: 600 }}>Continuation</span> — Multi-act side arcs like Shadow Hero, merchant chains, cult subplot. Meet in Act 1 \u2192 appears in Act 2, 3, 4 (priority 4)</div>
            <div><span style={{ color: "#6a6abf", fontWeight: 600 }}>One-shot</span> — Self-contained side quests, pool-filled (priority 5)</div>
          </div>
        </div>

        {/* Parallel Tracking Artifacts */}
        <div style={{
          background: "#140c18", border: "1px solid #2a1a30",
          borderRadius: 6, padding: "14px 18px", marginBottom: 20,
        }}>
          <div style={{
            fontSize: 11, color: "#8a6abf", fontWeight: 600,
            textTransform: "uppercase", letterSpacing: 1, marginBottom: 8,
          }}>
            Parallel Tracking Artifacts (Filled During Story Writing)
          </div>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 8 }}>
            {[
              { name: "Unique Items", desc: "Boss drops, quest rewards, companion items, hidden treasures. Name, type, stats, source quest, lore.", color: "#d4a574" },
              { name: "Unique Enemies", desc: "Named mini-bosses, quest encounters, rare creatures. Name, family, rank, abilities, source quest.", color: "#bf6a6a" },
              { name: "Unique NPCs", desc: "Every named character. Name, race, role, personality, quest appearances, relationships.", color: "#6abf6a" },
              { name: "Basic Items", desc: "Shop/drop equipment by act. Weapons, armor, shields, accessories, potions. Prices and stats.", color: "#6a8abf" },
            ].map((t, i) => (
              <div key={i} style={{
                background: "#0a0a0f", border: `1px solid ${t.color}33`,
                borderRadius: 4, padding: "10px 12px",
              }}>
                <div style={{ fontSize: 11, color: t.color, fontWeight: 600, marginBottom: 4 }}>
                  {t.name}
                </div>
                <div style={{ fontSize: 10, color: "#6a6058", lineHeight: 1.5 }}>
                  {t.desc}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Phases */}
        {PHASES.map(phase => (
          <PhaseSection
            key={phase.id}
            phase={phase}
            isExpanded={expanded === phase.id}
            onToggle={() => setExpanded(expanded === phase.id ? null : phase.id)}
          />
        ))}

        {/* Next Steps */}
        <div style={{
          marginTop: 20, padding: 16,
          background: "#1a0a0a", border: "1px solid #3a1a1a",
          borderRadius: 6,
        }}>
          <div style={{
            fontSize: 12, color: "#bf6a6a", fontWeight: 600,
            textTransform: "uppercase", letterSpacing: 1, marginBottom: 8,
          }}>
            Immediate Next Steps
          </div>
          <div style={{ fontSize: 11, color: "#a09888", lineHeight: 2 }}>
            <div>1. <span style={{ color: "#d4a574", fontWeight: 600 }}>Companion Roster v2</span> — 13 characters fully designed (CRITICAL blocker)</div>
            <div>2. <span style={{ color: "#d4a574", fontWeight: 600 }}>Story Format Spec</span> — Define exactly what each story brief contains</div>
            <div>3. <span style={{ color: "#bfa86a", fontWeight: 600 }}>NPC Roster</span> — Recurring characters across acts</div>
            <div>4. <span style={{ color: "#bfa86a", fontWeight: 600 }}>Basic Items Catalog</span> — Baseline equipment per act</div>
            <div>5. <span style={{ color: "#6a8abf", fontWeight: 600 }}>Acts 2-4 Bestiary</span> — Enemy families for remaining acts</div>
            <div>6. <span style={{ color: "#bf6a8a", fontWeight: 600 }}>Begin Story Descriptions</span> — Start with main quests, fill trackers in parallel</div>
          </div>
        </div>

        {/* Stats Footer */}
        <div style={{
          marginTop: 16, padding: 12,
          background: "#0f0e0c", border: "1px solid #1a1815",
          borderRadius: 4, textAlign: "center",
          fontSize: 10, color: "#4a4238", lineHeight: 1.8,
        }}>
          9 systems completed \u2022 2 docs need fixing \u2022 5 pre-story tasks \u2022 7 story-phase tasks \u2022 5 refinement tasks
          <br />
          Estimated: ~3-4 sessions to reach story writing phase
        </div>
      </div>
    </div>
  );
}
