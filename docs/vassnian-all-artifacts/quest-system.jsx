import { useState } from "react";

// ============================================================
// VASSNIAN QUEST SYSTEM — DESIGN REFERENCE
// ============================================================

const QUEST_TYPES = [
  {
    id: "main",
    label: "Main Quest",
    color: "#d4a574",
    bg: "#2e2418",
    border: "#5a4a30",
    priority: 1,
    desc: "Core story quests. Locked positions in the array. Always happens. Boss selection determines WHICH main quests appear (e.g. Goblin King run = goblin-themed main quests). 4 possible boss paths per act means 4 sets of main quests per act.",
    chainable: false,
    conditional: false,
  },
  {
    id: "recruitment",
    label: "Recruitment Quest",
    color: "#6abf6a",
    bg: "#1a2e1a",
    border: "#2d5a2d",
    priority: 2,
    desc: "Only appears when player is MISSING companions (empty party slots). Leads to finding and recruiting a new party member. Disappears once party is full. Each recruitable companion has their own recruitment quest.",
    chainable: false,
    conditional: true,
  },
  {
    id: "companion",
    label: "Companion Quest",
    color: "#6a9abf",
    bg: "#1a2228",
    border: "#2d4a5a",
    priority: 3,
    desc: "Continuation chains tied to a specific companion you already have. Triggered after recruitment. Gets inserted into the array, displacing one-shots/part-1s. Examples: finding the orc's prosthetic arm, dark artifacts for Paladin #2, helping Battle Mage search for her team.",
    chainable: true,
    conditional: true,
  },
  {
    id: "continuation",
    label: "Continuation Quest",
    color: "#bf8a6a",
    bg: "#2e2018",
    border: "#5a3a28",
    priority: 4,
    desc: "Multi-part story chains NOT tied to specific companions. Part 1 appears as filler. Success = remaining parts placed in future sub-acts. Failure = chain removed, slots freed. Example: Shadow Party quest chain.",
    chainable: true,
    conditional: false,
  },
  {
    id: "oneshot",
    label: "One-Shot Quest",
    color: "#8a8078",
    bg: "#1a1918",
    border: "#3a3530",
    priority: 5,
    desc: "Standalone quests with no chain. Fill remaining slots. First to be displaced when higher-priority quests need space. Provide variety, loot, XP, and world-building.",
    chainable: false,
    conditional: false,
  },
];

const TAG_TYPES = [
  { id: "companion_present", label: "Companion Present", color: "#6abf6a", desc: "Checks if specific companion is in active party. Triggers unique dialogue, reactions, or alternate paths.", example: 'companion_present: "orc_warrior" → Orc reacts to orc NPCs differently' },
  { id: "companion_missing", label: "Companion Missing", color: "#bf6a6a", desc: "Checks if party has empty slots. Controls recruitment quest availability.", example: 'companion_missing: count >= 1 → recruitment quests can appear' },
  { id: "world_skill", label: "World Skill", color: "#bfa86a", desc: "Checks party world skills. Opens secret paths, alternate solutions, bonus loot, or skips combat.", example: 'world_skill: "lockpicking" → secret door opens to treasure room' },
  { id: "has_item", label: "Has Item", color: "#bf6abf", desc: "Checks player inventory for specific items. Triggers special events, quest branches, or companion reactions.", example: 'has_item: "dark_artifact_1" → Paladin #2 gains new ability + story event' },
  { id: "choice_flag", label: "Choice Flag", color: "#6a8abf", desc: "Tracks outcomes of previous quests. Affects NPC attitudes, available options, and story consequences in chain quests.", example: 'choice_flag: "saved_village" = true → villagers help you in part 2' },
  { id: "banter", label: "Banter", color: "#8abf6a", desc: "Pulls contextual party dialogue based on companion combination. Can be specific pair or random from pool.", example: 'banter: ["cat_knight", "wolf_paladin"] → rivalry dialogue during travel' },
  { id: "personality_react", label: "Personality React", color: "#bfbf6a", desc: "Companion trait triggers reaction to quest context. Adds flavor and character depth.", example: 'personality_react: "drinks" + "bar_scene" → Dwarf priest orders a round' },
  { id: "chain_outcome", label: "Chain Outcome", color: "#bf8a6a", desc: "Checks result of previous chain part. Gates access to next part and modifies its content.", example: 'chain_outcome: "shadow_party_chain" = "success" → part 2 unlocked' },
  { id: "stat_check", label: "Stat Check", color: "#8a6abf", desc: "Checks party stats against threshold. Enables harder paths, bonus objectives, or alternate encounters.", example: 'stat_check: party_avg_level >= 3 → elite path available' },
  { id: "boss_theme", label: "Boss Theme", color: "#d4a574", desc: "Tags main quests to specific boss selection. Only these main quests appear when that boss is rolled for the act.", example: 'boss_theme: "goblin_king" → goblin-themed main quests + events' },
];

