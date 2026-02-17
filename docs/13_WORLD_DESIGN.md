# Vassnian — 13 World Design & Content Systems

## Systems that define HOW content is organized, NOT the content itself.

---

## 1. Mission / Quest System

### Mission Selection
Every time the player goes on a mission, they choose from **3 options**:

```
┌──────────────────────────────┐
│  Choose Your Next Mission:   │
│                              │
│ ┌──────────────────────────┐ │
│ │ 🌲 Dark Forest Patrol    │ │
│ │ Difficulty: ★★☆          │ │
│ │ Time: 2 units             │ │
│ │ "Strange sounds from the  │ │
│ │  northern forest..."      │ │
│ └──────────────────────────┘ │
│ ┌──────────────────────────┐ │
│ │ 👤 Sir Aldric's Past     │ │  ← companion story (portrait shown)
│ │ Difficulty: ★☆☆          │ │
│ │ Time: 1 unit              │ │
│ │ "Aldric wants to visit a  │ │
│ │  place from his past..."  │ │
│ └──────────────────────────┘ │
│ ┌──────────────────────────┐ │
│ │ ⚔️ Bandit Stronghold     │ │
│ │ Difficulty: ★★★          │ │
│ │ Time: 3 units             │ │
│ │ "A heavily fortified camp │ │
│ │  with rumors of treasure" │ │
│ └──────────────────────────┘ │
│                              │
└──────────────────────────────┘
```

**Rules:**
- Difficulty shown by stars or time cost
- Companion stories marked with companion portrait/name
- Only 1 companion story per selection round
- **If you skip a companion story, it's gone forever**
- Unchosen missions may or may not reappear

### Mission Data Structure
```rust
pub struct MissionDef {
    pub id: String,
    pub title: String,
    pub description: String,          // short summary
    pub phase: GamePhase,             // which phase this belongs to
    pub difficulty: MissionDifficulty,
    pub time_cost: i32,               // abstract time units
    pub terrain: Terrain,
    pub is_companion_story: Option<String>,  // companion_id
    pub has_special_item: bool,
    pub story_id: String,             // which story JSON to load
    pub continuation_of: Option<String>,     // previous mission ID
    pub continues_to: Option<Vec<String>>,   // next mission IDs on completion
    pub is_required: bool,            // mandatory event?
}
```

### Mission Catalog File
Create: `data/missions/mission_catalog.json`
```json
{
  "missions": [
    {
      "id": "m_ph0_king_intro",
      "title": "The Summoning",
      "description": "The King addresses the assembled Godsents.",
      "phase": "phase_0",
      "difficulty": "none",
      "time_cost": 0,
      "terrain": "city",
      "is_required": true,
      "story_id": "story_king_intro"
    },
    {
      "id": "m_ph1_forest_patrol",
      "title": "Dark Forest Patrol",
      "description": "Strange sounds from the northern forest. Investigate.",
      "phase": "phase_1",
      "difficulty": "medium",
      "time_cost": 2,
      "terrain": "forest",
      "is_required": false,
      "has_special_item": false,
      "story_id": "story_forest_patrol"
    }
  ]
}
```

> **For now:** Only create the schema and a few placeholder entries. Real mission list comes after world is built.

---

## 2. Enemy Types & Levels

### Enemy Classification
Every enemy has:
- **Level** — for power scaling and random assignment
- **Creature types** — for terrain matching (forest beast, undead, demon, etc.)
- **Phase range** — which phases they appear in

```json
{
  "id": "forest_wolf",
  "name": "Forest Wolf",
  "level": 2,
  "enemy_types": ["forest", "beast"],
  "phase_range": [0, 1],
  "stats": { "strength": 3, "vitality": 2, "speed": 4 }
}
```

### Terrain → Enemy Matching
When generating encounters for a mission:
1. Look at mission terrain
2. Pull enemies whose `enemy_types` match that terrain
3. Filter by `phase_range` matching current phase
4. Scale stats by level if needed

| Terrain | Enemy Types That Appear |
|---------|------------------------|
| Forest | beast, forest, bandit |
| Mountain | beast, dragon, giant |
| Underground | undead, demon, dark_mage |
| ShadowWorld | all types (more dangerous variants) |
| City | bandit, rogue, corrupt_guard |

---

## 3. Item System (Expanded)

### Item Types

