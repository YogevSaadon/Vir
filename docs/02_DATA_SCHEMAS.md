# Vassnian — Data Schemas

All game content is stored as JSON in `/data/`. This document defines every schema.

---

## 1. Character / Class Definition

**File:** `data/characters/classes.json`

```json
{
  "classes": [
    {
      "id": "knight",
      "name": "Knight",
      "description": "Team protector — stands at the front and protects everyone.",
      "category": "warrior",
      "position": "front",
      "resource_type": "cooldown",
      "base_stats": {
        "strength": 3,
        "vitality": 4,
        "intelligence": 1,
        "faith": 1,
        "speed": 2,
        "dexterity": 2,
        "luck": 1
      },
      "starting_combat_skills": [],
      "starting_world_skills": [],
      "portrait": "characters/player_knight.png",
      "attack_type": "melee",
      "flavor_text": "A wall of steel between your allies and death."
    }
  ]
}
```

**Rust struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ClassCategory,
    pub position: Position,
    pub resource_type: ResourceType,
    pub base_stats: StatBlock,
    pub starting_combat_skills: Vec<String>,  // skill IDs
    pub starting_world_skills: Vec<String>,   // skill IDs
    pub portrait: String,
    pub attack_type: AttackType,
    pub flavor_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassCategory { Warrior, Caster, Rogue, Healer }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position { Front, Back, Flexible }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType { Cooldown, Mana, ManaCooldown }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackType { Melee, Ranged }
