# Vassnian — Architecture (Living Document)

## Crate Dependency Graph
```
vassnian_game (binary)
├── vassnian_engine (core systems)
└── vassnian_content (game data)
    └── vassnian_engine
```

## Module Tree

### vassnian_engine
```
src/
├── lib.rs
├── character/
│   ├── mod.rs
│   ├── stats.rs       — StatBlock, DerivedStats
│   ├── entity.rs      — Entity (Player/Companion/Enemy)
│   └── equipment.rs   — Equipment slots, stat modifiers
├── combat/
│   ├── mod.rs
│   ├── atb.rs         — ATB bar logic
│   ├── damage.rs      — Damage formulas
│   └── battle.rs      — Battle state management
├── story/
│   ├── mod.rs
│   ├── engine.rs      — Story node processing
│   ├── tags.rs        — Tag system
│   └── choice.rs      — Choice resolution
├── ai/
│   ├── mod.rs
│   └── basic.rs       — Basic AI (attack nearest)
├── inventory/
│   ├── mod.rs
│   ├── items.rs       — Potion, Equipment structs
│   └── belt.rs        — Potion belt (4 slots)
├── save/
│   └── mod.rs         — Save/load system
├── asset/
│   └── mod.rs         — AssetRef with placeholder fallback
└── audio/
    ├── mod.rs
    ├── manager.rs     — AudioManager trait
    └── refs.rs        — AudioRef (silent fallback)
```

### vassnian_content
```
src/
├── lib.rs
├── classes.rs         — Class definitions
├── skills.rs          — Combat skill definitions
├── world_skills.rs    — World skill definitions
└── loader.rs          — JSON data loader
```

### vassnian_game
```
src/
├── main.rs            — Entry point, Macroquad window config
├── app.rs             — Game state manager
├── screens/
│   └── mod.rs         — (stubs)
├── ui/
│   └── mod.rs         — (stubs)
├── rendering/
│   └── mod.rs         — (stubs)
└── platform/
    └── mod.rs         — (stubs)
```

## Current Status

### Phase 1 — COMPLETE
- Workspace with 3 crates compiles
- Macroquad window opens in portrait (390x844)
- Shows "VASSNIAN" title text
- All module stubs created

### Phase 2 — IN PROGRESS
- Core data structs
- JSON schemas
- Data loader

## Build Log

### Phase 1 — 2026-02-10
- Workspace created (vassnian_engine, vassnian_content, vassnian_game)
- Macroquad 0.4.14 window running in portrait mode
- All module stubs in place
- Rust 1.93.0 installed
