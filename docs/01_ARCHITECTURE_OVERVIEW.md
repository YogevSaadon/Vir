# Vassnian — Architecture Overview

## Hero Title
The player character is addressed as **"Godsent"** by all NPCs. This is their isekai identity — established in the god's prologue.

## 1. System Diagram

```
┌─────────────────────────────────────────────────────────┐
│                   vassnian_game                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ Screens  │  │    UI    │  │ Platform │              │
│  │          │  │Components│  │  Layer   │              │
│  │• Menu    │  │• Button  │  │• Ads stub│              │
│  │• Create  │  │• Tooltip │  │• Save IO │              │
│  │• Story   │  │• ATB bar │  │• WASM    │              │
│  │• Combat  │  │• Portrait│  │• Android │              │
│  │• Shop    │  │• Potion  │  │          │              │
│  │• Inventory│ │  Belt    │  │          │              │
│  └────┬─────┘  └────┬─────┘  └──────────┘              │
│       │              │                                   │
│       └──────┬───────┘                                   │
│              │ uses                                       │
├──────────────┼───────────────────────────────────────────┤
│              ▼                                           │
│  ┌───────────────────────┐  ┌────────────────────────┐  │
│  │   vassnian_engine      │  │  vassnian_content       │  │
│  │                       │  │                        │  │
│  │ • Combat System       │  │ • Class Definitions    │  │
│  │ • ATB Manager         │  │ • Skill Definitions    │  │
│  │ • Story Engine        │  │ • World Skill Defs     │  │
│  │ • Tag System          │  │ • JSON Data Loader     │  │
│  │ • AI System           │  │                        │  │
│  │ • Character System    │  │ Reads from:            │  │
│  │ • Inventory System    │  │ /data/*.json           │  │
│  │ • Save System         │  │                        │  │
│  │ • Asset System        │  │                        │  │
│  │ • Audio System        │  │                        │  │
│  └───────────────────────┘  └────────────────────────┘  │
│         ▲                            ▲                   │
│         │                            │                   │
│         │ engine defines traits      │ content provides  │
│         │ and generic systems        │ concrete data     │
│         │                            │                   │
└─────────┴────────────────────────────┴───────────────────┘

        ┌─────────────────────────┐
        │      /data/ (JSON)      │
        │                         │
        │  stories/  skills/      │
        │  characters/ items/     │
        │  missions/  config/     │
        └─────────────────────────┘

        ┌─────────────────────────┐
        │     /assets/            │
        │                         │
        │  characters/ skills/    │
        │  backgrounds/ ui/       │
        │  avatars/               │
        │  audio/music/           │
        │  audio/sfx/             │
        └─────────────────────────┘
```

---

## 2. Crate Responsibilities

### vassnian_engine (Core — No game-specific knowledge)
The engine crate is **reusable**. It defines systems and traits, not content.

| Module | Responsibility |
|--------|---------------|
| `combat/` | ATB timer, damage calculation, targeting rules, turn resolution, win/lose conditions |
| `combat/atb.rs` | ATB bar logic: fill rate, reset, pause/resume |
| `combat/damage.rs` | Damage formulas: attack vs defense, crits, equipment bonuses |
| `combat/battle.rs` | Battle state: units, positions, turn order, potion use |
| `combat/skills_runtime.rs` | Cooldown tracking, cast bar state, status effects, skill slots |
| `character/` | Entity, stats, equipment slots, skill slots, world skills, party |
| `character/stats.rs` | 7 primary stats, derived stat formulas, DerivedStatFormula system |
| `character/entity.rs` | Base Entity struct, Player/Companion/Enemy variants |
| `character/equipment.rs` | Equipment slots, stat modifiers from gear |
| `character/party.rs` | Party (Godsent group): members, gold, companion story tracking |
| `story/` | Story node graph, choice resolution, tag checks |
| `story/engine.rs` | Processes story JSON: current node, available choices, transitions |
| `story/tags.rs` | Tag system: set/get/check tags, tag-based filtering |
| `story/choice.rs` | Choice resolution: stat checks, world skill checks, outcomes |
| `ai/` | AI decision making for combat |
| `ai/basic.rs` | Basic AI: attack nearest, heal most wounded |
| `ai/class_ai.rs` | (Future) Class-specific AI behaviors |
| `inventory/` | Item storage, gold, potion belt |
| `inventory/items.rs` | Item trait, Potion, Equipment structs |
| `inventory/belt.rs` | Potion belt: max 4, use and destroy |
| `save/` | Serialize/deserialize game state |
| `asset/` | AssetRef with placeholder fallback (images), ImageManager stubs |
| `audio/` | AudioRef with silent fallback, AudioManager trait |
| `audio/manager.rs` | Play music (loop), play SFX (one-shot), stop, volume control |
| `audio/refs.rs` | AudioRef struct — points to audio file, silent if missing |
| `world/` | Terrain, game phases, missions, story pool, world state |
| `world/terrain.rs` | 8 terrain types (Forest, Mountain, Desert, etc.) |
| `world/phase.rs` | 8 game phases (Phase0 through Phase4.5), phase thresholds |
| `world/mission.rs` | MissionDef, MissionOption, MissionDifficulty, MissionContext |
| `world/story_pool.rs` | Story pool management: fixed, random, continuation tracking |
| `world/world_state.rs` | Runtime world state: time, phase progression, mission history |