```

---

## 2. Stat Block

Used everywhere a character has stats.

```json
{
  "strength": 1,
  "vitality": 1,
  "intelligence": 1,
  "faith": 1,
  "speed": 1,
  "dexterity": 1,
  "luck": 1
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatBlock {
    pub strength: i32,
    pub vitality: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub speed: i32,
    pub dexterity: i32,
    pub luck: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedStats {
    pub max_hp: i32,        // base from vitality
    pub max_mana: i32,      // base from intelligence
    pub physical_damage: i32, // base from strength
    pub magic_damage: i32,   // base from intelligence
    pub defense: i32,        // base from vitality
    pub evasion: f32,        // base from dexterity
    pub crit_chance: f32,    // base from luck + dexterity
    pub atb_speed: f32,      // base from speed
}
```

---

## 3. Combat Skills

**File:** `data/skills/combat_skills.json`

```json
{
  "combat_skills": [
    {
      "id": "taunt",
      "name": "Taunt",
      "description": "Forces enemies to attack you for 2 turns.",
      "class_origin": "knight",
      "type": "active",
      "speed": { "type": "normal" },
      "resource_cost": null,
      "cooldown_turns": 3,
      "can_be_interrupted": false,
      "target": "self",
      "effects": [
        { "type": "apply_status", "status": "taunt", "duration": 2 }
      ],
      "stat_requirements": { "strength": 3 },
      "icon": "skills/taunt.png"
    },
    {
      "id": "stun",
      "name": "Stun",
      "description": "Freezes an enemy's ATB bar.",
      "class_origin": "knight",
      "type": "active",
      "speed": { "type": "normal" },
      "resource_cost": null,
      "cooldown_turns": 3,
      "can_be_interrupted": false,
      "target": "single_enemy",
      "effects": [
        { "type": "freeze_atb", "duration": 2.0 }
      ],
      "stat_requirements": { "strength": 2, "vitality": 2 },
      "icon": "skills/stun.png"
    }
  ]
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkillDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub class_origin: String,
    pub skill_type: SkillType,
    pub resource_cost: Option<i32>,
    #[serde(default)]
    pub cooldown_turns: i32,       // turns until reusable (0 = no cooldown)
    #[serde(default = "default_normal_speed")]
    pub speed: SkillSpeed,         // Instant / Normal / CastTime
    #[serde(default)]
    pub can_be_interrupted: bool,  // only matters for CastTime
    pub target: TargetType,
    pub effects: Vec<SkillEffect>,
    pub stat_requirements: Option<StatBlock>,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillType { Active, Passive, Toggle }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SkillSpeed {
    Instant,                    // No ATB cost, long CD
    Normal,                     // Standard ATB turn (default)
    CastTime { seconds: f32 },  // Starts on turn, resolves after delay
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Self_,
    SingleAlly,
    SingleEnemy,
    AllAllies,
    AllEnemies,
    AllFrontEnemies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SkillEffect {
    Damage { base: i32, scaling_stat: String, multiplier: f32 },
    Heal { base: i32, scaling_stat: String, multiplier: f32 },
    ApplyStatus { status: String, duration: i32 },
    FreezeAtb { duration: f32 },
    RemoveStatus { status: String },
    ModifyStat { stat: String, amount: i32, duration: i32 },
}
```

---

## 4. World Skills

**File:** `data/skills/world_skills.json`

```json
{
  "world_skills": [
    {
      "id": "climbing",
      "name": "Climbing",
      "description": "Scale walls, cliffs, and obstacles.",
      "icon": "skills/climbing.png"
    },
    {
      "id": "persuasion",
      "name": "Persuasion",
      "description": "Convince others through words and charm.",
      "icon": "skills/persuasion.png"
    },
    {
      "id": "lockpicking",
      "name": "Lockpicking",
      "description": "Open locked doors and chests.",
      "icon": "skills/lockpicking.png"
    }
  ]
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSkillDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}
```

---

## 5. Story Data

**File:** `data/stories/mvp_story_01.json`

Stories are node graphs. Each node has text and choices that lead to other nodes.

```json
{
  "story_id": "mvp_story_01",
  "title": "The Forest Path",
  "entry_node": "start",
  "required_tags": [],
  "nodes": {
    "start": {
      "text": "You wake up in a dark forest. The trees loom above you, their branches clawing at a bruised sky. A narrow path splits in two directions.",
      "speaker": null,
      "background": "backgrounds/dark_forest.png",
      "choices": [
        {
          "text": "Take the left path through the thorns.",
          "next_node": "thorns_path",
          "requirement": null,
          "tags_to_set": ["chose_thorns"]
        },
        {
          "text": "Scale the cliff to get a better view.",
          "next_node": "cliff_view",
          "requirement": {
            "type": "world_skill",
            "skill_id": "climbing",
            "source": "any"
          },
          "tags_to_set": ["climbed_cliff"]
        },
        {
          "text": "Push through the boulders blocking the right path.",
          "next_node": "boulder_path",
          "requirement": {
            "type": "stat",
            "stat": "strength",
            "min_value": 3
          },
          "tags_to_set": ["used_strength"]
        }
      ]
    },
    "cliff_view": {
      "text": "From above, you spot a camp of goblins. They haven't seen you yet.",
      "speaker": null,
      "background": "backgrounds/cliff_view.png",
      "choices": [
        {
          "text": "Sneak past them.",
          "next_node": "sneak_past",
          "requirement": null,
          "tags_to_set": ["avoided_goblins"]
        },
        {
          "text": "Attack while they're unaware!",
          "next_node": "ambush_combat",
          "requirement": null,
          "tags_to_set": ["fought_goblins"],
          "trigger_combat": {
            "enemy_group": "goblin_camp_weak",
            "on_win": "goblin_victory",
            "on_lose": "goblin_defeat"
          }
        }
      ]
    },
    "goblin_victory": {
      "text": "The goblins fall. You find a small pouch of gold among their belongings.",
      "speaker": null,
      "rewards": {
        "gold": 20,
        "items": []
      },
      "choices": [
        {
          "text": "Continue deeper into the forest.",
          "next_node": "end",
          "requirement": null,
          "tags_to_set": []
        }
      ]
    },
    "end": {
      "text": "The forest thins, and you see a small village in the distance.",
      "speaker": null,
      "type": "story_end",
      "tags_to_set": ["completed_forest_path"],
      "unlock_stories": ["mvp_story_02_shop"]
    }
  }
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryDef {
    pub story_id: String,
    pub title: String,
    pub entry_node: String,
    pub required_tags: Vec<String>,
    pub nodes: HashMap<String, StoryNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    pub text: String,
    pub speaker: Option<String>,
    pub background: Option<String>,
    #[serde(default)]
    pub choices: Vec<StoryChoice>,
    pub rewards: Option<Rewards>,
    #[serde(rename = "type")]
    pub node_type: Option<String>,  // "story_end", etc.
    #[serde(default)]
    pub tags_to_set: Vec<String>,
    #[serde(default)]
    pub unlock_stories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChoice {
    pub text: String,
    pub next_node: String,
    pub requirement: Option<ChoiceRequirement>,
    #[serde(default)]
    pub tags_to_set: Vec<String>,
    pub trigger_combat: Option<CombatTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChoiceRequirement {
    #[serde(rename = "stat")]
    Stat { stat: String, min_value: i32 },
    #[serde(rename = "world_skill")]
    WorldSkill { skill_id: String, source: String },  // "player", "companion", "any"
    #[serde(rename = "tag")]
    Tag { tag: String },
    #[serde(rename = "item")]
    Item { item_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatTrigger {
    pub enemy_group: String,
    pub on_win: String,   // next node on win
    pub on_lose: String,  // next node on lose
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rewards {
    #[serde(default)]
    pub gold: i32,
    #[serde(default)]
    pub items: Vec<String>,
    #[serde(default)]
    pub exp: i32,
}
```

---

## 6. Items

### Potions — `data/items/potions.json`

```json
{
  "potions": [
    {
      "id": "hp_potion_small",
      "name": "Small Health Potion",
      "description": "Restores 30 HP to an ally.",
      "target": "single_ally",
      "effect": { "type": "heal", "amount": 30 },
      "icon": "skills/hp_potion.png",
      "buy_price": 10,
      "sell_price": 5
    },
    {
      "id": "fire_potion",
      "name": "Fire Potion",
      "description": "Deals 25 fire damage to an enemy.",
      "target": "single_enemy",
      "effect": { "type": "damage", "amount": 25, "element": "fire" },
      "icon": "skills/fire_potion.png",
      "buy_price": 15,
      "sell_price": 7
    }
  ]
}
```

### Equipment — `data/items/equipment.json`

```json
{
  "equipment": [
    {
      "id": "rusty_sword",
      "name": "Rusty Sword",
      "description": "A dull blade, but better than bare fists.",
      "slot": "weapon",
      "weapon_type": "sword",
      "attack_type": "melee",
      "stat_bonuses": { "strength": 2 },
      "icon": "items/rusty_sword.png",
      "buy_price": 25,
      "sell_price": 10
    }
  ]
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target: TargetType,
    pub effect: PotionEffect,
    pub icon: String,
    pub buy_price: i32,
    pub sell_price: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PotionEffect {
    Heal { amount: i32 },
    Damage { amount: i32, element: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slot: EquipSlot,
    pub weapon_type: Option<String>,
    pub attack_type: Option<AttackType>,
    pub stat_bonuses: StatBlock,
    pub icon: String,
    pub buy_price: i32,
    pub sell_price: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquipSlot { Weapon, Armor, Accessory }
```

---

## 7. Shop Inventory

**File:** `data/items/shop_inventories.json`

```json
{
  "shops": {
    "village_shop": {
      "name": "Village General Store",
      "shopkeeper": "Old Merchant",
      "items": [
        { "item_id": "hp_potion_small", "item_type": "potion", "stock": 5 },
        { "item_id": "rusty_sword", "item_type": "equipment", "stock": 1 }
      ]
    }
  }
}
```

---

## 8. Enemy Groups

**File:** `data/characters/enemies.json`

```json
{
  "enemy_groups": {
    "goblin_camp_weak": {
      "enemies": [
        {
          "id": "goblin_warrior",
          "name": "Goblin Warrior",
          "portrait": "characters/goblin_warrior.png",
          "stats": {
            "strength": 2,
            "vitality": 2,
            "intelligence": 1,
            "faith": 1,
            "speed": 3,
            "dexterity": 2,
            "luck": 1
          },
          "attack_type": "melee",
          "combat_skills": [],
          "ai_type": "basic",
          "exp_reward": 10,
          "gold_reward": 5,
          "position": "front"
        },
        {
          "id": "goblin_scout",
          "name": "Goblin Scout",
          "portrait": "characters/goblin_scout.png",
          "stats": {
            "strength": 1,
            "vitality": 1,
            "intelligence": 1,
            "faith": 1,
            "speed": 4,
            "dexterity": 3,
            "luck": 2
          },
          "attack_type": "melee",
          "combat_skills": [],
          "ai_type": "basic",
          "exp_reward": 8,
          "gold_reward": 3,
          "position": "front"
        }
      ]
    }
  }
}
```

---

## 9. Companions

**File:** `data/characters/companions.json`

```json
{
  "companions": [
    {
      "id": "sir_aldric",
      "name": "Sir Aldric",
      "description": "A fallen knight seeking redemption. Quiet, loyal, deadly.",
      "class": "knight",
      "portrait": "characters/companion_aldric.png",
      "stats": {
        "strength": 3,
        "vitality": 3,
        "intelligence": 1,
        "faith": 2,
        "speed": 2,
        "dexterity": 1,
        "luck": 1
      },
      "combat_skills": [],
      "world_skills": ["climbing"],
      "ai_type": "basic",
      "join_story": "mvp_story_01",
      "join_node": "meet_aldric"
    }
  ]
}
```

---

## 10. Game Config

**File:** `data/config/game_config.json`

```json
{
  "game_title": "Vassnian",
  "hero_title": "Godsent",
  "atb_base_duration": 4.0,
  "atb_speed_modifier": 0.05,
  "potion_belt_max": 4,
  "stat_base_value": 1,
  "free_stat_points": 2,
  "world_skills_at_creation": 1,
  "max_injuries": 3,
  "starting_gold": 50,
  "max_party_size": 4,
  "skill_bar_slots": 4,
  "mvp_mode": true,
  "audio": {
    "music_volume": 0.7,
    "sfx_volume": 1.0,
    "music_tracks": {
      "main_menu": "audio/music/menu_theme.ogg",
      "world": "audio/music/world_exploration.ogg",
      "combat": "audio/music/combat_battle.ogg",
      "combat_boss": "audio/music/combat_boss.ogg",
      "shop": "audio/music/shop_theme.ogg",
      "game_over": "audio/music/game_over.ogg"
    }
  }
}
```

---

## 11. Save Data

**File:** Auto-generated at platform-specific save path

```json
{
  "version": 1,
  "player": {
    "name": "Player Name",
    "avatar": "avatars/avatar_01.png",
    "class": "knight",
    "level": 1,
    "exp": 0,
    "stats": { "strength": 2, "vitality": 2, "intelligence": 1, "faith": 1, "speed": 1, "dexterity": 1, "luck": 1 },
    "combat_skills": [],
    "world_skills": ["climbing"],
    "equipped": { "weapon": null, "armor": null, "accessory": null },
    "injuries": 0
  },
  "companions": ["sir_aldric"],
  "inventory": {
    "gold": 50,
    "potions": [],
    "equipment": [],
    "skill_books": [],
    "potion_belt": []
  },
  "world_state": {
    "tags": ["game_started", "chose_knight"],
    "completed_stories": [],
    "available_stories": ["mvp_story_01"],
    "world_level": 1
  },
  "current_story": null,
  "current_node": null
}
```
