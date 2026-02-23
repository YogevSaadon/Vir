# Engine Reference — What the Story JSON Supports

This is the single source of truth for writing story JSON files.
Last updated: 2026-02-21

---

## Story File Structure

```json
{
  "story_id": "unique_id",
  "title": "Display Title",
  "entry_node": "start",
  "required_tags": ["tag_that_must_exist_to_start"],
  "act": 1,
  "summary": "Short journal summary after completion.",
  "story_chain": "optional_chain_id",
  "chain_order": 1,
  "nodes": { ... }
}
```

| Field | Type | Required | Default | Notes |
|-------|------|----------|---------|-------|
| `story_id` | string | YES | — | Unique ID, used everywhere |
| `title` | string | YES | — | Shown in journal |
| `entry_node` | string | YES | — | Key of first node in `nodes` |
| `required_tags` | string[] | no | `[]` | Tags player must have to access this story |
| `act` | int | no | `1` | Which act (1-4) |
| `summary` | string | no | `null` | Journal text after completion |
| `story_chain` | string | no | `null` | Links multi-part stories (e.g. `"aldric_arc"`) |
| `chain_order` | int | no | `1` | Order within chain |
| `nodes` | object | YES | — | Map of node_id → StoryNode |

---

## Story Node

```json
{
  "text": "The narrative text shown to the player.",
  "speaker": "Sir Aldric",
  "background": "backgrounds/dark_forest.png",
  "type": "story_end",
  "choices": [...],
  "rewards": { ... },
  "tags_to_set": ["tag_name"],
  "unlock_stories": ["next_story_id"],
  "allows_shop": false,
  "allows_city": false,
  "shop_id": "village_shop",
  "companion_lines": { "aldric": "I remember this place..." },
  "image": { "id": "scene_forest_ambush", "description": "dark forest, two goblins around a campfire" },
  "scripts": [...]
}
```

| Field | Type | Required | Default | Notes |
|-------|------|----------|---------|-------|
| `text` | string | YES | — | Main narrative text |
| `speaker` | string | no | `null` | Character name shown as speaker |
| `background` | string | no | `null` | Asset path for background image |
| `type` | string | no | `null` | Set to `"story_end"` for final nodes |
| `choices` | Choice[] | no | `[]` | Player choices (empty = auto-advance or end) |
| `rewards` | Rewards | no | `null` | Gold/items/XP given at this node |
| `tags_to_set` | string[] | no | `[]` | Tags added to player's tag set |
| `unlock_stories` | string[] | no | `[]` | Story IDs made available after this node |
| `allows_shop` | bool | no | `false` | Shows SHOP button on this node |
| `allows_city` | bool | no | `false` | Shows CITY button (future) |
| `shop_id` | string | no | `null` | Which shop to open (must exist in shop_inventories.json) |
| `companion_lines` | object | no | `{}` | companion_id → dialogue line |
| `image` | object | no | `null` | `{ "id": "...", "description": "..." }` for art |
| `scripts` | Script[] | no | `[]` | Special scripting (scaffolded, not yet wired) |

---

## Choice

```json
{
  "text": "What the player clicks",
  "next_node": "node_id_to_go_to",
  "requirement": { ... },
  "tags_to_set": ["choice_tag"],
  "trigger_combat": { ... }
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `text` | string | YES | Button text |
| `next_node` | string | YES | Target node ID |
| `requirement` | Requirement | no | If set, choice is hidden/locked when not met |
| `tags_to_set` | string[] | no | Tags added when this choice is picked |
| `trigger_combat` | CombatTrigger | no | Starts a fight |

---

## Requirements (on choices)

Four types, specified by `"type"`:

### Stat check
```json
{ "type": "stat", "stat": "strength", "min_value": 3 }
```
Stats: `strength`, `vitality`, `intelligence`, `faith`, `speed`, `dexterity`, `luck`

### World skill check
```json
{ "type": "world_skill", "skill_id": "climbing", "source": "any" }
```
Source: `"player"`, `"companion"`, or `"any"` (either)

Current world skills: `climbing`, `persuasion`, `lockpicking`

### Tag check
```json
{ "type": "tag", "tag": "met_aldric" }
```

### Item check
```json
{ "type": "item", "item_id": "rusty_sword" }
```

---

## Combat Trigger

```json
{
  "enemy_group": "goblin_camp_weak",
  "on_win": "victory_node_id",
  "on_lose": "defeat_node_id"
}
```

`enemy_group` must match a key in `data/characters/enemies.json`.

---

## Rewards

```json
{
  "gold": 20,
  "items": ["item_id"],
  "exp": 18
}
```

All fields default to `0`/`[]`.

---

## Tags — The Game's Memory

Tags are strings stored in a set. They persist across stories and saves.
Used for: tracking choices, unlocking content, gating choices.

### Conventions
- `met_<character>` — met someone (e.g. `met_aldric`)
- `companion_<id>_joined` — companion joined party
- `chose_<path>` — player chose a specific path
- `completed_<area>` — finished a location/quest
- `was_defeated` — lost a combat
- `god_intro_complete` — finished the god intro

---

## Existing Content IDs

### Stories
| ID | Title | Act |
|----|-------|-----|
| `intro_god` | Voice of a God | 1 |
| `mvp_story_01` | The Forest Path | 1 |
| `mvp_story_02_shop` | The Village Market | 1 |

### Enemy Groups
| ID | Contents |
|----|----------|
| `goblin_camp_weak` | Goblin Warrior + Goblin Scout |

### Companions
| ID | Name | Class | World Skills |
|----|------|-------|--------------|
| `sir_aldric` | Sir Aldric | Knight | climbing |

### Shops
Check `data/items/shop_inventories.json` for available shop IDs.

### Items
| ID | Type | Slot |
|----|------|------|
| `hp_potion_small` | Potion | belt |
| `fire_potion` | Potion | belt |
| `rusty_sword` | Equipment | main_hand |

### Equipment Slots
`helmet`, `necklace`, `armor`, `boots`, `main_hand`, `off_hand`, `ring` (x3)

### World Skills
`climbing`, `persuasion`, `lockpicking`

---

## File Locations

- Stories: `data/stories/<story_id>.json`
- Enemies: `data/characters/enemies.json`
- Companions: `data/characters/companions.json`
- Items: `data/items/potions.json`, `data/items/equipment.json`
- Shops: `data/items/shop_inventories.json`
- Missions: `data/missions/mission_catalog.json`
- Config: `data/config/game_config.json`

## WASM Note

New story files must ALSO be added to the `load_embedded_data()` function
in `vassnian_content/src/loader.rs` as `include_str!` entries.
