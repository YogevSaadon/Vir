# Game Architecture Decisions — Master Tracker

## Project: **Vassnian**
### Hero Title: **Godsent** — NPCs address the player as "Godsent" throughout the story (isekai identity)

---

## ✅ DECIDED

| # | Topic | Decision |
|---|-------|----------|
| 1 | Engine | **Macroquad** (lightest, mobile-ready, fast) |
| 2 | Data format | **JSON** (with serde) |
| 3 | Demo player class | **Knight only** |
| 4 | Tone | **Dark fantasy** — serious, you die a lot, roguelite. Stakes are real. |
| 5 | Setting | **Isekai** — modern person pulled into classic fantasy. God isekai's you, teaches the ropes, info tooltips on everything you click |
| 6 | Save system | **Single auto-save slot** (roguelite feel) |
| 7 | Skill bar | **4 active skill slots + potion button**. Swappable outside combat. Fewer skills = empty slots |
| 8 | Demo companions | **1 companion**, Knight class, **basic AI** (fallback: player can direct AI) |
| 9 | Monetization | **Free demo (MVP) up to first boss** → **free + ads**. Ad integration layer as stub. |
| 10 | The God | **One-time intro only** |
| 11 | Demo class selection | **Knight only** |
| 12 | Point-buy stats | **Start with 1 in all stats** + free points to distribute. Each point adds correct derived stats |
| 13 | Character creation flow | **Avatar image pick (placeholder for now) → Class pick → stat point-buy → world skill selection → enter world**. Combat skills come with class automatically. |
| 14 | Skill progression | Player levels up: **1 combat skill from choices + 2 world skills from all world skills**. Player can also find skill books of other classes later. Companions: fixed skills, no new ones. |
| 15 | Demo battles | **2v2, all melee** |
| 16 | MVP scope | **NOT a full demo — it's an MVP**. Full demo = all of Act 1. MVP = bare minimum to test all systems |
| 17 | MVP content | **2 story encounters + 1 combat**: Story 1 = uses world skills + companion world skills (e.g., companion good at climbing → option appears). Story 2 = meet shopkeeper + enter store (tests store UI). 1 combat encounter between them. |
| 18 | MVP skills | Knight starts with **2 combat skills only** |
| 19 | Story system | **Data-driven**: each story = own JSON data (choices, implications, tags). World-level data determines next available stories based on: companions, stage progression, story implications, tag words. Inspired by **Life in Adventure** (mobile) |
| 20 | Working name | **Vassnian**. Hero's in-game title: **Godsent** (NPCs address player as this, isekai identity) |
| 21 | World skills | Shown at character creation. Player picks world skills separately from combat skills. Companions also have world skills (e.g., climbing) that unlock dialogue options in stories |

---

## ❓ STILL UNANSWERED

### Potions & Items — ✅ DECIDED
| # | Topic | Decision |
|---|-------|----------|
| 22 | Potion system | **Consumable** (limited stock, destroyed on use, must restock) |
| 23 | Potion types | **HP potion** (target ally), **Fire potion** (target enemy) |
| 24 | Mid-combat use | **Does NOT cost a turn**. **Potion belt** under skill bar holds **max 4 potions per combat**. Destroyed on use, must restock after. |

### Combat Details — ✅ DECIDED (25-28)
| # | Topic | Decision |
|---|-------|----------|
| 25 | ATB speed | **~4 seconds base**, speed stat adjusts slightly |
| 26 | Pause | **Yes** — pause button + menu also pauses combat |
| 27 | Player control model | **No slow-down**. All party members (except player) are AI-controlled. Player only controls: **their own character, potions, and position swapping**. AI hierarchy: basic AI for all → class-specific AI layered on top later |
| 28 | Auto-battle | **No** — player always controls their own character |

### Story System Details — ✅ DECIDED
| # | Topic | Decision |
|---|-------|----------|
| 29 | Choices per encounter | **Story-dependent, open-ended**. Some stories are non-linear (tree-like sub-stories). No fixed number. |
| 30 | Visible tags on choices | **Yes — stats and world skills visible**. Stat checks: show best ally's name + stat value. World skill checks: show skill name. **No odds/percentages**. If you meet the requirement = instant pass. But passing isn't always the best choice. |
| 31 | Surprise combat in dialogue | **Yes** — dialogue IS the story choice mechanic. Same system. Attack shopkeeper, anger a dark knight, etc. Combat can trigger from any choice. |
| 32 | Text display speed | **Player's choice** in options menu (instant, typewriter, etc.) |

### UI & Visual — ✅ DECIDED
| # | Topic | Decision |
|---|-------|----------|
| 33 | Mobile orientation | **Portrait** (phone upright, tall — like reading/texting) |
| 34 | Font style | **Normal readable font** — nothing fancy for now |
| 35 | Color palette | **Two readable colors only** — placeholder aesthetic, easy to read. Full design later. |
| 36 | Loading screen | **None ideally** — Rust + light game should be instant. No loading screen architecture needed. |

### Architecture — ✅ DECIDED
| # | Topic | Decision |
|---|-------|----------|
| 37 | Target FPS | **60 FPS** |
| 38 | Online/offline | **Offline** — no online features |
| 39 | Language | **English only** |
| 47 | Rust project structure | **Workspace with 3+ crates** (engine, content, platform) |
| 48 | Data file organization | **Organized subfolders** (/data/stories, /data/skills, /data/characters) |
| 49 | Game state management | **Agent decides best practice** (recommend state stack for overlay screens like shop) |

### New Questions — ✅ DECIDED
| # | Topic | Decision |
|---|-------|----------|
| 40 | Free stat points | **2 free points** (base 1 in all stats + 2 to distribute) |
| 41 | World skills at creation | **1 world skill** |
| 42 | World skills examples | **3 placeholder world skills for MVP** (e.g., Climbing, Persuasion, Lockpicking). Names don't matter yet — system must be **scalable** so more can be added before demo stories are written. |
| 43 | Shop in MVP | **Yes** — sells 1 potion + 1 sword. That's it for MVP. |
| 44 | Knight starting skills | **NO combat skills for MVP** — just basic auto-attack + simple AI. 1 world skill. |
| 45 | Companion world skill | **1 locked-in world skill, different from player's pick**. Companion doesn't choose — it's preset. |
| 46 | MVP equipment | **No gear at start — hit with bare hands**. Shop is first chance to buy a sword. **Player starts with some gold**. |

---

## 🔮 FUTURE (not MVP, but architecture leaves room)

- All 8 classes with full skill trees
- Full companion quest chains + companion world skills
- Damage types & resistances
- Bestiary
- Romance system
- Roguelite meta-upgrades between runs
- Music & sound
- Advanced animations / injury-state portraits
- 4 areas with themes
- Secret skills (dual-stat unlocks)
- Skill books (cross-class learning)
- Multiple skeleton/summon types
- Ad integration (AdMob/Unity Ads)
- Paywall gate at boss 1

---

*Last updated: Round 5 of Q&A*
