# Agent Rules — Vassnian Project

## Identity
You are building **Vassnian**, a dark fantasy isekai text-based party RPG in Rust using Macroquad.
The player character is known as **"Godsent"** — NPCs address the player by this title throughout the story.

---

## 1. Architecture File (MANDATORY)

Maintain a file at the project root: `ARCHITECTURE.md`

This file must always reflect the **current state** of the codebase. Update it every time you:
- Add a new module, struct, enum, or trait
- Change dependencies between crates
- Add a new screen or system
- Modify data schemas

The file should contain:
- Crate dependency graph
- Module tree per crate
- Key structs/enums/traits with one-line descriptions
- Current status (what's built, what's stubbed, what's TODO)

---

## 2. File Organization Rules

### Rust Workspace Structure
```
vassnian/
├── Cargo.toml              # Workspace root
├── ARCHITECTURE.md          # Always up-to-date
├── README.md
│
├── vassnian_engine/          # Core game systems (reusable)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── combat/          # ATB, damage, targeting
│       ├── character/       # Stats, skills, entity hierarchy
│       ├── story/           # Story engine, tag system, branching
│       ├── ai/              # AI decision trees
│       ├── inventory/       # Items, equipment, potions
│       ├── save/            # Auto-save system
│       ├── asset/           # AssetRef, placeholder system (images + audio)
│       └── audio/           # Audio manager, music/sfx stubs
│
├── vassnian_content/         # Game-specific data & definitions
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── classes.rs       # Class definitions
│       ├── skills.rs        # Skill definitions
│       ├── world_skills.rs  # World skill definitions
│       └── loader.rs        # JSON data loader
│
├── vassnian_game/            # The actual game binary
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── app.rs           # Game state manager
│       ├── screens/         # One file per screen
│       │   ├── main_menu.rs
│       │   ├── character_creation.rs
│       │   ├── story.rs
│       │   ├── combat.rs
│       │   ├── shop.rs
│       │   ├── inventory.rs
│       │   ├── stats.rs
│       │   └── skill_loadout.rs
│       ├── ui/              # Reusable UI components
│       │   ├── mod.rs
│       │   ├── button.rs
│       │   ├── text_box.rs
│       │   ├── tooltip.rs
│       │   ├── atb_bar.rs
│       │   ├── portrait.rs
│       │   └── potion_belt.rs
│       ├── rendering/       # Draw logic
│       └── platform/        # Platform-specific code
│           ├── mod.rs
│           ├── ads.rs       # Ad integration stub
│           └── save_path.rs # Platform-specific save location
│
├── data/                    # All JSON game data
│   ├── stories/
│   │   ├── mvp_story_01.json
│   │   └── mvp_story_02_shop.json
│   ├── skills/
│   │   ├── combat_skills.json
│   │   └── world_skills.json
│   ├── characters/
│   │   ├── classes.json
│   │   └── companions.json
│   ├── items/
│   │   ├── potions.json
│   │   ├── equipment.json
│   │   └── shop_inventories.json
│   └── config/
│       ├── game_config.json
│       └── stat_formulas.json
│
└── assets/                  # All game assets
    ├── characters/          # Character portraits
    ├── skills/              # Skill icons
    ├── backgrounds/         # Story/battle backgrounds
    ├── ui/                  # UI elements (buttons, frames)
    ├── avatars/             # Player avatar choices
    └── audio/               # Sound & music files
        ├── music/
        │   ├── menu_theme.ogg
        │   ├── world_exploration.ogg
        │   ├── combat_battle.ogg
        │   ├── combat_boss.ogg
        │   ├── shop_theme.ogg
        │   └── game_over.ogg
        └── sfx/
            ├── ui_click.ogg
            ├── ui_confirm.ogg
            ├── ui_back.ogg
            ├── attack_melee.ogg
            ├── attack_hit.ogg
            ├── attack_miss.ogg
            ├── potion_use.ogg
            ├── skill_activate.ogg
            ├── heal.ogg
            ├── level_up.ogg
            ├── gold_pickup.ogg
            ├── death.ogg
            └── atb_ready.ogg
```

### Data Files
- ALL game content lives in `/data/` as JSON
- Code NEVER hardcodes game content (story text, skill names, stat values)
- Use `serde` for all serialization/deserialization
- Every data struct must derive `Serialize, Deserialize`

### Assets (Images)
```
assets/
├── characters/          # Portraits
├── skills/              # Skill icons
├── backgrounds/         # Story/battle backgrounds
├── ui/                  # UI elements
└── avatars/             # Player avatar choices
```
- Use `AssetRef` system — code references images by path
- Missing image = auto-placeholder with filename text (colored rectangle + name label)
- NEVER crash on missing asset

### Assets (Audio)
```
assets/audio/
├── music/               # Background music loops
│   ├── menu_theme.ogg
│   ├── world_exploration.ogg
│   ├── combat_battle.ogg
│   ├── combat_boss.ogg
│   ├── shop_theme.ogg
│   └── game_over.ogg
└── sfx/                 # Sound effects
    ├── ui_click.ogg
    ├── ui_confirm.ogg
    ├── ui_back.ogg
    ├── attack_melee.ogg
    ├── attack_hit.ogg
    ├── attack_miss.ogg
    ├── potion_use.ogg
    ├── skill_activate.ogg
    ├── heal.ogg
    ├── level_up.ogg
    ├── gold_pickup.ogg
    ├── death.ogg
    └── atb_ready.ogg
```
- Use `AudioRef` system — same pattern as `AssetRef`
- Missing audio file = **silent** (no crash, no error, just no sound)
- Audio manager must support: play music (looping), play SFX (one-shot), stop music, set volume
- MVP: all audio files are missing (silent game). The system is wired so dropping in .ogg files "just works"
- Use `kira` or `macroquad::audio` for playback (when audio files exist)

---

## 3. Coding Rules

### General
- Write idiomatic Rust — use `Result<T, E>` for errors, not panics
- Every public function must have a doc comment
- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` on all data structs
- Keep functions under 50 lines — split into helpers
- No `unwrap()` in game logic — only in tests or infallible cases with comment

### Architecture Principles
- **Engine knows nothing about content** — `vassnian_engine` never imports `vassnian_content`
- **Content defines data, engine processes it** — engine takes trait objects or generic data
- **Game wires them together** — `vassnian_game` imports both and connects them
- **Data-driven everything** — if it can be JSON, it should be JSON

### Naming Conventions
- Modules: `snake_case`
- Structs/Enums: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- JSON fields: `snake_case`

### Testing
- Write unit tests for engine logic (combat math, ATB, AI decisions)
- Write integration tests for data loading (can all JSON files parse?)
- Test command: `cargo test --workspace`

---

## 4. Build Order (MVP)

Build in this exact order. Each step must compile and run before moving to next:

### Phase 1: Skeleton
1. Set up workspace with 3 crates
2. `vassnian_engine`: empty lib with module stubs
3. `vassnian_content`: empty lib
4. `vassnian_game`: Macroquad window that shows "Vassnian" text
5. Verify: `cargo run` shows a window

### Phase 2: Core Data
6. Define stat system structs (7 primary stats, derived stats)
7. Define character struct (name, stats, world_skills, combat_skills, equipment)
8. Define skill structs (CombatSkill, WorldSkill)
9. Define item structs (Potion, Equipment)
10. Write JSON schemas and sample data files
11. Write loader that reads JSON into structs
12. Verify: unit tests pass for all data loading

### Phase 3: Game State & Screens
13. Implement game state manager (screen stack or state machine)
14. Main menu screen (New Game button)
15. Character creation screen (avatar pick → class pick → stat point-buy → world skill pick)
16. Tooltip/info panel system (click anything for info)
17. Verify: can go from menu → creation → see stats

### Phase 4: Story Engine
18. Define story JSON format (nodes, choices, tags, stat checks)
19. Implement tag system (HashMap<String, TagValue>)
20. Implement story renderer (text display, choices, branching)
21. Implement world skill / stat check on choices
22. Wire MVP story 1 (uses world skills + companion skill)
23. Verify: can play through story with choices

### Phase 5: Combat
24. Implement ATB system (timer per unit, ~4 sec base)
25. Implement basic auto-attack (melee, damage = strength-based)
26. Implement 2v2 combat layout (front row only for MVP)
27. Implement basic AI (attack nearest enemy)
28. Implement pause button
29. Implement potion belt (4 slots, instant use)
30. Implement win/lose conditions
31. Implement injury system (lose = +1 injury, 3 = permadeath)
32. Wire combat encounter from story
33. Verify: can fight 2v2, win/lose, injuries work

### Phase 6: Shop & Items
34. Implement shop screen (buy potion, buy sword)
35. Implement gold system
36. Implement equipment system (sword = more damage)
37. Implement inventory screen
38. Wire MVP story 2 (shopkeeper encounter → shop)
39. Verify: can buy sword, equip it, see damage increase

### Phase 7: Save & Polish
40. Implement auto-save (save after every story/combat)
41. Implement save/load (single slot)
42. AssetRef placeholder system
43. Options menu (text speed toggle)
44. Ad integration stub (trait with mock implementation)
45. Verify: full MVP loop works end to end

### Phase 8: Platform
46. Test WASM build
47. Test Android build (if possible)
48. Package for Itch.io

---

## 5. When Stuck

- If a design decision isn't in the docs, **make the simplest choice that doesn't close doors**
- If something needs clarification, add a `// TODO: DESIGN_QUESTION: <question>` comment
- If a system is too complex for MVP, **stub it** with a trait and simple implementation
- Keep a `TODO.md` file at project root with all open items

---

## 6. Communication Protocol

After each phase, write a brief status update in `ARCHITECTURE.md` under a "## Build Log" section:
```markdown
## Build Log
### Phase 1 — [date]
- ✅ Workspace created
- ✅ Macroquad window running
- ⚠️ Note: macroquad 0.4 has breaking changes, using 0.3
```

---

## 7. Dependencies (Cargo.toml)

### vassnian_engine
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rand = "0.8"
```

### vassnian_content
```toml
[dependencies]
vassnian_engine = { path = "../vassnian_engine" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### vassnian_game
```toml
[dependencies]
vassnian_engine = { path = "../vassnian_engine" }
vassnian_content = { path = "../vassnian_content" }
macroquad = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Pin versions. Do not use `*` or unpinned ranges.
