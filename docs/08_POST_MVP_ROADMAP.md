# Vassnian — Post-MVP Roadmap

## Phases Overview

```
MVP (current)                    Demo                           Full Game
─────────────────────────────    ────────────────────────────   ──────────────
Knight only                      All 8 classes                  All areas
2v2 melee                        6v6 formation                  4 areas + bosses
Auto-attack only                 Full skill system              All content
1 companion                      Multiple companions            Romance, bestiary
2 stories + 1 combat             Full Act 1 + first boss        Full story
Placeholder art                  AI-generated art               Polished art
Silent (audio wired)             Music + SFX                    Full soundtrack
PC only                          PC + Web (Itch.io)             Mobile + all platforms
```

---

## Phase 1: MVP (Current — See 03_MVP_SPEC.md)

What we're building now. Goal: prove all systems work end-to-end.

---

## Phase 2: MVP+ (Bridge to Demo)

Small additions to MVP before writing demo content. These are **mechanical foundations** the demo needs.

### 2.1 Cooldown System (Turn-Based)

**Add to MVP combat engine.** Cooldowns count in **turns**, not seconds. A turn = one full ATB cycle for that character.

```
Skill: Stun
Cooldown: 3 turns

Player uses Stun
    → Stun goes on cooldown
    → Player's ATB fills and acts → CD: 2
    → Player's ATB fills and acts → CD: 1
    → Player's ATB fills and acts → CD: 0 (Stun available again)
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCooldown {
    pub max_turns: i32,        // e.g., 3
    pub remaining_turns: i32,  // counts down each time THIS unit acts
}
```

**Key rule:** Cooldowns tick per-unit, not globally. A fast character's cooldowns recover faster than a slow one's.

### 2.2 Three Skill Speed Types

Every active skill has a **speed type** that determines when it executes:

| Speed Type | When it Happens | Use Case |
|-----------|-----------------|----------|
| **Instant** | Executes immediately when selected, no ATB cost. Long cooldown. | Emergency moves: dodge, quick heal, counter |
| **Normal** | Executes on your ATB turn (current system). Normal cooldown. | Most skills: attack, stun, taunt |
| **Cast Time** | Starts casting on ATB turn, resolves after X seconds. Interruptible. | Powerful spells: fireball, group heal, AoE |

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillSpeed {
    Instant,                    // No ATB cost, long CD
    Normal,                     // Standard ATB turn
    CastTime { seconds: f32 },  // Starts on turn, resolves after delay
}
```

**Instant skill rules:**
- Can be used ANY time (even when ATB is not full)
- Does NOT reset ATB bar
- Has **long cooldown** (5+ turns) to balance
- Example: Knight's emergency Shield Bash, Rogue's Dodge

**Cast Time rules:**
- Character starts casting on their ATB turn
- A **cast bar** appears above the character (separate from ATB)
- During casting: character is vulnerable, can't do anything else
- If character is **stunned or knocked down** during cast → cast is **interrupted** (wasted turn)
- On completion: spell resolves (usually powerful)
- ATB resets after cast completes (not when cast starts)

```
ATB fills → Player selects Fireball (cast: 2 sec)
    → ATB bar changes to CAST BAR [======>    ] 2 sec
    → Character glows/pulses (visual feedback)
    → If hit with Stun during cast → INTERRUPTED! (skill on half cooldown?)
    → Cast completes → Fireball resolves → ATB resets
```

**Cast Time visual:**
```
┌──────┐
│Knight│
│HP████│
│ATB═══│  ← normal ATB
│      │
└──────┘

