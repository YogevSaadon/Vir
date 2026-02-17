# Vassnian Architecture Package — README

## What is this?
This is the complete architecture package for **Vassnian**, a dark fantasy isekai RPG. The player character is known as the **"Godsent"**. Hand all 7 files to the coding agent. The agent should read them in order (00 → 06).

---

## File Index

| File | Purpose | Read When |
|------|---------|-----------|
| `00_AGENT_RULES.md` | **How to work**: coding standards, file organization, build order, communication protocol | FIRST — before writing any code |
| `01_ARCHITECTURE_OVERVIEW.md` | **System design**: crate structure, data flow, key traits, state management, constants | Before starting architecture |
| `02_DATA_SCHEMAS.md` | **JSON schemas**: every data type with Rust structs and example JSON files | When creating data structures or JSON files |
| `03_MVP_SPEC.md` | **What to build**: exact MVP scope, minute-by-minute player experience, systems checklist | Before each build phase |
| `04_UI_SCREENS.md` | **Screen layouts**: ASCII wireframes for most screens, behavior specs, navigation map | When building UI screens |
| `05_GAME_DESIGN_REFERENCE.md` | **Full game vision**: complete translated design doc with all classes, skills, mechanics | For context — NOT all of this is in MVP |
| `06_DECISIONS_TRACKER.md` | **All decisions made**: 49+ answered design questions | When unsure about a design choice |
| `07_SETUP_GUIDE.md` | **Developer setup**: what to install, how to build & run | One-time reference |
| `08_POST_MVP_ROADMAP.md` | **Demo plan**: MVP→Demo progression, skill speed types, content priority order | After MVP, planning demo |
| `08.5_MVP_ENGINE_SUPPLEMENTS.md` | **Engine scaffolding**: systems to build NOW (derived stats, terrain, time, scripts, etc.) even though MVP content won't use them | During MVP Phase 2 (Core Data) |
| `09_NOTES_INBOX.md` | **Notes inbox**: developer drops new notes here, agent processes them into correct docs | Every time developer adds notes |
| `10_STORY_STRUCTURE.md` | **Full story phases**: Phase 0-4.5, traitor system, cross-phase stories, companion arcs, boss pools | For narrative context and story engine design |
| `11_CLASS_SKILL_ADDITIONS.md` | **New classes & skills**: starting passives, Bard, Charisma, Quickstrike, etc. | When building class/skill systems |
| `12_UI_REVISIONS.md` | **REPLACES inventory/shop in 04**: grid-based inventory, revised shop layout, potion mode | When building inventory & shop screens |
| `13_WORLD_DESIGN.md` | **Content systems**: missions, enemies, terrain, items, city services, time, Godsent groups | When building world/content systems |
| `14_ROADMAP_TRACKER.md` | **Progress tracking**: agent marks tasks done here, tracks open questions | Update after EVERY completed task |

---

## Agent Quick Start

1. Read `00_AGENT_RULES.md` completely
2. Read `01_ARCHITECTURE_OVERVIEW.md` completely
3. Read `08.5_MVP_ENGINE_SUPPLEMENTS.md` — systems to scaffold during MVP
4. Read `12_UI_REVISIONS.md` — these REPLACE the inventory/shop screens in 04
5. Follow the **Build Order** in `00_AGENT_RULES.md` Phase 1-8
6. Use `14_ROADMAP_TRACKER.md` to mark progress after each task
7. Check `09_NOTES_INBOX.md` for new developer notes before each session
8. Reference other docs as needed per phase
9. Keep `ARCHITECTURE.md` updated in the project root after every phase

### Important Reading Notes:
- **04_UI_SCREENS.md** is mostly valid BUT the Inventory and Shop screens are **replaced** by `12_UI_REVISIONS.md`
- **08.5** contains systems to scaffold during MVP (not just demo) — read before Phase 2
- **10, 11, 13** are context for demo/full game — skim now, read deeply later
- **14** is your progress tracker — update it constantly

---

## Key Decisions Summary

- **Engine:** Macroquad (Rust, lightweight)
- **Architecture:** 3-crate workspace (vassnian_engine, vassnian_content, vassnian_game)
- **Data:** JSON files in /data/ with serde
- **MVP:** Knight only, 2v2 combat, 2 stories, 1 shop, auto-attack only
- **Platform:** PC first (portrait aspect ratio), mobile later
- **Style:** Placeholder visuals (two-color), audio system wired but silent (no files)
- **Hero Title:** "Godsent" — NPCs address the player this way
