import { useState } from "react";

const PARTIES = [
  {
    id: "iron_vanguard",
    name: "The Iron Vanguard",
    color: "#7a8fa6",
    accent: "#4a6a8a",
    bg: "#0e1218",
    border: "#1e2a38",
    act3challenge: "Political Siege — He's the King's right hand. You need evidence, allies, and perfect timing to dethrone a war hero.",
    leader: {
      title: "The Commander",
      gender: "Male",
      combat: "Knight — Heavy armor, tower shield, formation tactics. Commands party positioning mid-fight. Buffs allies' defense.",
      personality: "Calm, strategic, speaks in measured sentences. Never raises his voice. Everyone trusts him because he radiates competence and control.",
      act0: "Gives the player genuine tactical advice during early missions. Offers to share intel. Feels like a mentor figure.",
      act1: "Joint mission where his leadership saves the day. Your companions comment on how effective he is. Shares a quiet moment about 'protecting this world.'",
      act3: "Became the King's military advisor. Restructured Caelmund's army. Politically untouchable — generals answer to him. Accusing him means fighting the entire military establishment.",
      betrayal_style: "Cold and calculated. Not personal. 'You were a variable I needed to remove. Nothing more.'",
      secret: "He was the first Godsent Gorath contacted. He didn't hesitate — he saw the offer as a promotion.",
    },
    members: [
      {
        name: "Ser Aldric",
        role: "Paladin — Off-tank, divine shield spells, can protect the Commander",
        gender: "Male",
        personality: "Loyal soldier who follows orders without question. Quiet, stoic. In Act 0-1 he nods respectfully at the player. In Act 3 he's the Commander's enforcer — obeys without doubt.",
        note: "Fights defensively. Must be separated from Commander or he'll keep shielding him.",
      },
      {
        name: "Miriel",
        role: "Priest — Healer/buffer, keeps the party alive, dispels debuffs",
        gender: "Female",
        personality: "Devout, earnest, believes she's doing god's work. In Act 0-1 she heals your wounded after a joint fight. In Act 3 she genuinely believes the Commander is Caelmund's savior.",
        note: "The only one who might not fully know the truth. Possible mercy option in Act 3 — spare or kill.",
      },
      {
        name: "Halvard",
        role: "Warrior — Frontline DPS, greatsword, aggressive charges",
        gender: "Male",
        personality: "Gruff veteran who respects strength. In Act 0-1 he challenges the player to a friendly sparring match. In Act 3 he's become brutal — enjoys the power Gorath's blessing gives him.",
        note: "Berserker rage mechanic — gets stronger as HP drops. Take him out early or he snowballs.",
      },
    ],
  },
  {
    id: "shadow_circle",
    name: "The Shadow Circle",
    color: "#9a6abf",
    accent: "#6a3a8a",
    bg: "#12101a",
    border: "#2a1e38",
    act3challenge: "Magical Countermeasures — She controls the kingdom's wards and detection spells. You need to outsmart someone who's always three steps ahead.",
    leader: {
      title: "The Prodigy",
      gender: "Female",
      combat: "Mage — Glass cannon, devastating AoE spells, teleportation, counterspells. Hardest boss to pin down.",
      personality: "Brilliant, analytical, slightly condescending but wraps it in humor. Makes you feel smart for keeping up with her. Genuinely curious about magic and the world.",
      act0: "Deciphers ancient text the player found. Explains magical theory in a way that's actually helpful. Seems excited to have intellectual equals.",
      act1: "Discovers a clue about the seal weakening. Shares it openly (it's real — she mixes truth with manipulation). Your mage companion admires her.",
      act3: "Runs Caelmund's magical defense network. She redesigned the ward system. Exposing her means the kingdom loses its magical protection — is it worth it?",
      betrayal_style: "Disappointed, like a teacher with a slow student. 'I thought you'd figure it out sooner. I left hints. You just weren't clever enough.'",
      secret: "She figured out Gorath was real before anyone else. Instead of warning others, she saw an opportunity to study a god's power firsthand.",
    },
    members: [
      {
        name: "Shade",
        role: "Rogue — Assassin, stealth attacks, poisons, vanishes mid-fight",
        gender: "Male",
        personality: "Quiet, watchful, communicates in short sentences. In Act 0-1 he's mysterious but helpful — scouts ahead, disarms traps. In Act 3 he's her spy network coordinator. Knows everything about everyone.",
        note: "Disappears and reappears during the fight. Can backstab for massive damage if you lose track of him.",
      },
      {
        name: "Yara",
        role: "Ranger — Sniper, elemental arrows, summons a shadow hawk for scouting",
        gender: "Female",
        personality: "Sarcastic, keeps distance (literally and emotionally). In Act 0-1 she makes dry jokes your companions laugh at. In Act 3 she monitors the wilderness — nothing enters Caelmund without her knowing.",
        note: "Fights from extreme range. If you don't close the gap, she'll chip you down. Shadow hawk marks targets for Shade's backstabs.",
      },
      {
        name: "Tormund",
        role: "Mage — Support caster, shields the Prodigy, disrupts enemy spells with counterspells",
        gender: "Male",
        personality: "Anxious, devoted to the Prodigy, follows her lead completely. In Act 0-1 he's nervous but friendly. In Act 3 he's her research assistant — does the dirty work she won't do herself.",
        note: "Priority target. While he's alive, the Prodigy has a magic shield. Kill him first to make her vulnerable.",
      },
    ],
  },
  {
    id: "gilded_saints",
    name: "The Gilded Saints",
    color: "#d4a554",
    accent: "#8a7030",
    bg: "#181408",
    border: "#38301a",
    act3challenge: "Hearts and Minds — He's the people's champion. Attack him and the streets turn against you. You need to shatter the legend without becoming the villain.",
    leader: {
      title: "The Showman",
      gender: "Male",
      combat: "Warrior — Dual-wield flashy style, counterattacks, taunts that debuff your party's morale. Gets stronger with an audience.",
      personality: "Loud, funny, magnetic. Buys rounds for strangers. Tells great stories. The kind of person who makes you feel like his best friend within five minutes.",
      act0: "Throws an arm around the player at the tavern. Toasts 'to us Godsents!' Makes your companions laugh. Shares food and drink freely.",
      act1: "Saves a village together. He lets you take the credit publicly, but privately his people spread stories about HIS heroics. Your bard companion adores him.",
      act3: "Became a folk hero. Songs about him in every tavern. He runs public rallies, feeds the poor, puts on a perfect show. Behind closed doors: ruthless, vindictive, paranoid.",
      betrayal_style: "Grinning, theatrical. 'Oh come on, don't look so hurt! We had fun, didn't we? The drinks were real. The friendship? Eh...' Laughs while fighting you.",
      secret: "He was a nobody in his original world. Gorath offered him what he always wanted: to be loved and powerful. He'll never give it up.",
    },
    members: [
      {
        name: "Sister Corin",
        role: "Paladin — Tank, holy smite, aura that boosts party morale/damage",
        gender: "Female",
        personality: "True believer. In Act 0-1 she preaches about justice and destiny. In Act 3 she genuinely thinks the Showman is a divine champion. Her faith makes her terrifying — she fights with absolute conviction.",
        note: "Her morale aura buffs the whole party. Silence or debuff her first, or every enemy hits harder.",
      },
      {
        name: "Old Harken",
        role: "Priest — Healer, resurrection spell (can revive one fallen ally once), holy fire",
        gender: "Male",
        personality: "Grandfatherly, warm, tells long stories. In Act 0-1 he blesses your weapons and tells you about Caelmund's history. In Act 3 he's the Showman's moral shield — 'if Old Harken trusts him, he must be good.'",
        note: "MUST be killed or disabled before any other member or he'll resurrect them. Top priority target.",
      },
      {
        name: "Lysette",
        role: "Bard — Buffer/debuffer, songs that strengthen allies and weaken enemies, charm spells",
        gender: "Female",
        personality: "Witty, flirtatious, writes songs about the party's adventures. In Act 0-1 she writes a song about you and performs it at the tavern. In Act 3 she writes propaganda ballads that make the Showman untouchable in public opinion.",
        note: "Her songs stack buffs over time. The longer the fight goes, the stronger the enemy party gets. Rush or disrupt.",
      },
    ],
  },
  {
    id: "hollow_mercy",
    name: "The Hollow Mercy",
    color: "#6abf8a",
    accent: "#2a6a4a",
    bg: "#0a1810",
    border: "#1a3828",
    act3challenge: "Unmasking the Saint — She runs refugee camps and heals the wounded. She IS hope for thousands of desperate people. Exposing her could destroy more than it saves.",
    leader: {
      title: "The Shepherd",
      gender: "Female",
      combat: "Paladin/Healer hybrid — Heavy armor, healing aura, divine barriers. Extremely hard to kill. Doesn't deal huge damage but outlasts everyone.",
      personality: "Warm, gentle, always checking on others. Remembers everyone's name. Brings medicine to the sick. Speaks softly. The most disturbing traitor because her kindness seems completely real.",
      act0: "Heals your companion's wounds after a tough fight without being asked. Shares her supplies when yours run low. Asks about your world with genuine interest.",
      act1: "Runs a field hospital for injured soldiers. Your priest companion says she's 'what we should all aspire to be.' She quietly asks if you've noticed anything strange about Gorath's influence — misdirection.",
      act3: "Runs massive refugee operations. Thousands of displaced people depend on her camps. She's beloved by the helpless. Taking her down collapses the only support system refugees have — unless you build an alternative first.",
      betrayal_style: "Sad, almost pitying. 'I'm sorry. I truly am. But you don't understand what's coming. I'm protecting them — all of them. Gorath isn't what you think.' She believes her own lie.",
      secret: "She was a caretaker in her original world — burned out, ignored, unappreciated. Gorath showed her a vision of a world where she'd never be forgotten. She convinced herself serving him IS saving people.",
    },
    members: [
      {
        name: "Dorek",
        role: "Warrior — Defensive fighter, tower shield, intercepts attacks aimed at the Shepherd",
        gender: "Male",
        personality: "Rescued from the streets by the Shepherd. Worships her. In Act 0-1 he's shy but protective. In Act 3 he's a zealot — he'll die before letting anyone touch her.",
        note: "Bodyguard mechanic — physically blocks attacks aimed at the Shepherd. Must be moved or killed to reach her.",
      },
      {
        name: "Nessa",
        role: "Mage — Nature magic, entangling vines, poison clouds, terrain control",
        gender: "Female",
        personality: "Former dark mage who the Shepherd 'redeemed.' Grateful, devoted. In Act 0-1 she's quiet and awkward, clearly unused to being treated well. In Act 3 she runs the Shepherd's intelligence — uses magic to monitor threats to the camps.",
        note: "Controls the battlefield with area denial. Locks down movement while the Shepherd heals through your damage.",
      },
      {
        name: "Brother Fenn",
        role: "Priest — Dark priest hiding as light priest, curse spells, life drain, weakens enemies over time",
        gender: "Female",
        personality: "Cheerful, chatty, cooks for everyone. In Act 0-1 she makes camp meals and tells jokes. In Act 3 the mask slips slightly — she's the most openly cruel member when no civilians are watching.",
        note: "Applies stacking debuffs. Every turn you don't deal with her, your party gets weaker. She and the Shepherd create a war of attrition.",
      },
    ],
  },
];