| Type | Has Stats | Has Level | Sellable | Example |
|------|-----------|-----------|----------|---------|
| Weapon | ✅ damage, stat bonuses | ✅ | ✅ | Rusty Sword |
| Armor | ✅ defense | ✅ | ✅ | Leather Vest |
| Accessory | ✅ stat bonuses | ✅ | ✅ | Lucky Ring |
| Potion | ✅ effect amount | ✅ | ✅ | HP Potion |
| Key Item | ❌ description only | ❌ | ❌ | Tower Key |
| Crafting Material | ❌ description + destination | ❌ | ❌ | Strange Magic Orb |
| Skill Book | ❌ skill it teaches | ❌ | ✅ | Book of Fireball |

### Regular vs Special Items

| Category | Source | Availability |
|----------|--------|-------------|
| **Regular** | Random mission drops, shop stock | Common, repeatable |
| **Special** | Mini-boss drops, quest rewards, unique finds | Rare, often one-of-a-kind |
| **Crafting** | Mission finds, Shadow World exploration | Bring to Blacksmith/Mage in city |

### Item Level
Used for random shop generation:
- Shop stocks items at or near the current world level
- Higher level = better stats
- Level also used for loot table generation in missions

---

## 4. Terrain System

### Terrains
```
Forest, Mountain, Desert, Swamp, Underground, City, ShadowWorld, Plains
```

### Terrain Effects
- **Enemy spawns:** Terrain filters which enemies appear
- **Ranger bonus:** If Ranger chose this terrain, they get:
  - Combat: damage bonus in this terrain
  - Story: special choice options in this terrain
- **Visual:** Different background images per terrain
- **Future:** Weather/environmental effects

### Per-Mission Terrain
Each mission has exactly one terrain type, set in the mission catalog.

---

## 5. City Services

### Main City Layout (Conceptual)
The city is always accessible when in the overworld (costs time to visit). Contains:

| Service | Available From | Unlocked By |
|---------|---------------|-------------|
| **General Shop** | Phase 0 | Always |
| **Blacksmith** | Phase 0 (empty) → Phase 1+ | Finding crafting materials |
| **Mage** | Phase 0 (empty) → Phase 1+ | Finding magic items |
| **Inn** | Phase 0 | Always (heals? saves?) |
| **Bar** | Phase 0 | Where companion stories happen |

### Blacksmith
- Initially: no options
- Finding specific items → unlocks recipes
- Phase 3: can craft equipment from rare Shadow World materials
- Crafting = bring materials + gold → get unique equipment

### Mage
- Initially: no options
- Finding "strange magic orb" type items → bring here
- Mage crafts special items (enchantments, magic weapons, etc.)

### Trainers (In Missions, NOT in city)
- Found during missions, not in the main city
- Teach specific skills for a gold price
- Requirements: gold + stat requirements
- Appear as a story choice → opens skill purchase panel

---

## 6. Shop Access Rules (Summary)

| Context | Shop Available? | How? |
|---------|----------------|------|
| Overworld, between missions | ✅ Always | Shop button always visible, costs time |
| During overworld mission | ❌ Not until mission complete | — |
| Shadow World, during mission | ❌ Usually not | — |
| Shadow World, underground city | ✅ Yes | Shop button appears |
| Shadow World, meet merchant | ✅ Yes | Shop button appears |

The `allows_shop` flag in story node data controls this.

---

## 7. Time System

### How Time Works
- Every mission costs **time units**
- More difficult missions cost more time
- After enough time accumulates → phase advances
- Returning to city also costs time
- This creates tension: do you go back to shop/heal, or push forward?

### Time Thresholds (Tuning — Not Decided)
```json
{
  "phase_thresholds": {
    "phase_0_to_1": 5,
    "phase_1_to_1.5": 20,
    "phase_2_to_2.5": 30,
    "phase_3_to_4": 25
  }
}
```
*Exact values TBD during balancing.*

---

## 8. Godsent Groups

Other summoned hero groups you encounter throughout the game. Tracked as world tags.

### Group Story Types

| Type | Phase | Description |
|------|-------|-------------|
| Friendly group | 1 | Help them → they return later |
| Dead group | 2 | Find corpses in Shadow World, loot gear |
| Stuck group | 2 | Meet in underground city, shared stories |
| Rival group | 1-3 | Competitive, might help or hinder |
| Strong group | 4 | Allied reinforcements in final battle |

Each group encounter sets tags:
```
godsent_group_{id}_met
godsent_group_{id}_helped
godsent_group_{id}_alive
```

Multi-part chains: helping a group in Phase 1 → they appear in Shadow World → they help in Phase 4.
