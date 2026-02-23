import { useState } from "react";

const ENEMIES = [
  // RANK 1 - Weakest
  { rank: 1, family: "Slime", name: "Slime", role: "Front", hp: 1, dmg: 1, armor: 0, speed: 1, abilities: "Basic melee attack", notes: "Tutorial enemy. Appears in groups.", terrain: "Any" },
  { rank: 1, family: "Slime", name: "Acid Slime", role: "Front", hp: 1, dmg: 1, armor: 0, speed: 1, abilities: "Melee + Poison on hit", notes: "Introduces poison mechanic", terrain: "Swamp/Cave" },
  { rank: 1, family: "Slime", name: "Fire Slime", role: "Front", hp: 1, dmg: 2, armor: 0, speed: 1, abilities: "Melee + Burn on hit", notes: "Fire resistant", terrain: "Volcanic/Ruins" },
  { rank: 1, family: "Wolf", name: "Wolf", role: "Front", hp: 1, dmg: 1, armor: 0, speed: 3, abilities: "Bite, Pack Howl (+ATK when allies present)", notes: "Always in packs of 2-4", terrain: "Forest/Plains" },
  { rank: 1, family: "Wolf", name: "Alpha Wolf", role: "Front", hp: 2, dmg: 1, armor: 1, speed: 3, abilities: "Bite, Rally Pack (buffs all wolves)", notes: "1 per wolf pack, kill first to weaken others", terrain: "Forest/Plains" },
  { rank: 1, family: "Rat", name: "Giant Rat", role: "Front", hp: 1, dmg: 1, armor: 0, speed: 3, abilities: "Bite, Swarm (more rats = more damage)", notes: "Appears in large groups 3-4", terrain: "Cave/Sewer/Ruins" },
  { rank: 1, family: "Rat", name: "Plague Rat", role: "Front", hp: 1, dmg: 1, armor: 0, speed: 3, abilities: "Bite + Disease (DOT)", notes: "Disease stacks, dangerous in numbers", terrain: "Swamp/Sewer" },
  { rank: 1, family: "Insect", name: "Giant Beetle", role: "Front", hp: 2, dmg: 1, armor: 2, speed: 1, abilities: "Pinch, Shell Guard (self armor buff)", notes: "Tanky for rank 1, low threat", terrain: "Forest/Cave" },
  { rank: 1, family: "Insect", name: "Swarm Wasp", role: "Back", hp: 1, dmg: 1, armor: 0, speed: 4, abilities: "Sting (poison), Buzz (reduce accuracy)", notes: "Back row, annoying debuffer", terrain: "Forest/Plains" },
  { rank: 1, family: "Plant", name: "Thorn Vine", role: "Front", hp: 2, dmg: 1, armor: 0, speed: 0, abilities: "Lash, Entangle (slow target)", notes: "Stationary, blocks path", terrain: "Forest/Swamp" },
  { rank: 1, family: "Bat", name: "Cave Bat", role: "Back", hp: 1, dmg: 1, armor: 0, speed: 4, abilities: "Screech (reduce accuracy), Drain Bite (heal self)", notes: "Evasive, low priority target", terrain: "Cave" },

  // RANK 2 - Low
  { rank: 2, family: "Bandit", name: "Bandit", role: "Front", hp: 2, dmg: 2, armor: 1, speed: 2, abilities: "Sword Slash, Steal (removes a consumable)", notes: "Basic human enemy, versatile", terrain: "Road/Forest/Plains" },
  { rank: 2, family: "Bandit", name: "Bandit Archer", role: "Back", hp: 1, dmg: 3, armor: 0, speed: 2, abilities: "Arrow Shot, Aimed Shot (high dmg single target)", notes: "Priority backline target", terrain: "Road/Forest" },
  { rank: 2, family: "Bandit", name: "Bandit Brute", role: "Front", hp: 3, dmg: 2, armor: 2, speed: 1, abilities: "Heavy Swing (AoE front), Intimidate (reduce ATK)", notes: "Frontline tank, protect archers", terrain: "Road/Camp" },
  { rank: 2, family: "Spider", name: "Giant Spider", role: "Front", hp: 2, dmg: 2, armor: 1, speed: 2, abilities: "Poison Bite, Web Shot (immobilize 1 target)", notes: "Crowd control enemy", terrain: "Forest/Cave" },
  { rank: 2, family: "Spider", name: "Web Spinner", role: "Back", hp: 1, dmg: 1, armor: 0, speed: 2, abilities: "Web Trap (AoE slow), Hatchlings (summon 2 spiderlings)", notes: "Summoner type, kill fast", terrain: "Cave/Ruins" },
  { rank: 2, family: "Goblin", name: "Goblin", role: "Front", hp: 1, dmg: 2, armor: 1, speed: 3, abilities: "Stab, Dirty Trick (blind target)", notes: "Weak but annoying, groups of 3-4", terrain: "Forest/Cave/Mountain" },
  { rank: 2, family: "Goblin", name: "Goblin Archer", role: "Back", hp: 1, dmg: 3, armor: 0, speed: 3, abilities: "Arrow Shot, Poison Arrow", notes: "Glass cannon goblin", terrain: "Forest/Cave" },
  { rank: 2, family: "Goblin", name: "Goblin Shaman", role: "Back", hp: 1, dmg: 2, armor: 0, speed: 2, abilities: "Firebolt, Heal Ally, Hex (debuff)", notes: "Priority target, heals others", terrain: "Cave/Camp" },
  { rank: 2, family: "Snake", name: "Viper", role: "Front", hp: 1, dmg: 2, armor: 0, speed: 3, abilities: "Venomous Bite (strong poison), Coil (dodge next attack)", notes: "Strong poison, evasive", terrain: "Swamp/Forest" },
  { rank: 2, family: "Snake", name: "Constrictor", role: "Front", hp: 2, dmg: 2, armor: 1, speed: 2, abilities: "Wrap (immobilize + DOT), Squeeze (bonus dmg to immobilized)", notes: "Locks down 1 target", terrain: "Swamp/Forest" },
  { rank: 2, family: "Mushroom", name: "Fungal Shambler", role: "Front", hp: 3, dmg: 1, armor: 1, speed: 1, abilities: "Spore Cloud (AoE poison), Regenerate", notes: "Tanky, regenerates, poison AoE", terrain: "Forest/Cave/Swamp" },

  // RANK 3 - Mid
  { rank: 3, family: "Skeleton", name: "Skeleton Warrior", role: "Front", hp: 3, dmg: 3, armor: 3, speed: 2, abilities: "Sword Slash, Shield Block (reduce dmg), Bone Rattle (fear)", notes: "Balanced undead frontliner", terrain: "Ruins/Cave/Graveyard" },
  { rank: 3, family: "Skeleton", name: "Skeleton Archer", role: "Back", hp: 2, dmg: 3, armor: 1, speed: 2, abilities: "Bone Arrow, Volley (hit 2 targets)", notes: "Undead ranged, no poison but consistent", terrain: "Ruins/Graveyard" },
  { rank: 3, family: "Skeleton", name: "Skeleton Mage", role: "Back", hp: 1, dmg: 4, armor: 0, speed: 2, abilities: "Shadow Bolt, Drain Life (heal self), Curse (reduce stats)", notes: "Dangerous caster, high priority", terrain: "Ruins/Graveyard" },
  { rank: 3, family: "Goblin", name: "Goblin Warrior", role: "Front", hp: 3, dmg: 3, armor: 2, speed: 3, abilities: "Cleave, War Cry (buff allies ATK), Shield Bash (stun)", notes: "Upgraded goblin, real threat", terrain: "Cave/Mountain" },
  { rank: 3, family: "Goblin", name: "Goblin Bomber", role: "Back", hp: 1, dmg: 4, armor: 0, speed: 2, abilities: "Bomb Toss (AoE), Smoke Bomb (blind AoE), Boom! (self destruct)", notes: "AoE specialist, may suicide", terrain: "Cave/Mountain" },
  { rank: 3, family: "Bandit", name: "Bandit Leader", role: "Front", hp: 3, dmg: 3, armor: 3, speed: 2, abilities: "Command (buff all bandits), Power Strike, Parry", notes: "Buffs other bandits significantly", terrain: "Road/Camp/Town" },
  { rank: 3, family: "Bandit", name: "Bandit Mage", role: "Back", hp: 1, dmg: 4, armor: 0, speed: 2, abilities: "Firebolt, Ice Shard (slow), Barrier (shield ally)", notes: "Versatile caster bandit", terrain: "Camp/Ruins" },
  { rank: 3, family: "Elemental", name: "Mud Elemental", role: "Front", hp: 4, dmg: 2, armor: 4, speed: 1, abilities: "Slam, Mud Splash (slow AoE), Reform (heal when low)", notes: "Very tanky, slow, heals once", terrain: "Swamp/River" },
  { rank: 3, family: "Elemental", name: "Wind Sprite", role: "Back", hp: 1, dmg: 3, armor: 0, speed: 5, abilities: "Gust (knockback), Zephyr Blade (ranged cut), Evasion Aura", notes: "Hard to hit, disrupts formation", terrain: "Plains/Mountain" },
  { rank: 3, family: "Beast", name: "Wild Boar", role: "Front", hp: 3, dmg: 3, armor: 1, speed: 3, abilities: "Charge (high dmg + stun), Gore, Enrage (buff self when hurt)", notes: "Gets more dangerous as HP drops", terrain: "Forest/Plains" },
  { rank: 3, family: "Undead", name: "Zombie", role: "Front", hp: 4, dmg: 2, armor: 0, speed: 1, abilities: "Slam, Grab (immobilize), Rise Again (revive once at 30% HP)", notes: "Revives once, must kill twice", terrain: "Graveyard/Ruins/Swamp" },
  { rank: 3, family: "Undead", name: "Ghoul", role: "Front", hp: 3, dmg: 3, armor: 1, speed: 2, abilities: "Claw, Paralyzing Touch (stun), Feast (heal on kill)", notes: "Stun is very dangerous", terrain: "Graveyard/Ruins" },
  { rank: 3, family: "Harpy", name: "Harpy", role: "Back", hp: 2, dmg: 3, armor: 0, speed: 4, abilities: "Dive Attack (front row), Screech (confuse), Gust Wing (push back)", notes: "Moves between front and back rows", terrain: "Mountain/Cliff" },
  { rank: 3, family: "Lizardfolk", name: "Lizardfolk Scout", role: "Front", hp: 3, dmg: 3, armor: 2, speed: 2, abilities: "Spear Thrust, Tail Whip (knockback), Camouflage (stealth)", notes: "Can ambush, first strike", terrain: "Swamp/River" },

  // RANK 4 - High
  { rank: 4, family: "Cultist", name: "Cultist", role: "Front", hp: 3, dmg: 3, armor: 2, speed: 3, abilities: "Dark Dagger, Shadow Step (teleport behind), Fanaticism (ignore pain)", notes: "Tied to Gorath, gets stronger near bosses", terrain: "Any" },
  { rank: 4, family: "Cultist", name: "Cultist Channeler", role: "Back", hp: 2, dmg: 3, armor: 0, speed: 2, abilities: "Dark Heal, Curse (debuff), Summon Shade (spawn shadow ally)", notes: "Healer + summoner, priority kill", terrain: "Any" },
  { rank: 4, family: "Cultist", name: "Cult Enforcer", role: "Front", hp: 5, dmg: 3, armor: 4, speed: 1, abilities: "Crushing Blow, Shield Wall (protect backline), Zealot Rage (enrage)", notes: "Heavy tank protecting channelers", terrain: "Any" },
  { rank: 4, family: "Dire Beast", name: "Dire Wolf", role: "Front", hp: 3, dmg: 4, armor: 1, speed: 5, abilities: "Savage Bite (bleed), Howl (buff pack), Takedown (stun)", notes: "Alpha predator, often with regular wolves", terrain: "Forest/Mountain" },
  { rank: 4, family: "Dire Beast", name: "Dire Bear", role: "Front", hp: 6, dmg: 4, armor: 3, speed: 1, abilities: "Maul (massive single target), Roar (fear AoE), Thick Hide (DR)", notes: "Wall enemy, massive HP. HP 6 but Speed 1.", terrain: "Forest/Mountain/Cave" },
  { rank: 4, family: "Ogre", name: "Ogre", role: "Front", hp: 5, dmg: 4, armor: 2, speed: 1, abilities: "Club Smash (AoE), Throw Rock (ranged), Stomp (stun AoE)", notes: "Big dumb brute, hits everything hard", terrain: "Mountain/Cave" },
  { rank: 4, family: "Ogre", name: "Ogre Shaman", role: "Back", hp: 3, dmg: 3, armor: 1, speed: 1, abilities: "Earth Spike, Buff Ogre (+STR), Mud Armor (shield ally)", notes: "Buffs ogre frontliners", terrain: "Mountain/Cave" },
  { rank: 4, family: "Golem", name: "Stone Golem", role: "Front", hp: 6, dmg: 3, armor: 6, speed: 0, abilities: "Slam, Stone Skin (armor buff), Crumble (AoE on death)", notes: "Immune to poison/bleed. Must use magic. HP 6 + Armor 6 but Speed 0.", terrain: "Ruins/Mountain" },
  { rank: 4, family: "Wraith", name: "Shadow Wraith", role: "Back", hp: 2, dmg: 4, armor: 0, speed: 4, abilities: "Life Drain, Terror (fear single), Phase (50% dodge physical)", notes: "Half physical damage, use magic", terrain: "Graveyard/Ruins" },
  { rank: 4, family: "Lizardfolk", name: "Lizardfolk Warrior", role: "Front", hp: 4, dmg: 4, armor: 3, speed: 2, abilities: "Heavy Spear, Shield Block, Poison Dart (ranged)", notes: "Well-rounded, switches tactics", terrain: "Swamp" },
  { rank: 4, family: "Lizardfolk", name: "Lizardfolk Shaman", role: "Back", hp: 2, dmg: 3, armor: 1, speed: 2, abilities: "Water Surge (AoE), Regeneration (heal ally), Swamp Curse", notes: "Terrain-based caster", terrain: "Swamp" },
  { rank: 4, family: "Mimic", name: "Mimic", role: "Front", hp: 4, dmg: 4, armor: 3, speed: 3, abilities: "Surprise Attack (bonus first hit), Devour (high single), Loot Drop (good drops)", notes: "Disguised as chest. High reward.", terrain: "Any dungeon" },

  // RANK 5 - Strongest regulars
  { rank: 5, family: "Corrupted", name: "Corrupted Soldier", role: "Front", hp: 4, dmg: 4, armor: 4, speed: 3, abilities: "Dark Slash, Shield Wall, Corrupted Roar (debuff AoE), Undying (revive once)", notes: "Former Caelmund soldiers twisted by Gorath", terrain: "Any" },
  { rank: 5, family: "Corrupted", name: "Corrupted Beast", role: "Front", hp: 5, dmg: 5, armor: 2, speed: 4, abilities: "Rampage (random multi-hit), Mutate (gain new ability mid-fight), Frenzy", notes: "Unpredictable attack patterns", terrain: "Any" },
  { rank: 5, family: "Corrupted", name: "Corrupted Mage", role: "Back", hp: 2, dmg: 6, armor: 0, speed: 3, abilities: "Shadow Blast (AoE), Void Bolt (ignore armor), Dark Shield (absorb dmg)", notes: "Highest magic dmg in Act 1. DMG 6 but HP 2 + Armor 0.", terrain: "Any" },
  { rank: 5, family: "Troll", name: "Troll", role: "Front", hp: 5, dmg: 4, armor: 3, speed: 1, abilities: "Club Smash, Regenerate (heal each turn), Throw Boulder (ranged)", notes: "Must burst down or out-damage regen. Fire stops regen.", terrain: "Mountain/Forest/Cave" },
  { rank: 5, family: "Troll", name: "Troll Berserker", role: "Front", hp: 5, dmg: 6, armor: 1, speed: 2, abilities: "Frenzy (multi-hit), Blood Rage (more dmg when hurt), Regenerate", notes: "DPS race. DMG 6 but Armor 1.", terrain: "Mountain/Cave" },
  { rank: 5, family: "Undead", name: "Revenant", role: "Front", hp: 5, dmg: 4, armor: 4, speed: 3, abilities: "Cursed Blade (bleed+curse), Death Grip (immobilize), Undying Will (revive twice)", notes: "Mini-boss tier. Revives twice.", terrain: "Graveyard/Ruins" },
  { rank: 5, family: "Elemental", name: "Fire Elemental", role: "Back", hp: 3, dmg: 6, armor: 0, speed: 4, abilities: "Fireball (AoE), Flame Aura (dmg on melee contact), Immolate (big single)", notes: "Punishes melee. DMG 6 but Armor 0.", terrain: "Volcanic/Ruins" },
  { rank: 5, family: "Knight", name: "Dark Knight", role: "Front", hp: 5, dmg: 4, armor: 5, speed: 2, abilities: "Heavy Strike, Dark Aura (debuff AoE), Challenge (force target), Last Stand", notes: "Mini-boss tier. Full kit tank+damage.", terrain: "Ruins/Road" },
];

