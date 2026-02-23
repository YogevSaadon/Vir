# Vassnian Development Roadmap

Flow: Systems (done) -> Fix Docs -> Pre-Story -> Story Descriptions -> Refinement

---

## Completed (9 items)
- Lore Bible v3 (canonical, locked)
- Core Systems (3 stats, 8 classes, skills, formation, mana, no crits)
- Numbers & Formulas (HP, Mana, ATB, armor, equipment ranges, enemy power bands)
- World Skills (13 locked, target 18)
- Quest System Template (5 types, 10 tags, priority array)
- Act 1 Bestiary (53 enemies, 19 families, 5 ranks)
- 13 Boss Designs (4 per act + Gorath)
- 4 Traitor Party Compositions (with full details)
- Main Quest Table (Act 0 through Gorath, ~40 quests)

## Needs Fixing (2 items)
1. **Numbers Doc — Boss Count:** Says 16, should say 13. Add Act 1.5/2.5.
2. **Companion Roster — Full Rebuild:** 13 companions need: names, races, personalities, world skills, handicaps, recruitment concepts.

---

## Phase 1: Pre-Story Foundations

| ID | Task | Priority | Depends On |
|----|------|----------|------------|
| 1A | Companion Roster v2 (13 fully designed) | CRITICAL | — |
| 1B | NPC Roster (recurring named NPCs) | High | — |
| 1C | Basic Items Catalog (shop/drop per act) | High | — |
| 1D | Acts 2-4 Bestiary | Medium | — |
| 1E | Story Format Spec (writer agent format) | CRITICAL | 1A |

## Phase 2: Story Descriptions

| ID | Task | Priority | Depends On | Parallel |
|----|------|----------|------------|----------|
| 2A | Main Quest Descriptions | CRITICAL | 1E | 2E,2F,2G |
| 2B | Companion Quest Descriptions (26 quests) | High | 1A,1E | 2E,2F,2G |
| 2C | Continuation Quest Descriptions | High | 1A,1B,1E | 2E,2F,2G |
| 2D | One-Shot Quest Descriptions (5-10/act) | Medium | 1E | 2E,2F,2G |
| 2E | Unique Items Tracker (parallel fill) | Parallel | — | — |
| 2F | Unique Enemies Tracker (parallel fill) | Parallel | — | — |
| 2G | Unique NPCs Tracker (parallel fill) | Parallel | — | — |

## Phase 3: Refinement & Handoff

| ID | Task | Priority | Depends On |
|----|------|----------|------------|
| 3A | Consistency Pass | High | 2A-2D |
| 3B | Balance Review | Medium | 2E,2F,3A |
| 3C | Writer Agent Briefing Package | CRITICAL | 3A,3B |
| 3D | Coding Agent Architecture Package | CRITICAL | 3A,3B |
| 3E | Fill Remaining Skill Gaps (12 skills) | Low | — |

---

## Story Description Format (Target)
Each story description contains:
- Scene Description — what happens, who, where
- Implications — consequences, setups
- Tags — checked (prerequisites) and set (consequences)
- Image Tags — named scene images (e.g. forest_path_01)
- Unique Refs — items/enemies/NPCs introduced (added to trackers)