While casting:
┌──────┐
│Knight│
│HP████│
│CAST▓▓│  ← cast bar (different color from ATB)
│🔥 2s │  ← spell icon + time remaining
└──────┘
```

### 2.3 Updated Skill Schema

```json
{
  "id": "fireball",
  "name": "Fireball",
  "description": "Hurls a ball of fire at an enemy.",
  "class_origin": "mage",
  "type": "active",
  "speed": { "type": "cast_time", "seconds": 2.0 },
  "cooldown_turns": 2,
  "resource_cost": 15,
  "target": "single_enemy",
  "effects": [
    { "type": "damage", "base": 40, "scaling_stat": "intelligence", "multiplier": 1.5 }
  ],
  "stat_requirements": { "intelligence": 4 },
  "icon": "skills/fireball.png",
  "can_be_interrupted": true
}
```

```json
{
  "id": "emergency_block",
  "name": "Emergency Block",
  "description": "Instantly raise your shield. Long recovery.",
  "class_origin": "knight",
  "type": "active",
  "speed": { "type": "instant" },
  "cooldown_turns": 6,
  "resource_cost": null,
  "target": "self",
  "effects": [
    { "type": "modify_stat", "stat": "defense", "amount": 10, "duration": 1 }
  ],
  "stat_requirements": { "vitality": 3 },
  "icon": "skills/emergency_block.png",
  "can_be_interrupted": false
}
```

```json
{
  "id": "stun",
  "name": "Stun",
  "description": "Freezes an enemy's ATB bar.",
  "class_origin": "knight",
  "type": "active",
  "speed": { "type": "normal" },
  "cooldown_turns": 3,
  "resource_cost": null,
  "target": "single_enemy",
  "effects": [
    { "type": "freeze_atb", "duration": 2.0 }
  ],
  "stat_requirements": { "strength": 2 },
  "icon": "skills/stun.png",
  "can_be_interrupted": false
}
```

### 2.4 Three-Bar Combat UI Update

Each unit now potentially shows **3 bars**:

```
┌──────────┐
│  Knight  │
│ HP ████░░│  ← Health (always visible)
│ MP ███░░░│  ← Mana (only for mana-using classes; hidden for cooldown-only classes)
│ATB ═════ │  ← ATB timer (or CAST bar when casting)
└──────────┘
```

| Bar | When Visible | Color |
|-----|-------------|-------|
| **HP** | Always | Green → Yellow → Red |
| **MP (Mana)** | Only for Mana/ManaCooldown classes (Mage, Cleric, Paladin, Necromancer) | Blue |
| **ATB** | Always (switches to Cast bar during casting) | White (ATB) / Orange (Cast) |

**For MVP Knight (cooldown only):** HP + ATB only (no mana bar). The mana bar system must exist in engine code but Knight won't use it.

### 2.5 What to Add to MVP Engine Now

These should be **built into the engine** during MVP even if the demo content uses them:

| System | MVP Implementation | Demo Expands To |
|--------|-------------------|-----------------|
| Cooldown tracking | Track per-skill, per-unit CD. Tick on unit's turn. | All skills have CDs |
| SkillSpeed enum | Define Instant/Normal/CastTime. MVP only uses Normal. | Demo uses all 3 |
| Cast bar | Engine supports cast state. MVP never triggers it. | Mage/Cleric use it |
| Mana system | Engine tracks mana. MVP Knight has no mana. | Mage/Cleric use it |
| Mana bar UI | Render if unit has mana. MVP Knight: hidden. | Shows for casters |
| Interrupt system | If stunned during cast → cancel. MVP: no casts to interrupt. | Tactical depth |
| Skill slot system | 4 slots, swappable outside combat. MVP: empty. | Demo: full loadout |

---

## Phase 3: Demo (Act 1 — Up to First Boss)

### 3.1 Content Priority Order

Build content in this order — story and mechanics first, polish last:

```
Step 1: STORY & DATA
    → Write all Act 1 stories (JSON)
    → Define all story branches and tag logic
    → Write companion recruitment stories
    → Write shopkeeper encounters
    → Define the first boss encounter

Step 2: SKILLS & STATS
    → Define all 8 classes with full starting skills
    → Balance stat formulas
    → Create skill books for cross-class learning
    → Define world skills (full list)
    → Define secret skills (dual-stat unlocks)

Step 3: COMBAT CONTENT
    → Design enemy types for Act 1 (varied AI)
    → Create enemy groups with scaling
    → Design first boss (unique skills, phases?)
    → Balance 6v6 encounters
    → Implement class-specific AI

Step 4: COMPANIONS
    → Write companion characters (personality, dialogue)
    → Define companion skill sets
    → Define companion world skills
    → Implement companion quest chains

Step 5: ITEMS & ECONOMY
    → Design equipment tiers for Act 1
    → Balance shop prices
    → Create skill book drop tables
    → Design potion variety

Step 6: VISUALS
    → Generate character portraits (AI)
    → Generate backgrounds (AI)
    → Generate skill icons (AI)
    → Create UI theme (colors, fonts, frames)
    → Add battle visual effects (shake, flash, etc.)

Step 7: SOUND & MUSIC
    → Source/create menu theme
    → Source/create exploration music
    → Source/create battle music
    → Add UI sound effects
    → Add combat sound effects
```

### 3.2 Demo Mechanical Additions

Everything from MVP+ (section 2) plus:

| System | What's New |
|--------|-----------|
| **6v6 Formation** | Front/back rows, 3 units per row |
| **Ranged vs Melee targeting** | Melee = front only, Ranged = all (with exposure penalty) |
| **Position swapping** | Swap 2 allies, both ATB reset |
| **Spear weapons** | Hit front + back in same column |
| **Barbarian Push** | Swap enemy front/back positions |
| **Toggle skills** | Cost max mana to maintain (Stealth, Blessing, Skeleton upkeep) |
| **Summoning** | Necromancer skeletons, Ranger pet — own ATB bars |
| **All 8 classes** | Full skill sets, class-specific AI |
| **Companion system** | Multiple companions, AI personalities, recruitment quests |
| **Skill books** | Find/buy books to learn other class skills |
| **World level** | Difficulty scales as clues collected |
| **Tag system (full)** | Stories branch based on past decisions |
| **Injury + ad removal** | Watch ad to remove 1 injury (AdMob integration) |
| **Equipment system** | Weapon types, armor, accessories with stat bonuses |
| **Status effects** | Poison, stun, slow, burn, etc. with visual indicators |
| **Resistance board** | Show enemy weaknesses in combat UI |
| **Bestiary** | Track defeated enemy types |

### 3.3 Demo Story Structure

```
God Intro (Isekai)
    → Character Creation (all 8 classes)
    → Act 1 begins
    │
    ├── Opening stories (linear, introduce world)
    │   ├── First companion joins
    │   └── First combat (tutorial-ish)
    │
    ├── Exploration phase (non-linear, tag-driven)
    │   ├── ~4 quests available at a time
    │   ├── Varying difficulty
    │   ├── Side stories as puzzles
    │   ├── More companions discoverable
    │   ├── Shops accessible
    │   └── Clues collected → world level rises
    │
    ├── Story events (triggered by tags/progress)
    │   ├── Mini-bosses
    │   └── Companion quests
    │
    └── FIRST BOSS
        ├── Weakened by investigation quests? (optional)
        ├── Boss fight (unique mechanics)
        ├── Win → Act 1 complete → "Buy full game" / ad gate
        └── Lose → injury → retry