const ENCOUNTER_FORMULA = "Encounter Power = (Sum of all enemy ranks) \u00f7 4 (always divide by 4, even if fewer than 4 enemies)";

const rankColors = {
  1: { bg: "#1a2e1a", border: "#2d5a2d", text: "#6abf6a", label: "Fodder" },
  2: { bg: "#2e2a1a", border: "#5a4d2d", text: "#bfa86a", label: "Low" },
  3: { bg: "#2e1e1a", border: "#5a3a2d", text: "#bf7a6a", label: "Mid" },
  4: { bg: "#2a1a2e", border: "#4d2d5a", text: "#9a6abf", label: "High" },
  5: { bg: "#2e1a1a", border: "#5a2d2d", text: "#bf4a4a", label: "Elite" },
};

const tierColor = (t) => {
  if (t === 0) return "#3a3530";
  if (t <= 1) return "#4a7a4a";
  if (t <= 2) return "#7a9a4a";
  if (t <= 3) return "#bfa86a";
  if (t <= 4) return "#bf7a4a";
  if (t <= 5) return "#bf4a4a";
  return "#ff3a6a";
};

const roleIcons = { Front: "\u2694\ufe0f", Back: "\ud83c\udff9" };

const families = [...new Set(ENEMIES.map(e => e.family))].sort();