### vassnian_content (Game Data — Defines what's in THIS game)
| Module | Responsibility |
|--------|---------------|
| `classes.rs` | Knight, Barbarian, etc. — starting stats, default skills |
| `skills.rs` | All combat skill definitions |
| `world_skills.rs` | All world skill definitions |
| `loader.rs` | Reads `/data/` JSON files into engine structs |

### vassnian_game (Binary — Wires everything together)
| Module | Responsibility |
|--------|---------------|
| `main.rs` | Entry point, Macroquad setup |
| `app.rs` | Game state manager, screen transitions, audio context |
| `screens/` | One screen per file, each implements `Screen` trait |
| `ui/` | Reusable UI widgets (buttons, tooltips, bars) |
| `rendering/` | Draw helpers, colors, layout constants |
| `platform/` | Ads stub, save paths, platform detection |

---

## 3. Key Data Flow

### Character Creation Flow
```
Main Menu
    → [New Game]
    → God Intro (isekai text, declares player "Godsent")
    → Character Creation Screen
        → Avatar Selection (pick image from list)
        → Class Selection (Knight only for MVP)
            → Click class → info panel shows details
        → Stat Point-Buy
            → 7 stats start at class base values
            → 2 free points to distribute
            → Stat descriptions shown inline
            → Derived stats update in real-time
        → World Skill Selection (pick 1 of 3)
            → Fixed description panel below skill list
        → Confirm
    → Phase 0 Arrival (NOT YET CODED — see doc 10):
        → Summoning circle, King speech, companion select, bar scene
    → Auto-save created
    → First mission / story encounter loads
```

### Story Flow
```
Story Screen
    → Load story JSON by ID
    → Display story text (speed based on settings)
    → Show available choices
        → Each choice may have:
            - No requirement (always available)
            - Stat requirement (show "[STR 5] Break the door" + best ally name/value)
            - World skill requirement (show "[Climbing] Scale the wall")
            - Companion world skill (show "[Companion: Climbing] Let ally climb")
        → If requirement met → instant pass
        → If not met → choice hidden or shown as unavailable
    → Player picks choice
    → Apply tags/implications
    → Branch to next node OR trigger combat OR transition to shop
    → Auto-save
```

### Combat Flow
```
Combat Screen
    → Load enemy data
    → Place units (2v2 MVP, all front row)
    → Start ATB timers (~4 sec base, modified by speed)
    → Loop:
        │ All ATB bars fill simultaneously
        │
        ├── Enemy ATB full → AI picks action → execute → reset ATB
        ├── Companion ATB full → AI picks action → execute → reset ATB
        ├── Player ATB full → highlight skill bar → wait for input
        │       → Player picks skill/target → execute → reset ATB
        │
        ├── Potion use: ANYTIME, no turn cost, from potion belt
        ├── Pause: freezes all ATB
        │
        ├── Unit reaches 0 HP → falls (removed from combat)
        │
        ├── All enemies dead → WIN → loot → return to story
        └── All allies dead → LOSE → +1 injury → return to story
            → 3 injuries → permadeath → game over
    → Auto-save after combat
```

### Shop Flow
```
Story triggers shop encounter
    → Shop Screen opens (overlay or transition)
    → Display shop inventory (loaded from JSON)
    → Player can:
        → Buy items (if enough gold)
        → View item info (tooltip)
        → Exit shop → return to story
    → Auto-save after purchase
```

---

## 4. State Management

Recommended: **State Stack** pattern

```rust
enum GameScreen {
    MainMenu,
    CharacterCreation(CreationStep),
    Story(StoryState),
    Combat(CombatState),
    Shop(ShopState),
    Inventory,
    Stats,
    SkillLoadout,
    Options,
    GameOver,
}

enum CreationStep {
    AvatarSelect,
    ClassSelect,
    StatPointBuy,
    WorldSkillSelect,
    Confirm,
}
```