```

### 3.4 Demo Scale Targets

| Element | Target |
|---------|--------|
| Classes playable | 8 |
| Companions recruitable | 3-5 |
| Story encounters | 20-30 |
| Combat encounters | 10-15 |
| Unique enemy types | 8-12 |
| Boss fights | 1 main + 1-2 mini |
| Shops | 2-3 |
| Equipment items | 15-20 |
| Skill books findable | 5-10 |
| World skills | 6-10 |
| Combat skills total | 40-60 (across all classes) |
| Play time | 30-60 minutes |

### 3.5 Demo Platform Targets

| Platform | Status |
|----------|--------|
| PC (Windows/Mac/Linux) | ✅ Primary |
| Web (WASM on Itch.io) | ✅ Demo launch platform |
| Android | 🔄 If time allows |
| iOS | ❌ After demo feedback |

### 3.6 Demo Definition of Done

The demo is complete when:
1. Player can play through all of Act 1 and reach the first boss
2. All 8 classes are playable with distinct skill sets
3. At least 3 companions are recruitable with unique AI
4. 6v6 formation combat works with all targeting rules
5. Cast time, instant, and normal skills all work
6. Tag system drives story branching correctly
7. Shop, inventory, equipment, and skill books all work
8. AI-generated art for all characters, skills, and backgrounds
9. Music and SFX for all screens
10. Playable on Itch.io (WASM build)
11. Ad integration works (at least stub for rewarded ads)
12. Paywall gate at boss completion ("buy to continue")

---

## Phase 4: Full Game (Post-Demo — Future)

Not detailed yet. High level:

- Areas 2, 3, 4 with unique themes
- All companion quest chains
- Romance system
- Roguelite meta-upgrades (between runs)
- Secret skills (dual-stat unlocks)
- Advanced boss mechanics (phases, unique skills)
- Damage types and resistance system
- Full bestiary
- Advanced animations (injury-state portraits)
- Full mobile release (Android + iOS)
- Steam release
- Monetization: premium unlock or free + ads

---

## Architecture Notes for Agent

### What to build into the engine NOW (MVP) even though demo uses it:

The following systems should have **traits/structs defined** in `vassnian_engine` during MVP, even if the MVP content doesn't use them. This prevents refactoring later:

```rust
// Cooldown — build now, Knight uses in demo
pub struct SkillCooldown {
    pub max_turns: i32,
    pub remaining_turns: i32,
}

// Skill speed — build enum now, MVP only uses Normal
pub enum SkillSpeed {
    Instant,
    Normal,
    CastTime { seconds: f32 },
}

// Cast state — build now, no one casts in MVP
pub struct CastState {
    pub skill_id: String,
    pub total_time: f32,
    pub elapsed: f32,
    pub caster_id: EntityId,
}

// Mana — build now, Knight has no mana
pub struct ManaPool {
    pub current: i32,
    pub max: i32,
    pub base_max: i32,        // before toggle reductions
    pub toggle_reserved: i32,  // sum of active toggles
}

// Status effects — build now, MVP has none
pub struct StatusEffect {
    pub id: String,
    pub duration_turns: i32,
    pub effect: StatusEffectType,
}

pub enum StatusEffectType {
    Stun,
    Poison { damage_per_turn: i32 },
    Slow { atb_modifier: f32 },
    Burn { damage_per_turn: i32 },
    Buff { stat: String, amount: i32 },
    Debuff { stat: String, amount: i32 },
}

// Formation position — build now, MVP uses front only
pub enum FormationRow { Front, Back }
pub struct FormationPosition {
    pub row: FormationRow,
    pub column: usize,  // 0, 1, 2 (for 3 per row)
}
```

### JSON schema compatibility

All demo skill JSONs must be **backwards compatible** with MVP. The MVP loader should:
- Accept `speed` field (default to `Normal` if missing)
- Accept `cooldown_turns` field (default to `0` if missing)
- Accept `can_be_interrupted` field (default to `false` if missing)
- Accept `resource_cost` as `null` for cooldown-only classes

This way MVP data files are valid demo data files — no migration needed.
