# Vassnian — Agent Instructions

## Identity
You are working on **Vassnian**, a dark fantasy isekai text-based party RPG in Rust + Macroquad.
Hero title: **"Godsent"**. 3-crate workspace: `vassnian_engine`, `vassnian_content`, `vassnian_game`.

## Critical Rules — READ FIRST

### 1. NEVER implement without approval
- When the user gives a task, **plan first, explain what you'll do, wait for "go"**
- Do NOT write code, create files, or modify anything until the user confirms
- Exception: bug fixes where the fix is obvious and small (< 10 lines)

### 2. NEVER create content
- **ZERO tolerance**: Do not write story text, lore, dialogue, character names, item descriptions, party banter, or any narrative content
- The user generates all content with a separate storyteller AI and provides it to you
- Your job: organize the content, check engine compatibility, wire it into the game
- If content is needed, ask the user to provide it — never generate it yourself

### 3. NEVER decide balance numbers
- Damage, gold, XP, stat values, HP, scaling curves — **always ask the user**
- The user has a separate AI for stats and balancing
- Your job: make the systems **scaleable** so numbers can be tuned from JSON config
- If a number must be chosen, present options and let the user pick

### 4. NEVER add features the user didn't ask for
- No bonus refactors, no "while I'm here" improvements
- No docstrings/comments on code you didn't change
- No error handling for impossible scenarios
- If you see something wrong unrelated to the task, mention it — don't fix it

### 5. Keep changes minimal and focused
- Touch only the files needed for the current task
- Prefer small edits over rewrites
- If a task requires 5+ files, break it down and confirm scope first

### 6. ALWAYS build and test after changes
- `cargo build` after every code change
- `cargo test` after every code change
- Report results. Don't hide warnings or errors.

### 7. Design for scalability
- Every number that could be tuned → JSON config, not hardcoded
- Every list that could grow → Vec/HashMap loaded from data files
- Every system → works with 1 item or 100 items without code changes

## Architecture Reference

### Crate rules
- `vassnian_engine` — core systems, knows NOTHING about content
- `vassnian_content` — data definitions, JSON loader, depends on engine
- `vassnian_game` — binary, UI, wires engine + content together
- ALL game content in `/data/` as JSON, never hardcoded

### Key conventions
- All data structs: `Debug, Clone, Serialize, Deserialize`
- No `unwrap()` in game logic
- Functions under 50 lines
- `serde(default)` on optional fields for backwards compat
- `#[cfg(target_arch = "wasm32")]` guards for platform-specific code

### Building
- `cargo build` — native build
- `cargo test` — run all tests
- `scripts/build_wasm.ps1` — WASM build
- WASM uses `load_embedded_data()` with `include_str!` — new data files must be added there too

## Documentation Map

| Doc | Purpose |
|-----|---------|
| `docs/00_AGENT_RULES.md` | Coding standards, build order |
| `docs/01_ARCHITECTURE_OVERVIEW.md` | System design, crate structure |
| `docs/02_DATA_SCHEMAS.md` | JSON schemas with examples |
| `docs/03_MVP_SPEC.md` | MVP scope (COMPLETE) |
| `docs/05_GAME_DESIGN_REFERENCE.md` | Full game vision |
| `docs/06_DECISIONS_TRACKER.md` | 49+ answered design questions |
| `docs/10_STORY_STRUCTURE.md` | Story phases, arcs, traitor system |
| `docs/13_WORLD_DESIGN.md` | Missions, enemies, terrain, items |
| `docs/14_ROADMAP_TRACKER.md` | Progress tracker with status |
| `docs/storyteller/` | Raw content from storyteller AI — see README inside |

## Multi-AI Pipeline

The user works with multiple AIs. This agent's role is **engineering only**:

| AI | Role | Provides |
|----|------|----------|
| **Storyteller AI** | Stories, lore, dialogue, banter | Story descriptions → full story JSON with tags, choices, companion lines |
| **Balancing AI** | Stats, numbers, scaling | Stat tables, XP curves, damage formulas, gold economy |
| **This agent (you)** | Engineering | Organize content, build engine systems, wire data, ensure scalability |

### Content Workflow

When the user dumps raw content (from another AI or their own notes):

1. **Read it** — understand what it contains
2. **Ask questions** — clarify anything ambiguous
3. **Organize it** — save clean version to `docs/storyteller/` with proper filename
4. **Check engine compatibility** — does the engine support the data format?
5. **Propose changes** — list what engine/data changes are needed, wait for approval
6. **Implement** — only after user says go

### Story Pipeline

1. **User provides** story description (from storyteller AI)
2. **User provides** full story JSON (from storyteller AI) — with tags, choices, banter, items
3. **You validate** the JSON against engine schema
4. **You fix** format issues (wrong field names, missing defaults)
5. **You wire** it into the game (loader, mission catalog, WASM embed)
6. **You never** write or modify story text, dialogue, or banter

### Stats Pipeline

1. **User provides** balance numbers (from balancing AI or their own decisions)
2. **You implement** the config structure in JSON
3. **You wire** the config into engine code (replacing any hardcoded values)
4. **You never** pick balance numbers yourself

## Current Status
- MVP: COMPLETE (Phase 1-8)
- 62 tests pass, 0 warnings
- WASM builds clean
- Next: Demo content (Act 1 stories, bestiary, items, classes)
