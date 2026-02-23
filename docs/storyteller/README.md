# docs/storyteller/ — Game Design Reference

Organized reference docs for the Vassnian game. All content extracted from
`docs/vassnian-all-artifacts/` (raw JSX/MD dumps from the storyteller AI).

---

## Document Index

### Core Reference (read these first)
| File | Contents |
|------|----------|
| `lore_bible.md` | Kingdom, gods, races, Shadow World, acts, Godsent summoning |
| `game_systems.md` | 3 stats, 8 classes, combat, skills, world skills, formations |
| `numbers_and_formulas.md` | HP/Mana/ATB formulas, damage, armor, equipment ranges, enemy power bands |

### Content Databases
| File | Contents |
|------|----------|
| `bestiary_act1.md` | 53 enemies, 19 families, 5 ranks, tier-based stats |
| `basic_items.md` | Weapons, armor, shields, accessories, potions by act with prices |
| `companions.md` | 13 companions (mostly TBD), distribution by act, world skill assignments |
| `traitor_parties.md` | 4 preset traitor compositions with full details |

### Story & Quests
| File | Contents |
|------|----------|
| `main_quests.md` | Full main quest table, Act 0 through Gorath, all boss paths |
| `quest_system.md` | 5 quest types, 10 tag types, priority array, displacement rules |
| `unique_trackers.md` | Unique items/enemies/NPCs (filled during story writing) |

### Planning
| File | Contents |
|------|----------|
| `roadmap.md` | Development phases, dependencies, next steps |

### Engine Integration
| File | Contents |
|------|----------|
| `clean_engine_reference.md` | Current story JSON format the engine supports |
| `story_index.md` | Master list of stories by act with status |
| `story_template.json` | JSON skeleton for new stories |

---

## Raw Artifacts

The original JSX/MD files from the storyteller AI are preserved in
`docs/vassnian-all-artifacts/`. These organized docs are the canonical
reference — use them instead of the raw artifacts.

## Workflow

1. User designs content with storyteller/balancing AI
2. User pastes raw content into `vassnian-all-artifacts/` or chat
3. Coding agent organizes into clean docs here
4. Coding agent checks engine compatibility and proposes changes
5. User approves, agent implements