The state stack allows overlays (e.g., opening inventory during story, or tooltip over anything).

---

## 5. Platform Abstraction

```rust
// Platform trait — implemented per target
trait PlatformService {
    fn save_path(&self) -> PathBuf;
    fn show_rewarded_ad(&self) -> bool;  // stub returns false
    fn get_screen_size(&self) -> (f32, f32);
    fn is_mobile(&self) -> bool;
}
```

For MVP, `DesktopPlatform` + WASM build are done. Mobile implementation comes later.
WASM uses `load_embedded_data()` with `include_str!` for JSON files.

---

## 6. Key Traits

```rust
/// Any screen in the game
trait Screen {
    fn update(&mut self, ctx: &mut GameContext) -> ScreenTransition;
    fn draw(&self, ctx: &GameContext);
}

/// Screen transition commands
enum ScreenTransition {
    None,
    Push(GameScreen),    // Open overlay
    Pop,                 // Close current
    Replace(GameScreen), // Swap current
    Quit,
}

/// AI decision maker
trait CombatAI {
    fn decide_action(&self, unit: &Entity, battle: &BattleState) -> CombatAction;
}

/// Ad service (stubbed for MVP)
trait AdService {
    fn show_rewarded_ad(&self, reward_type: RewardType) -> bool;
    fn is_ad_available(&self) -> bool;
}

/// Audio reference — like AssetRef but for sounds
struct AudioRef {
    path: String,  // "audio/music/combat_battle.ogg"
    // If file doesn't exist → silent (no crash)
}

/// Audio manager — handles music and SFX
trait AudioManager {
    fn play_music(&mut self, track: &AudioRef, looping: bool);
    fn stop_music(&mut self);
    fn play_sfx(&mut self, sfx: &AudioRef);
    fn set_music_volume(&mut self, volume: f32);  // 0.0 to 1.0
    fn set_sfx_volume(&mut self, volume: f32);
}

// MVP implementation: SilentAudioManager — all methods are no-ops
// When real audio files are dropped into assets/audio/, swap to MacroquadAudioManager
```

---

## 7. Audio — Screen-to-Music Mapping

Each screen/state has an associated music track. The audio manager crossfades or cuts between them.

| Screen/State | Music Track | Notes |
|-------------|-------------|-------|
| Main Menu | `audio/music/menu_theme.ogg` | Loops |
| Character Creation | `audio/music/menu_theme.ogg` | Same as menu |
| God Intro | `audio/music/menu_theme.ogg` | Same as menu (or silence) |
| Story / Exploration | `audio/music/world_exploration.ogg` | Loops |
| Combat (normal) | `audio/music/combat_battle.ogg` | Loops, starts on combat enter |
| Combat (boss) | `audio/music/combat_boss.ogg` | Future |
| Shop | `audio/music/shop_theme.ogg` | Loops |
| Game Over | `audio/music/game_over.ogg` | Plays once |

SFX triggers are hardcoded at the call site:
```rust
// Example: in combat when attack lands
audio.play_sfx(&AudioRef::new("audio/sfx/attack_hit.ogg"));

// Example: in UI when button clicked  
audio.play_sfx(&AudioRef::new("audio/sfx/ui_click.ogg"));
```
```

---

## 7. Configuration Constants

```rust
// combat
const ATB_BASE_DURATION: f32 = 4.0;     // seconds
const ATB_SPEED_MODIFIER: f32 = 0.05;   // per speed point
const POTION_BELT_MAX: usize = 4;

// stats
const STAT_COUNT: usize = 7;
const STAT_BASE_VALUE: i32 = 1;
const FREE_STAT_POINTS: i32 = 2;

// world skills
const WORLD_SKILLS_AT_CREATION: usize = 1;
const WORLD_SKILLS_PER_LEVEL: usize = 2;
const COMBAT_SKILLS_PER_LEVEL: usize = 1;

// injuries
const MAX_INJURIES: i32 = 3; // permadeath at 3

// shop MVP
const STARTING_GOLD: i32 = 50; // TBD exact value

// display
const TARGET_FPS: u32 = 60;
const DESIGN_WIDTH: f32 = 390.0;  // portrait mobile reference
const DESIGN_HEIGHT: f32 = 844.0; // iPhone 14 reference

// audio (MVP: silent — all AudioRef files missing = no sound)
const DEFAULT_MUSIC_VOLUME: f32 = 0.7;
const DEFAULT_SFX_VOLUME: f32 = 1.0;

// identity
const GAME_TITLE: &str = "Vassnian";
const HERO_TITLE: &str = "Godsent";  // NPCs address player as this
```