export default function Bestiary() {
  const [selectedRank, setSelectedRank] = useState(null);
  const [selectedFamily, setSelectedFamily] = useState(null);
  const [expandedEnemy, setExpandedEnemy] = useState(null);

  const filtered = ENEMIES.filter(e => {
    if (selectedRank && e.rank !== selectedRank) return false;
    if (selectedFamily && e.family !== selectedFamily) return false;
    return true;
  });

  const countByRank = (r) => ENEMIES.filter(e => e.rank === r).length;
  const countByFamily = (f) => ENEMIES.filter(e => e.family === f).length;

  return (
    <div style={{
      minHeight: "100vh",
      background: "#0a0a0f",
      color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 960, margin: "0 auto" }}>
        <div style={{
          textAlign: "center",
          marginBottom: 32,
          borderBottom: "1px solid #2a2520",
          paddingBottom: 24,
        }}>
          <h1 style={{
            fontSize: 28,
            color: "#d4a574",
            fontWeight: 400,
            letterSpacing: 4,
            textTransform: "uppercase",
            margin: 0,
          }}>
            \u2620 Vassnian Bestiary \u2620
          </h1>
          <p style={{ color: "#6a6058", fontSize: 13, marginTop: 8 }}>
            Act 1 \u2022 Ranks 1\u20135 \u2022 {ENEMIES.length} Enemy Types \u2022 Stats: Tier 0\u20135 (6+ possible)
          </p>
          <p style={{
            color: "#8a7a6a",
            fontSize: 11,
            marginTop: 12,
            background: "#12110f",
            display: "inline-block",
            padding: "6px 16px",
            borderRadius: 4,
            border: "1px solid #2a2520",
            fontFamily: "monospace",
          }}>
            {ENCOUNTER_FORMULA}
          </p>
        </div>

        <div style={{ marginBottom: 16 }}>
          <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 8, textTransform: "uppercase", letterSpacing: 2 }}>Filter by Rank</div>
          <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
            <button
              onClick={() => setSelectedRank(null)}
              style={{
                background: !selectedRank ? "#2a2520" : "transparent",
                border: `1px solid ${!selectedRank ? "#d4a574" : "#2a2520"}`,
                color: !selectedRank ? "#d4a574" : "#6a6058",
                padding: "6px 14px",
                borderRadius: 4,
                cursor: "pointer",
                fontSize: 12,
                fontFamily: "inherit",
              }}
            >
              All ({ENEMIES.length})
            </button>
            {[1,2,3,4,5].map(r => (
              <button
                key={r}
                onClick={() => setSelectedRank(selectedRank === r ? null : r)}
                style={{
                  background: selectedRank === r ? rankColors[r].bg : "transparent",
                  border: `1px solid ${selectedRank === r ? rankColors[r].border : "#2a2520"}`,
                  color: selectedRank === r ? rankColors[r].text : "#6a6058",
                  padding: "6px 14px",
                  borderRadius: 4,
                  cursor: "pointer",
                  fontSize: 12,
                  fontFamily: "inherit",
                }}
              >
                Rank {r} \u2014 {rankColors[r].label} ({countByRank(r)})
              </button>
            ))}
          </div>
        </div>

        <div style={{ marginBottom: 24 }}>
          <div style={{ fontSize: 11, color: "#6a6058", marginBottom: 8, textTransform: "uppercase", letterSpacing: 2 }}>Filter by Family</div>
          <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
            <button
              onClick={() => setSelectedFamily(null)}
              style={{
                background: !selectedFamily ? "#2a2520" : "transparent",
                border: `1px solid ${!selectedFamily ? "#d4a574" : "#1a1815"}`,
                color: !selectedFamily ? "#d4a574" : "#5a5248",
                padding: "4px 10px",
                borderRadius: 3,
                cursor: "pointer",
                fontSize: 11,
                fontFamily: "inherit",
              }}
            >
              All
            </button>
            {families.map(f => (
              <button
                key={f}
                onClick={() => setSelectedFamily(selectedFamily === f ? null : f)}
                style={{
                  background: selectedFamily === f ? "#2a2520" : "transparent",
                  border: `1px solid ${selectedFamily === f ? "#d4a574" : "#1a1815"}`,
                  color: selectedFamily === f ? "#d4a574" : "#5a5248",
                  padding: "4px 10px",
                  borderRadius: 3,
                  cursor: "pointer",
                  fontSize: 11,
                  fontFamily: "inherit",
                }}
              >
                {f} ({countByFamily(f)})
              </button>
            ))}
          </div>
        </div>

        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          {filtered.map((e, i) => {
            const rc = rankColors[e.rank];
            const key = `${e.rank}-${e.name}`;
            const isExpanded = expandedEnemy === key;
            return (
              <div
                key={key}
                onClick={() => setExpandedEnemy(isExpanded ? null : key)}
                style={{
                  background: isExpanded ? rc.bg : "#0f0e0c",
                  border: `1px solid ${isExpanded ? rc.border : "#1a1815"}`,
                  borderRadius: 4,
                  padding: "10px 14px",
                  cursor: "pointer",
                  transition: "all 0.15s ease",
                }}
              >
                <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
                  <span style={{
                    fontSize: 10,
                    color: rc.text,
                    background: rc.bg,
                    border: `1px solid ${rc.border}`,
                    padding: "2px 6px",
                    borderRadius: 3,
                    fontFamily: "monospace",
                    minWidth: 18,
                    textAlign: "center",
                  }}>
                    R{e.rank}
                  </span>
                  <span style={{ fontSize: 14 }}>{roleIcons[e.role]}</span>
                  <span style={{ fontSize: 14, color: "#d4c4b0", fontWeight: 600, flex: 1 }}>
                    {e.name}
                  </span>
                  <div style={{ display: "flex", gap: 4 }}>
                    {[
                      { label: "HP", val: e.hp },
                      { label: "D", val: e.dmg },
                      { label: "A", val: e.armor },
                      { label: "S", val: e.speed },
                    ].map(s => (
                      <span key={s.label} style={{
                        fontSize: 10,
                        fontFamily: "monospace",
                        color: tierColor(s.val),
                        background: "#08080a",
                        padding: "1px 4px",
                        borderRadius: 2,
                      }}>
                        {s.label}{s.val}
                      </span>
                    ))}
                  </div>
                  <span style={{ fontSize: 11, color: "#4a4238" }}>{e.family}</span>
                </div>

                {isExpanded && (
                  <div style={{ marginTop: 12, paddingTop: 10, borderTop: `1px solid ${rc.border}` }}>
                    <div style={{
                      display: "grid",
                      gridTemplateColumns: "repeat(4, 1fr)",
                      gap: 8,
                      marginBottom: 10,
                    }}>
                      {[
                        { label: "HP", value: e.hp },
                        { label: "DMG", value: e.dmg },
                        { label: "ARMOR", value: e.armor },
                        { label: "SPEED", value: e.speed },
                      ].map(s => (
                        <div key={s.label} style={{
                          background: "#08080a",
                          padding: "8px",
                          borderRadius: 3,
                          textAlign: "center",
                        }}>
                          <div style={{ fontSize: 9, color: "#5a5248", textTransform: "uppercase", letterSpacing: 1 }}>{s.label}</div>
                          <div style={{
                            fontSize: 20,
                            fontWeight: 700,
                            color: tierColor(s.value),
                            marginTop: 2,
                            fontFamily: "monospace",
                          }}>
                            {s.value}
                          </div>
                          <div style={{ fontSize: 9, color: "#3a3530", marginTop: 2 }}>
                            {s.value === 0 ? "NONE" : s.value >= 6 ? "OVER MAX" : `TIER ${s.value}/5`}
                          </div>
                        </div>
                      ))}
                    </div>
                    <div style={{ marginBottom: 6 }}>
                      <span style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 1 }}>Abilities: </span>
                      <span style={{ fontSize: 12, color: "#b0a898" }}>{e.abilities}</span>
                    </div>
                    <div style={{ display: "flex", gap: 16, marginBottom: 6 }}>
                      <div>
                        <span style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 1 }}>Role: </span>
                        <span style={{ fontSize: 12, color: "#b0a898" }}>{e.role} Row</span>
                      </div>
                      <div>
                        <span style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 1 }}>Terrain: </span>
                        <span style={{ fontSize: 12, color: "#b0a898" }}>{e.terrain}</span>
                      </div>
                    </div>
                    <div style={{
                      fontSize: 11,
                      color: "#8a7a6a",
                      fontStyle: "italic",
                      marginTop: 4,
                      padding: "6px 8px",
                      background: "#08080a",
                      borderRadius: 3,
                      borderLeft: `2px solid ${rc.border}`,
                    }}>
                      {e.notes}
                    </div>
                  </div>
                )}
              </div>
            );
          })}
        </div>

        <div style={{
          marginTop: 32,
          padding: 20,
          background: "#0f0e0c",
          border: "1px solid #1a1815",
          borderRadius: 4,
        }}>
          <h3 style={{ color: "#d4a574", fontSize: 14, fontWeight: 400, letterSpacing: 2, textTransform: "uppercase", marginTop: 0 }}>
            Tier System
          </h3>
          <div style={{ display: "flex", gap: 12, flexWrap: "wrap", marginBottom: 16 }}>
            {[0,1,2,3,4,5,6].map(t => (
              <div key={t} style={{ display: "flex", alignItems: "center", gap: 6 }}>
                <span style={{
                  fontFamily: "monospace",
                  fontSize: 16,
                  fontWeight: 700,
                  color: tierColor(t),
                }}>{t}</span>
                <span style={{ fontSize: 11, color: "#5a5248" }}>
                  {t === 0 ? "None" : t === 1 ? "Minimal" : t === 2 ? "Low" : t === 3 ? "Medium" : t === 4 ? "High" : t === 5 ? "Max" : "Over Max"}
                </span>
              </div>
            ))}
          </div>
          <p style={{ fontSize: 11, color: "#6a6058", margin: 0 }}>
            Enemies with Tier 6+ in a stat must have lower tiers elsewhere to compensate (e.g. Stone Golem: HP 6, Armor 6, Speed 0).
          </p>
        </div>

        <div style={{
          marginTop: 16,
          padding: 20,
          background: "#0f0e0c",
          border: "1px solid #1a1815",
          borderRadius: 4,
        }}>
          <h3 style={{ color: "#d4a574", fontSize: 14, fontWeight: 400, letterSpacing: 2, textTransform: "uppercase", marginTop: 0 }}>
            Act 1 Distribution
          </h3>
          <div style={{ fontSize: 12, color: "#8a7a6a", lineHeight: 1.8 }}>
            <div>Sub-act 1.1 (Lvl 1-2): Rank 1-2 \u2014 Slimes, Rats, Wolves, Bats, Insects, Bandits, Goblins</div>
            <div>Sub-act 1.2 (Lvl 2-3): Rank 2-3 \u2014 Upgraded Goblins/Bandits, Skeletons, Undead, Elementals, Harpies</div>
            <div>Sub-act 1.3 (Lvl 4-5): Rank 3-5 \u2014 Cultists, Dire Beasts, Ogres, Trolls, Corrupted, Dark Knights</div>
            <div style={{ marginTop: 12, color: "#6a6058", fontStyle: "italic" }}>
              Boss-themed enemies layer ON TOP during sub-act events. Regular enemies are constant across all boss selections.
            </div>
          </div>
        </div>

        <div style={{
          marginTop: 16,
          padding: 20,
          background: "#0f0e0c",
          border: "1px solid #1a1815",
          borderRadius: 4,
        }}>
          <h3 style={{ color: "#d4a574", fontSize: 14, fontWeight: 400, letterSpacing: 2, textTransform: "uppercase", marginTop: 0 }}>
            {families.length} Enemy Families
          </h3>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
            {families.map(f => {
              const members = ENEMIES.filter(e => e.family === f);
              const minR = Math.min(...members.map(m => m.rank));
              const maxR = Math.max(...members.map(m => m.rank));
              return (
                <div key={f} style={{
                  background: "#08080a",
                  padding: "6px 12px",
                  borderRadius: 3,
                  border: "1px solid #1a1815",
                  fontSize: 11,
                }}>
                  <span style={{ color: "#b0a898" }}>{f}</span>
                  <span style={{ color: "#4a4238", marginLeft: 6 }}>
                    {members.length} types \u2022 R{minR}{maxR !== minR ? `\u2013${maxR}` : ""}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
