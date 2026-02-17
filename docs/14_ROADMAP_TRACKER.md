# Vassnian — 14 Roadmap & Progress Tracker

## How to Use This File
Agent: update the status column after completing each task. Use these symbols:
- ⬜ Not started
- 🔨 In progress
- ✅ Complete
- ⚠️ Blocked (add note)
- 🔄 Needs revision

---

## MVP Build Phases (from 00_AGENT_RULES.md)

### Phase 1: Skeleton
| # | Task | Status | Notes |
|---|------|--------|-------|
| 1 | Set up workspace with 3 crates | ✅ | |
| 2 | vassnian_engine: empty lib with module stubs | ✅ | |
| 3 | vassnian_content: empty lib | ✅ | |
| 4 | vassnian_game: Macroquad window showing "Vassnian" | ✅ | |
| 5 | Verify: `cargo run` shows a window | ✅ | |

### Phase 2: Core Data
| # | Task | Status | Notes |
|---|------|--------|-------|
| 6 | Define stat system structs (7 primary + derived) | ✅ | DerivedStatFormula system added (08.5 §1) |
| 7 | Define character struct | ✅ | Entity with stats, skills, equipment |
| 8 | Define skill structs (with speed, cooldown_turns, exclusivity) | ✅ | SkillExclusivity enum added (08.5 §2) |
| 9 | Define item structs (with level, rarity, category) | ✅ | ItemRarity + ItemCategory + level fields (08.5 §4) |
| 10 | Write JSON schemas and sample data files | ✅ | All data in /data/ |
| 11 | Write loader that reads JSON into structs | ✅ | load_all_data + stat_formulas.json |
| 12 | Define terrain enum, game phase enum, time tracking | ✅ | world module: Terrain (8), GamePhase (8), PhaseThresholds |
| 13 | Define mission struct and story script enum | ✅ | MissionDef, MissionOption, StoryScript (4 variants) |
| 14 | Define enemy level/type fields | ✅ | level, enemy_types, phase_range on EnemyDef |
| 15 | Verify: unit tests pass for all data loading | ✅ | 46 tests pass |

### Phase 3: Game State & Screens
| # | Task | Status | Notes |
|---|------|--------|-------|
| 16 | Implement game state manager (screen stack) | ✅ | |
| 17 | Main menu screen (New Game, Continue, Options) | ✅ | |
| 18 | Character creation: avatar pick | ✅ | |
| 19 | Character creation: class pick (Knight only) | ✅ | Show locked icons for other classes |
| 20 | Character creation: stat point-buy | ✅ | |
| 21 | Character creation: world skill pick | ✅ | |
| 22 | Character creation: passive sub-choice (scaffolded, skip for Knight) | ✅ | ClassPassive + PassiveSubChoice structs in classes.rs |
| 23 | Tooltip/info panel system | ✅ | draw_tooltip in ui, info panel in inventory |
| 24 | Verify: menu → creation → see stats | ✅ | |

### Phase 4: Story Engine
| # | Task | Status | Notes |
|---|------|--------|-------|
| 25 | Define story JSON format (nodes, choices, tags, stat checks) | ✅ | |
| 26 | Implement tag system (HashMap) | ✅ | TagSystem + companion tags |
| 27 | Implement story renderer (text display, choices, branching) | ✅ | Paginated with arrow navigation |
| 28 | Implement world skill / stat check on choices | ✅ | |
| 29 | Add allows_shop flag to story nodes | ✅ | allows_shop + allows_city on StoryNode |
| 30 | Add optional story scripts field (scaffolded, empty) | ✅ | StoryScript enum (4 variants) |
| 31 | Wire MVP story 1 (uses world skills + companion) | ✅ | |
| 32 | Verify: can play through story with choices | ✅ | |

### Phase 5: Combat
| # | Task | Status | Notes |
|---|------|--------|-------|
| 33 | Implement ATB system (~4 sec base, speed-modified) | ✅ | |
| 34 | Implement auto-attack (melee, strength-based) | ✅ | |
| 35 | Implement 2v2 combat layout (front row only) | ✅ | |
| 36 | Implement basic AI (attack nearest enemy) | ✅ | |
| 37 | Implement pause button | ✅ | |
| 38 | Implement potion belt (4 slots, instant use) | ✅ | |
| 39 | Implement 3-bar display (HP always, MP if caster, ATB) | ✅ | Knight: HP + ATB only |
| 40 | Scaffold cooldown system (turn-based, per-unit) | ✅ | cooldown_turns field on skills |
| 41 | Scaffold cast bar system (CastTime speed type) | ✅ | SkillSpeed::CastTime{seconds} |
| 42 | Scaffold instant skill support | ✅ | SkillSpeed::Instant |
| 43 | Implement win/lose conditions | ✅ | |
| 44 | Implement injury system (3 = permadeath) | ✅ | |
| 45 | Wire combat encounter from story | ✅ | |
| 46 | Verify: 2v2 fight, win/lose, injuries work | ✅ | |