const QUEST_TEMPLATE = {
  id: "quest_unique_id",
  type: "main | recruitment | companion | continuation | oneshot",
  name: "Quest Display Name",
  act: "1-4",
  sub_act_range: "[1.1, 1.3] — which sub-acts it can appear in",
  chain_id: "null | chain_group_id",
  chain_part: "null | 1, 2, 3...",
  rank_range: "[min, max] — enemy rank range for encounters",
  boss_theme: "null | boss_id — only for main quests",
  companion_id: "null | companion_id — only for recruitment/companion quests",
  tags: "[array of tag objects]",
  rewards: {
    xp: "amount",
    gold: "amount",
    items: "[item_ids]",
    companion_unlock: "null | companion_id",
    skill_unlock: "null | skill_id",
    story_flag: "null | { flag_name: value }",
  },
  on_success: "null | { place_chain_part: 2, set_flag: 'x' }",
  on_failure: "null | { remove_chain: chain_id, set_flag: 'y' }",
  description: "Story summary for reference",
};

const ARRAY_EXAMPLE_BEFORE = [
  { slot: 1, quest: "The King's Command", type: "main", locked: true, note: "Goblin King boss path — goblin raid reported" },
  { slot: 2, quest: "Find the Lost Warrior", type: "recruitment", locked: false, note: "Player missing 1 companion" },
  { slot: 3, quest: "Shadow Party Pt.1", type: "continuation", locked: false, note: "New chain intro" },
  { slot: 4, quest: "Wolves at the Farm", type: "oneshot", locked: false, note: "Filler" },
  { slot: 5, quest: "Merchant's Lost Goods", type: "oneshot", locked: false, note: "Filler" },
];

const ARRAY_EXAMPLE_AFTER = [
  { slot: 1, quest: "The King's Command", type: "main", locked: true, note: "Unchanged — main quest locked" },
  { slot: 2, quest: "The Orc's Past (Pt.1)", type: "companion", locked: false, note: "Orc recruited → his chain inserted" },
  { slot: 3, quest: "Shadow Party Pt.1", type: "continuation", locked: false, note: "Unchanged — active chain stays" },
  { slot: 4, quest: "Wolves at the Farm", type: "oneshot", locked: false, note: "Unchanged — still has room" },
  { slot: 5, quest: "—removed—", type: "removed", locked: false, note: "Merchant quest displaced to fit companion chain" },
];

const ARRAY_FLOW_STEPS = [
  { step: 1, title: "Lock Main Quests", desc: "Main quests placed in fixed positions. Boss selection determines which set of main quests. These NEVER move." },
  { step: 2, title: "Check Companion Slots", desc: "If party has empty slots → place recruitment quests. Once party full → these slots free up." },
  { step: 3, title: "Place Active Chain Parts", desc: "Any continuation or companion chain you're in the middle of — next parts get placed. These take priority over new content." },
  { step: 4, title: "Introduce New Chain Part-1s", desc: "Fresh continuation chains and companion chains get their part 1 placed. These are the hooks for new storylines." },
  { step: 5, title: "Fill with One-Shots", desc: "Remaining empty slots filled with one-shot quests. These are the first to be displaced if something higher-priority needs space." },
  { step: 6, title: "React to Events", desc: "Mid sub-act changes: recruit companion → insert their chain (kick one-shots). Fail a chain → remove future parts (add one-shots). Complete chain part → queue next part for next sub-act." },
];

const typeColorMap = {};
QUEST_TYPES.forEach(t => { typeColorMap[t.id] = t; });