function PartyCard({ party, isExpanded, onToggle }) {
  return (
    <div style={{
      background: party.bg,
      border: `1px solid ${party.border}`,
      borderRadius: 6,
      marginBottom: 12,
      overflow: "hidden",
    }}>
      {/* Header */}
      <div
        onClick={onToggle}
        style={{
          padding: "14px 18px",
          cursor: "pointer",
          display: "flex",
          alignItems: "center",
          gap: 12,
          borderBottom: isExpanded ? `1px solid ${party.border}` : "none",
        }}
      >
        <span style={{
          width: 10, height: 10, borderRadius: "50%",
          background: party.color, flexShrink: 0,
        }} />
        <span style={{ fontSize: 16, color: party.color, fontWeight: 600, flex: 1 }}>
          {party.name}
        </span>
        <span style={{
          fontSize: 10, color: "#0a0a0f", background: party.color,
          padding: "2px 8px", borderRadius: 3, fontWeight: 700,
        }}>
          {party.leader.gender === "Male" ? "M" : "F"} LEADER
        </span>
        <span style={{ fontSize: 11, color: "#5a5248" }}>
          {party.leader.title}
        </span>
        <span style={{ color: "#4a4238", fontSize: 14 }}>
          {isExpanded ? "\u25B2" : "\u25BC"}
        </span>
      </div>

      {isExpanded && (
        <div style={{ padding: "0 18px 18px" }}>
          {/* Leader */}
          <div style={{
            background: "#08080c",
            border: `1px solid ${party.accent}`,
            borderLeft: `3px solid ${party.color}`,
            borderRadius: 4,
            padding: "14px 16px",
            marginTop: 14,
          }}>
            <div style={{
              display: "flex", alignItems: "center", gap: 10, marginBottom: 10,
            }}>
              <span style={{
                fontSize: 14, color: party.color, fontWeight: 700,
              }}>
                \u2605 {party.leader.title}
              </span>
              <span style={{
                fontSize: 10, padding: "2px 8px", borderRadius: 3,
                border: `1px solid ${party.accent}`, color: party.color,
              }}>
                GODSENT LEADER
              </span>
              <span style={{ fontSize: 10, color: "#6a6058" }}>
                {party.leader.gender}
              </span>
            </div>

            <Field label="Combat" value={party.leader.combat} color={party.color} />
            <Field label="Personality" value={party.leader.personality} color={party.color} />
            <Field label="Act 0 — Meeting" value={party.leader.act0} color="#6abf6a" />
            <Field label="Act 1 — Bonding" value={party.leader.act1} color="#bfa86a" />
            <Field label="Act 3 — Public Role" value={party.leader.act3} color="#bf6a6a" />
            <Field label="Betrayal Style" value={party.leader.betrayal_style} color="#d4a574" />
            <Field label="Secret" value={party.leader.secret} color="#8a6abf" />
          </div>

          {/* Members */}
          <div style={{
            fontSize: 11, color: "#5a5248", textTransform: "uppercase",
            letterSpacing: 2, marginTop: 16, marginBottom: 8,
          }}>
            Party Members
          </div>

          {party.members.map((m, i) => (
            <div key={i} style={{
              background: "#0c0c10",
              border: `1px solid ${party.border}`,
              borderRadius: 4,
              padding: "12px 14px",
              marginBottom: 8,
            }}>
              <div style={{
                display: "flex", alignItems: "center", gap: 8, marginBottom: 8,
              }}>
                <span style={{ fontSize: 13, color: "#d4c4b0", fontWeight: 600 }}>
                  {m.name}
                </span>
                <span style={{
                  fontSize: 10, color: "#6a6058",
                  background: "#14121a",
                  padding: "2px 6px", borderRadius: 3,
                }}>
                  {m.gender}
                </span>
              </div>
              <Field label="Role" value={m.role} color={party.color} />
              <Field label="Personality" value={m.personality} color="#a09888" />
              <Field label="Combat Note" value={m.note} color="#bf8a6a" />
            </div>
          ))}

          {/* Act 3 Challenge */}
          <div style={{
            background: "#1a0a0a",
            border: "1px solid #3a1a1a",
            borderRadius: 4,
            padding: "12px 14px",
            marginTop: 12,
          }}>
            <div style={{
              fontSize: 11, color: "#bf6a6a", fontWeight: 600,
              textTransform: "uppercase", letterSpacing: 1, marginBottom: 6,
            }}>
              Act 3 Challenge
            </div>
            <div style={{ fontSize: 12, color: "#c8a0a0", lineHeight: 1.6 }}>
              {party.act3challenge}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function Field({ label, value, color }) {
  return (
    <div style={{ marginBottom: 6, fontSize: 11, lineHeight: 1.6 }}>
      <span style={{ color: color, fontWeight: 600 }}>{label}: </span>
      <span style={{ color: "#a09888" }}>{value}</span>
    </div>
  );
}

export default function TraitorParties() {
  const [expanded, setExpanded] = useState("iron_vanguard");
  const [view, setView] = useState("full");

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
            fontSize: 24, color: "#d4a574", fontWeight: 400,
            letterSpacing: 4, textTransform: "uppercase", margin: 0,
          }}>
            \u2620 The Traitor Parties \u2620
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            4 Godsent Leaders \u2022 2M / 2F \u2022 Act 0 Meeting \u2022 Act 1 Bonding \u2022 Act 3 Boss Fight
          </p>
        </div>

        {/* View Toggle */}
        <div style={{
          display: "flex", gap: 4, marginBottom: 16,
        }}>
          {[
            { id: "full", label: "Full Details" },
            { id: "compare", label: "Compare Leaders" },
            { id: "combat", label: "Combat Notes" },
          ].map(v => (
            <button
              key={v.id}
              onClick={() => setView(v.id)}
              style={{
                background: view === v.id ? "#1a1815" : "transparent",
                border: `1px solid ${view === v.id ? "#2a2520" : "#1a1815"}`,
                color: view === v.id ? "#d4a574" : "#5a5248",
                padding: "6px 14px", borderRadius: 4,
                cursor: "pointer", fontSize: 11, fontFamily: "inherit",
              }}
            >
              {v.label}
            </button>
          ))}
        </div>

        {/* Full Details View */}
        {view === "full" && PARTIES.map(p => (
          <PartyCard
            key={p.id}
            party={p}
            isExpanded={expanded === p.id}
            onToggle={() => setExpanded(expanded === p.id ? null : p.id)}
          />
        ))}

        {/* Compare Leaders View */}
        {view === "compare" && (
          <div style={{ overflowX: "auto" }}>
            <table style={{
              width: "100%", borderCollapse: "collapse", fontSize: 11,
            }}>
              <thead>
                <tr>
                  {["", "The Commander", "The Prodigy", "The Showman", "The Shepherd"].map((h, i) => (
                    <th key={i} style={{
                      background: "#1a1815",
                      color: i === 0 ? "#6a6058" : PARTIES[i-1].color,
                      padding: "10px 12px", textAlign: "left",
                      borderBottom: "1px solid #2a2520", fontWeight: 600,
                      minWidth: i === 0 ? 90 : 160,
                    }}>{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {[
                  { label: "Gender", key: "gender" },
                  { label: "Combat", key: "combat" },
                  { label: "Betrayal Style", key: "betrayal_style" },
                  { label: "Secret", key: "secret" },
                ].map((row, ri) => (
                  <tr key={ri} style={{ background: ri % 2 === 0 ? "#0a0a0f" : "#0f0e0c" }}>
                    <td style={{
                      padding: "10px 12px", color: "#6a6058", fontWeight: 600,
                      borderBottom: "1px solid #1a1815", verticalAlign: "top",
                    }}>{row.label}</td>
                    {PARTIES.map((p, pi) => (
                      <td key={pi} style={{
                        padding: "10px 12px", color: "#a09888",
                        borderBottom: "1px solid #1a1815", verticalAlign: "top",
                        lineHeight: 1.5,
                      }}>{p.leader[row.key]}</td>
                    ))}
                  </tr>
                ))}
                <tr style={{ background: "#1a0a0a" }}>
                  <td style={{
                    padding: "10px 12px", color: "#bf6a6a", fontWeight: 600,
                    borderBottom: "1px solid #1a1815", verticalAlign: "top",
                  }}>Act 3 Challenge</td>
                  {PARTIES.map((p, pi) => (
                    <td key={pi} style={{
                      padding: "10px 12px", color: "#c8a0a0",
                      borderBottom: "1px solid #1a1815", verticalAlign: "top",
                      lineHeight: 1.5,
                    }}>{p.act3challenge}</td>
                  ))}
                </tr>
              </tbody>
            </table>
          </div>
        )}

        {/* Combat Notes View */}
        {view === "combat" && PARTIES.map(p => (
          <div key={p.id} style={{
            background: p.bg, border: `1px solid ${p.border}`,
            borderRadius: 6, padding: "14px 18px", marginBottom: 12,
          }}>
            <div style={{
              fontSize: 14, color: p.color, fontWeight: 600, marginBottom: 10,
            }}>
              {p.name} — Boss Fight
            </div>
            <div style={{
              fontSize: 11, color: "#d4c4b0", marginBottom: 8, lineHeight: 1.6,
            }}>
              <span style={{ color: p.color, fontWeight: 600 }}>Leader: </span>
              {p.leader.combat}
            </div>
            {p.members.map((m, i) => (
              <div key={i} style={{
                background: "#08080c", borderRadius: 3,
                padding: "8px 10px", marginBottom: 4,
                borderLeft: `2px solid ${p.accent}`,
              }}>
                <div style={{ fontSize: 11, color: "#a09888", lineHeight: 1.5 }}>
                  <span style={{ color: "#d4c4b0", fontWeight: 600 }}>{m.name}</span>
                  {" — "}{m.role}
                </div>
                <div style={{
                  fontSize: 10, color: "#bf8a6a", marginTop: 4,
                  fontStyle: "italic",
                }}>
                  \u26A0 {m.note}
                </div>
              </div>
            ))}
          </div>
        ))}

        {/* Summary Footer */}
        <div style={{
          marginTop: 24, padding: 14,
          background: "#0f0e0c", border: "1px solid #1a1815",
          borderRadius: 4, textAlign: "center",
        }}>
          <div style={{ fontSize: 11, color: "#5a5248", lineHeight: 1.8 }}>
            Each traitor gets: 1 Main Quest in Act 0 (meeting) \u2022 1 Main Quest in Act 1 (bonding) \u2022 Act 1.5 betrayal \u2022 Act 3 boss arc from the shadows
          </div>
          <div style={{ fontSize: 10, color: "#3a3530", marginTop: 4 }}>
            One party is randomly selected per run \u2022 All 4 are always designed but only 1 appears
          </div>
        </div>
      </div>
    </div>
  );
}