### Phase 6: Shop & Items
| # | Task | Status | Notes |
|---|------|--------|-------|
| 47 | Implement inventory screen (GRID/SQUARES) | ✅ | Grid UI with character silhouette |
| 48 | Implement item info panel (click any square) | ✅ | Detail popup with equip/unequip |
| 49 | Implement equipment system (equip/unequip) | ✅ | |
| 50 | Implement gold system | ✅ | |
| 51 | Implement party bar (switch characters) | ✅ | |
| 52 | Implement shop screen (AFTER inventory) | ✅ | |
| 53 | Implement potion mode in shop (belt view) | ✅ | |
| 54 | Wire MVP story 2 (shopkeeper → shop) | ✅ | |
| 55 | Verify: buy sword, equip, damage increase | ✅ | |

### Phase 7: Save & Polish
| # | Task | Status | Notes |
|---|------|--------|-------|
| 56 | Implement auto-save (after story/combat/shop) | ✅ | |
| 57 | Implement save/load (single slot) | ✅ | |
| 58 | AssetRef placeholder system (colored rect + name) | ✅ | |
| 59 | AudioRef silent fallback system | ✅ | SilentAudioManager |
| 60 | Image manager (preload/release stubs) | ✅ | ImageManager in asset module |
| 61 | Options menu (text speed + volume sliders) | ✅ | In-game + main menu variants |
| 62 | Ad integration stub (trait + mock) | ✅ | show_rewarded_ad() in platform |
| 63 | Mission selection scaffolding (struct, not wired) | ✅ | MissionDef, MissionOption, MissionDifficulty |
| 64 | Story pool / setup scaffolding (struct, not wired) | ✅ | StoryPool with fixed/random/continuation pools |
| 65 | Verify: full MVP loop end to end | ✅ | menu → god intro → char creation → story → combat → shop → inventory → stats → game over |

### Phase 8: Platform
| # | Task | Status | Notes |
|---|------|--------|-------|
| 66 | Test WASM build | ✅ | Builds clean, ~1MB, embedded data loader for WASM |
| 67 | Test Android build (if possible) | ⬜ | Not attempted yet |
| 68 | Package for Itch.io | ✅ | web_build/ dir ready, build_wasm.ps1 script |

---

## Post-MVP Milestones

### Engine Scaffolds (Done — structs exist, not wired to gameplay)
| Task | Status | Notes |
|------|--------|-------|
| SkillCooldown / CastState / StatusEffect runtime | ✅ | skills_runtime.rs, wired into CombatUnit |
| UseSkill combat action | ✅ | CombatAction::UseSkill variant + stub handler |
| SkillSlots (4-slot bar on Entity + CombatUnit) | ✅ | Persistent on Entity, per-combat on CombatUnit |
| WorldState (time tracking + phase progression) | ✅ | world_state.rs with advance_time, force_phase |
| Party system (Godsent group management) | ✅ | party.rs: members, gold, companion story tracking |
| Mission catalog JSON | ✅ | data/missions/mission_catalog.json (4 placeholders) |
| DerivedStatFormula (data-driven stat scaling) | ✅ | stat_formulas.json + evaluate() |
| ClassPassive + PassiveSubChoice | ✅ | classes.rs: terrain/deity/weapon sub-choices |
| SkillExclusivity (ClassOnly / LearnableByAll) | ✅ | skills.rs |
| ItemRarity + ItemCategory | ✅ | items.rs: Common/Uncommon/Rare/Unique |
| StoryScript (4 variants) | ✅ | engine.rs: RandomSelect, ForcedBattle, PartySplit, TimelineJump |
| ImageManager stubs | ✅ | asset/mod.rs: preload/release |

### Demo Prep — Gameplay Implementation (see 08_POST_MVP_ROADMAP.md)
| Task | Status | Notes |
|------|--------|-------|
| Wire cooldown system into combat loop | ⬜ | Structs ready, needs turn-by-turn integration |
| Wire 3 skill speed types (instant/normal/cast) | ⬜ | Enums ready, needs combat execution |
| Wire cast bar + interruption | ⬜ | CastState ready, needs UI + cancel logic |
| Wire mana system into combat | ⬜ | current_mana on Entity, needs cost/regen |
| Implement 6v6 formation | ⬜ | FormationRow::Front/Back ready |
| Implement all 8+ classes | ⬜ | ClassDef supports it, needs JSON + balancing |
| Wire class passives with sub-choices | ⬜ | Structs ready, needs creation screen + effects |
| Implement mission selection screen (3 choices) | ⬜ | MissionDef + MissionOption ready |
| Wire story pool / randomization | ⬜ | StoryPool ready, needs game loop integration |
| Wire time / phase progression | ⬜ | WorldState ready, needs mission cost integration |
| Implement Phase 0 intro (King, companion select, bar scene) | ⬜ | Story engine can handle it, needs JSON stories |
| Build bestiary | ⬜ | |
| Build world lore | ⬜ | |
| Write story summaries | ⬜ | |
| Build AI story writer | ⬜ | |
| Write Act 1 stories | ⬜ | |

---

## Open Design Questions

| Question | Source | Status |
|----------|--------|--------|
| Is Charisma a new (8th) primary stat, a derived stat, or class-specific? | 11 §2 | ❓ Needs decision |
| Is Bard class #9, or does it replace one of the original 8? | 11 §3 | ❓ Needs decision |
| Does "Warrior" = Knight, or is it a separate class? | 11 §1 note | ❓ Needs decision |
| What is the final boss? | 10 §Phase 4 | ❓ Not decided |
| Exact time thresholds per phase? | 13 §7 | ❓ Balancing later |
| Starting gold amount? | Various | ✅ 50 (set in game_config.json) |
