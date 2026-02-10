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
| `04_UI_SCREENS.md` | **Screen layouts**: ASCII wireframes for every screen, behavior specs, navigation map | When building UI screens |
| `05_GAME_DESIGN_REFERENCE.md` | **Full game vision**: complete translated design doc with all classes, skills, mechanics | For context — NOT all of this is in MVP |
| `06_DECISIONS_TRACKER.md` | **All decisions made**: 49 answered design questions | When unsure about a design choice |

---

## Agent Quick Start

1. Read `00_AGENT_RULES.md` completely
2. Read `01_ARCHITECTURE_OVERVIEW.md` completely  
3. Follow the **Build Order** in `00_AGENT_RULES.md` Phase 1-8
4. Reference other docs as needed per phase
5. Keep `ARCHITECTURE.md` updated in the project root after every phase

---

## Key Decisions Summary

- **Engine:** Macroquad (Rust, lightweight)
- **Architecture:** 3-crate workspace (vassnian_engine, vassnian_content, vassnian_game)
- **Data:** JSON files in /data/ with serde
- **MVP:** Knight only, 2v2 combat, 2 stories, 1 shop, auto-attack only
- **Platform:** PC first (portrait aspect ratio), mobile later
- **Style:** Placeholder visuals (two-color), audio system wired but silent (no files)
- **Hero Title:** "Godsent" — NPCs address the player this way