export default function QuestSystem() {
  const [activeTab, setActiveTab] = useState("types");
  const [expandedTag, setExpandedTag] = useState(null);
  const [showAfter, setShowAfter] = useState(false);

  const tabs = [
    { id: "types", label: "Quest Types" },
    { id: "tags", label: "Tag System" },
    { id: "template", label: "Quest Template" },
    { id: "array", label: "Array Flow" },
    { id: "example", label: "Live Example" },
  ];

  return (
    <div style={{
      minHeight: "100vh",
      background: "#0a0a0f",
      color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 960, margin: "0 auto" }}>
        {/* Header */}
        <div style={{
          textAlign: "center",
          marginBottom: 24,
          borderBottom: "1px solid #2a2520",
          paddingBottom: 20,
        }}>
          <h1 style={{
            fontSize: 26,
            color: "#d4a574",
            fontWeight: 400,
            letterSpacing: 4,
            textTransform: "uppercase",
            margin: 0,
          }}>
            \u2694 Vassnian Quest System \u2694
          </h1>
          <p style={{ color: "#6a6058", fontSize: 12, marginTop: 8 }}>
            5 Quest Types \u2022 10 Tag Types \u2022 Dynamic Array \u2022 Priority-Based Placement
          </p>
        </div>

        {/* Tabs */}
        <div style={{
          display: "flex",
          gap: 4,
          marginBottom: 24,
          borderBottom: "1px solid #1a1815",
          paddingBottom: 0,
        }}>
          {tabs.map(t => (
            <button
              key={t.id}
              onClick={() => setActiveTab(t.id)}
              style={{
                background: activeTab === t.id ? "#1a1815" : "transparent",
                border: "1px solid",
                borderColor: activeTab === t.id ? "#2a2520" : "transparent",
                borderBottom: activeTab === t.id ? "1px solid #0a0a0f" : "1px solid #1a1815",
                color: activeTab === t.id ? "#d4a574" : "#5a5248",
                padding: "8px 16px",
                borderRadius: "4px 4px 0 0",
                cursor: "pointer",
                fontSize: 12,
                fontFamily: "inherit",
                marginBottom: -1,
              }}
            >
              {t.label}
            </button>
          ))}
        </div>

        {/* TAB: Quest Types */}
        {activeTab === "types" && (
          <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
            <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 4, textTransform: "uppercase", letterSpacing: 2 }}>
              Priority Order (1 = highest, 5 = lowest \u2014 displaced first)
            </div>
            {QUEST_TYPES.map(qt => (
              <div key={qt.id} style={{
                background: qt.bg,
                border: `1px solid ${qt.border}`,
                borderRadius: 4,
                padding: "14px 18px",
              }}>
                <div style={{ display: "flex", alignItems: "center", gap: 10, marginBottom: 8 }}>
                  <span style={{
                    fontFamily: "monospace",
                    fontSize: 11,
                    color: "#0a0a0f",
                    background: qt.color,
                    padding: "2px 8px",
                    borderRadius: 3,
                    fontWeight: 700,
                  }}>
                    P{qt.priority}
                  </span>
                  <span style={{ fontSize: 16, color: qt.color, fontWeight: 600 }}>
                    {qt.label}
                  </span>
                  <div style={{ flex: 1 }} />
                  {qt.chainable && (
                    <span style={{
                      fontSize: 10,
                      color: "#bf8a6a",
                      border: "1px solid #5a3a28",
                      padding: "2px 8px",
                      borderRadius: 3,
                    }}>CHAINABLE</span>
                  )}
                  {qt.conditional && (
                    <span style={{
                      fontSize: 10,
                      color: "#bf6abf",
                      border: "1px solid #5a2d5a",
                      padding: "2px 8px",
                      borderRadius: 3,
                    }}>CONDITIONAL</span>
                  )}
                </div>
                <p style={{ fontSize: 12, color: "#a09888", margin: 0, lineHeight: 1.6 }}>
                  {qt.desc}
                </p>
              </div>
            ))}
          </div>
        )}

        {/* TAB: Tag System */}
        {activeTab === "tags" && (
          <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
            <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 4, textTransform: "uppercase", letterSpacing: 2 }}>
              10 Tag Types \u2014 Click to expand
            </div>
            {TAG_TYPES.map(tag => {
              const isExp = expandedTag === tag.id;
              return (
                <div
                  key={tag.id}
                  onClick={() => setExpandedTag(isExp ? null : tag.id)}
                  style={{
                    background: isExp ? "#12110f" : "#0f0e0c",
                    border: `1px solid ${isExp ? "#2a2520" : "#1a1815"}`,
                    borderRadius: 4,
                    padding: "10px 14px",
                    cursor: "pointer",
                    transition: "all 0.15s ease",
                  }}
                >
                  <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
                    <span style={{
                      width: 8,
                      height: 8,
                      borderRadius: "50%",
                      background: tag.color,
                      flexShrink: 0,
                    }} />
                    <span style={{ fontSize: 13, color: tag.color, fontWeight: 600 }}>
                      {tag.label}
                    </span>
                    <span style={{ fontSize: 11, color: "#4a4238", flex: 1 }}>
                      {tag.id}
                    </span>
                  </div>
                  {isExp && (
                    <div style={{ marginTop: 10, paddingTop: 8, borderTop: "1px solid #1a1815" }}>
                      <p style={{ fontSize: 12, color: "#a09888", margin: "0 0 8px 0", lineHeight: 1.6 }}>
                        {tag.desc}
                      </p>
                      <div style={{
                        fontSize: 11,
                        color: "#8a7a6a",
                        background: "#08080a",
                        padding: "6px 10px",
                        borderRadius: 3,
                        borderLeft: `2px solid ${tag.color}`,
                        fontFamily: "monospace",
                      }}>
                        {tag.example}
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}

        {/* TAB: Quest Template */}
        {activeTab === "template" && (
          <div>
            <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 12, textTransform: "uppercase", letterSpacing: 2 }}>
              Every quest uses this structure
            </div>
            <div style={{
              background: "#0f0e0c",
              border: "1px solid #1a1815",
              borderRadius: 4,
              padding: "16px 20px",
              fontFamily: "monospace",
              fontSize: 11,
              lineHeight: 1.8,
              color: "#a09888",
              overflowX: "auto",
            }}>
              {Object.entries(QUEST_TEMPLATE).map(([key, val]) => {
                const isObj = typeof val === "object" && val !== null;
                return (
                  <div key={key}>
                    <span style={{ color: "#6a9abf" }}>{key}</span>
                    <span style={{ color: "#4a4238" }}>: </span>
                    {isObj ? (
                      <div style={{ paddingLeft: 20 }}>
                        {Object.entries(val).map(([k2, v2]) => (
                          <div key={k2}>
                            <span style={{ color: "#8abf6a" }}>{k2}</span>
                            <span style={{ color: "#4a4238" }}>: </span>
                            <span style={{ color: "#bfa86a" }}>{JSON.stringify(v2)}</span>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <span style={{ color: "#bfa86a" }}>{JSON.stringify(val)}</span>
                    )}
                  </div>
                );
              })}
            </div>

            <div style={{
              marginTop: 16,
              background: "#0f0e0c",
              border: "1px solid #1a1815",
              borderRadius: 4,
              padding: "16px 20px",
            }}>
              <div style={{ fontSize: 12, color: "#d4a574", marginBottom: 8, fontWeight: 600 }}>Key Fields Explained</div>
              <div style={{ fontSize: 11, color: "#8a7a6a", lineHeight: 1.8 }}>
                <div><span style={{ color: "#6a9abf" }}>chain_id</span> + <span style={{ color: "#6a9abf" }}>chain_part</span> \u2014 Links quest to a multi-part chain. null for one-shots and main quests.</div>
                <div><span style={{ color: "#6a9abf" }}>boss_theme</span> \u2014 Only for main quests. Ties them to a specific boss selection. null means appears regardless.</div>
                <div><span style={{ color: "#6a9abf" }}>companion_id</span> \u2014 Only for recruitment and companion quests. Which companion this quest is about.</div>
                <div><span style={{ color: "#6a9abf" }}>rank_range</span> \u2014 Enemy rank range for combat in this quest. Matched to sub-act level.</div>
                <div><span style={{ color: "#6a9abf" }}>on_success</span> \u2014 What happens when quest completed. Place next chain part, set flags, unlock rewards.</div>
                <div><span style={{ color: "#6a9abf" }}>on_failure</span> \u2014 What happens when quest failed. Remove chain, set failure flags, free slots.</div>
                <div><span style={{ color: "#6a9abf" }}>tags</span> \u2014 Array of conditional tags that modify quest content based on party state, items, skills, and history.</div>
              </div>
            </div>
          </div>
        )}

        {/* TAB: Array Flow */}
        {activeTab === "array" && (
          <div>
            <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 12, textTransform: "uppercase", letterSpacing: 2 }}>
              How the quest array fills each sub-act
            </div>
            <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
              {ARRAY_FLOW_STEPS.map(s => (
                <div key={s.step} style={{
                  background: "#0f0e0c",
                  border: "1px solid #1a1815",
                  borderRadius: 4,
                  padding: "12px 16px",
                  display: "flex",
                  gap: 14,
                  alignItems: "flex-start",
                }}>
                  <div style={{
                    width: 28,
                    height: 28,
                    borderRadius: "50%",
                    background: "#1a1815",
                    border: "1px solid #2a2520",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontSize: 13,
                    color: "#d4a574",
                    fontWeight: 700,
                    flexShrink: 0,
                  }}>
                    {s.step}
                  </div>
                  <div>
                    <div style={{ fontSize: 13, color: "#d4c4b0", fontWeight: 600, marginBottom: 4 }}>
                      {s.title}
                    </div>
                    <div style={{ fontSize: 11, color: "#8a7a6a", lineHeight: 1.6 }}>
                      {s.desc}
                    </div>
                  </div>
                </div>
              ))}
            </div>

            <div style={{
              marginTop: 20,
              background: "#12110f",
              border: "1px solid #2a2520",
              borderRadius: 4,
              padding: "16px 20px",
            }}>
              <div style={{ fontSize: 12, color: "#d4a574", marginBottom: 8, fontWeight: 600 }}>Displacement Rules</div>
              <div style={{ fontSize: 11, color: "#8a7a6a", lineHeight: 1.8 }}>
                <div>When a higher-priority quest needs a slot, it kicks from the BOTTOM of priority:</div>
                <div style={{ paddingLeft: 16, marginTop: 4 }}>
                  <div>\u2022 One-shots are displaced first (Priority 5)</div>
                  <div>\u2022 Then unused chain part-1s (Priority 4)</div>
                  <div>\u2022 Active chain parts and recruitment quests are protected</div>
                  <div>\u2022 Main quests NEVER move</div>
                </div>
                <div style={{ marginTop: 8, color: "#6a6058", fontStyle: "italic" }}>
                  Chain failure: all remaining parts of that chain removed \u2192 slots freed \u2192 filled with one-shots
                </div>
                <div style={{ color: "#6a6058", fontStyle: "italic" }}>
                  Chain success: next part queued for next sub-act \u2192 may displace a one-shot there
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB: Live Example */}
        {activeTab === "example" && (
          <div>
            <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 12, textTransform: "uppercase", letterSpacing: 2 }}>
              Act 1, Sub-act 1.1 \u2014 {showAfter ? "After Recruiting Orc Warrior" : "Initial Array"}
            </div>

            <button
              onClick={() => setShowAfter(!showAfter)}
              style={{
                background: showAfter ? "#2d5a2d" : "#2a2520",
                border: `1px solid ${showAfter ? "#4a8a4a" : "#3a3530"}`,
                color: showAfter ? "#6abf6a" : "#d4a574",
                padding: "8px 20px",
                borderRadius: 4,
                cursor: "pointer",
                fontSize: 12,
                fontFamily: "inherit",
                marginBottom: 16,
              }}
            >
              {showAfter ? "\u2190 Show Before Recruitment" : "Recruit Orc Warrior \u2192"}
            </button>

            <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
              {(showAfter ? ARRAY_EXAMPLE_AFTER : ARRAY_EXAMPLE_BEFORE).map(item => {
                const tc = typeColorMap[item.type] || { color: "#5a5248", bg: "#0f0e0c", border: "#1a1815" };
                const isRemoved = item.type === "removed";
                return (
                  <div key={item.slot} style={{
                    background: isRemoved ? "#1a0a0a" : tc.bg,
                    border: `1px solid ${isRemoved ? "#3a1a1a" : tc.border}`,
                    borderRadius: 4,
                    padding: "10px 14px",
                    opacity: isRemoved ? 0.5 : 1,
                  }}>
                    <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
                      <span style={{
                        fontFamily: "monospace",
                        fontSize: 10,
                        color: "#5a5248",
                        background: "#08080a",
                        padding: "2px 6px",
                        borderRadius: 3,
                      }}>
                        SLOT {item.slot}
                      </span>
                      {!isRemoved && (
                        <span style={{
                          fontSize: 10,
                          color: "#0a0a0f",
                          background: tc.color,
                          padding: "2px 8px",
                          borderRadius: 3,
                          fontWeight: 700,
                          textTransform: "uppercase",
                        }}>
                          {item.type}
                        </span>
                      )}
                      <span style={{
                        fontSize: 13,
                        color: isRemoved ? "#5a3a3a" : "#d4c4b0",
                        fontWeight: 600,
                        flex: 1,
                        textDecoration: isRemoved ? "line-through" : "none",
                      }}>
                        {item.quest}
                      </span>
                      {item.locked && (
                        <span style={{
                          fontSize: 10,
                          color: "#d4a574",
                          border: "1px solid #5a4a30",
                          padding: "1px 6px",
                          borderRadius: 3,
                        }}>LOCKED</span>
                      )}
                    </div>
                    <div style={{ fontSize: 10, color: "#6a6058", marginTop: 4, paddingLeft: 2 }}>
                      {item.note}
                    </div>
                  </div>
                );
              })}
            </div>

            {showAfter && (
              <div style={{
                marginTop: 16,
                background: "#12110f",
                border: "1px solid #2a2520",
                borderRadius: 4,
                padding: "12px 16px",
              }}>
                <div style={{ fontSize: 12, color: "#6abf6a", marginBottom: 6, fontWeight: 600 }}>What happened:</div>
                <div style={{ fontSize: 11, color: "#8a7a6a", lineHeight: 1.8 }}>
                  <div>1. Player completed "Find the Lost Warrior" \u2192 recruited Orc Warrior</div>
                  <div>2. Recruitment quest removed (party slot filled)</div>
                  <div>3. Orc's companion chain activated \u2192 "The Orc's Past Pt.1" inserted</div>
                  <div>4. One-shot "Merchant's Lost Goods" displaced to make room</div>
                  <div>5. Shadow Party chain stays \u2014 it's an active continuation, protected</div>
                </div>
              </div>
            )}

            <div style={{
              marginTop: 16,
              background: "#0f0e0c",
              border: "1px solid #1a1815",
              borderRadius: 4,
              padding: "12px 16px",
            }}>
              <div style={{ fontSize: 12, color: "#d4a574", marginBottom: 6, fontWeight: 600 }}>Tags firing in this sub-act:</div>
              <div style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 11 }}>
                <div style={{ color: "#bfa86a" }}>
                  \u2022 <span style={{ color: "#6a9abf" }}>boss_theme</span>: "goblin_king" \u2192 Main quest is goblin-themed
                </div>
                <div style={{ color: "#bfa86a" }}>
                  \u2022 <span style={{ color: "#6a9abf" }}>world_skill</span>: party has "tracking" \u2192 Wolf-folk Paladin finds shortcut in Wolves quest
                </div>
                {showAfter && (
                  <>
                    <div style={{ color: "#bfa86a" }}>
                      \u2022 <span style={{ color: "#6a9abf" }}>companion_present</span>: "orc_warrior" \u2192 NPCs react to orc in party
                    </div>
                    <div style={{ color: "#bfa86a" }}>
                      \u2022 <span style={{ color: "#6a9abf" }}>personality_react</span>: orc is depressed \u2192 quiet dialogue, minimal banter
                    </div>
                  </>
                )}
                <div style={{ color: "#bfa86a" }}>
                  \u2022 <span style={{ color: "#6a9abf" }}>banter</span>: ["dwarf_priest", "halfling_warrior"] \u2192 Gina teases the old dwarf
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Footer Summary */}
        <div style={{
          marginTop: 32,
          padding: 16,
          background: "#0f0e0c",
          border: "1px solid #1a1815",
          borderRadius: 4,
          textAlign: "center",
        }}>
          <div style={{ fontSize: 11, color: "#5a5248", lineHeight: 1.8 }}>
            5 Quest Types \u2022 10 Tag Types \u2022 Priority-Based Array \u2022 Dynamic Displacement \u2022 Chain Success/Failure Branching
          </div>
          <div style={{ fontSize: 10, color: "#3a3530", marginTop: 4 }}>
            Quest count per sub-act: TBD \u2022 Boss quest sets: 4 per act \u2022 Total bosses: 13 (4+4+4+Gorath)
          </div>
        </div>
      </div>
    </div>
  );
}
