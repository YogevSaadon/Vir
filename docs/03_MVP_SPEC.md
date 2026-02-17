# Vassnian — MVP Specification

## What is the MVP?
The **minimum viable product** to test all core systems end-to-end. NOT the demo (demo = all of Act 1). The MVP proves the engine works.

---

## MVP Content Summary

| Element | Scope |
|---------|-------|
| Classes | Knight only |
| Player combat skills | None — auto-attack only |
| Player world skills | 1 (chosen from 3 options) |
| Companion | 1 Knight companion with 1 different world skill, basic AI |
| Story encounters | 2 (one adventure + one shop meeting) |
| Combat encounters | 1 (2v2, all melee, basic enemies) |
| Shop | 1 (sells 1 potion type + 1 sword) |
| Items | HP potion, Fire potion, Rusty Sword |
| Starting equipment | None (bare fists) |
| Starting gold | ~50 |
| Save system | Auto-save single slot |
| Art | All placeholders (colored rectangles + text labels) |
| Sound | **System wired, files missing** — silent MVP. Drop .ogg files to enable |
| Platform | PC only (Macroquad window, portrait aspect ratio) |

---

## MVP Player Experience (Minute by Minute)

> **Status key:** ✅ = implemented in code, 🔨 = scaffolded/stubbed, ⬜ = not yet coded

### Minute 0-1: God Intro ✅
1. Main menu: "New Game" button ✅
2. God speaks: isekai text (2-3 text boxes, declares you "Godsent") ✅
3. Transition to character creation ✅

### Minute 1-3: Character Creation ✅
4. **Avatar select**: Pick from 3-4 placeholder portraits ✅
5. **Class select**: Knight only (info panel with description, stats, role) ✅
6. **Stat point-buy**: ✅
   - 7 stats start at class base values
   - 2 free points to distribute
   - Stat descriptions shown inline
   - Derived stats update live
7. **World skill select**: Pick 1 from 3 (Climbing, Persuasion, Lockpicking) ✅
   - Fixed description panel below skill list ✅
8. **Confirm** → Enter world → auto-save ✅

### Minute 3-4: Phase 0 — Arrival Sequence ⬜ (NEW — from doc 10)
> This replaces the old "immediately start story encounters" flow.
> See doc 10 § Phase 0 for full details.

9. **Summoning circle** — appear in the new world above a summoning circle ⬜
10. **Other Godsents** — look around, see many other summoned heroes ⬜
11. **The King speaks** — explains the world, why he summoned heroes ⬜
12. **Equipment & gold** — King gives minimal equipment + starting gold ⬜
13. **Companion selection** — choose 1 of 3 companions ⬜
    - MVP: Sir Aldric auto-joins (only 1 companion) ✅ (current code)
14. **Departure** — leave in pairs to the first city ⬜
15. **Bar scene** — companion tells their story, gives advice ⬜
16. **City access** — shops available ⬜

### Minute 4-7: First Mission (Story Encounter 1) ✅
17. Story text appears (dark forest setting) ✅
18. Choices appear: ✅
    - Basic choice (no requirement)
    - Stat-gated choice: "[STR 3] Push the boulder"
    - World skill choice: "[Climbing] Scale the wall"
    - Companion world skill: "[Sir Aldric: Climbing] Let Aldric climb"
19. Player picks choices, branches through 3-4 nodes ✅
20. One branch leads to combat trigger ✅

### Minute 7-10: Combat ✅
21. Combat screen loads: 2v2 ✅
    - Player Knight (front) + Companion Knight (front)
    - vs 2 Goblin enemies (front)
22. ATB bars fill (~4 seconds) ✅
23. Companion acts via AI (auto-attack nearest) ✅
24. Player's ATB fills → skill bar lights up → player taps attack + picks target ✅
25. Potions available in belt (if player bought any) ✅
26. Pause button works ✅
27. Combat resolves: win or lose ✅
    - Win: get gold + EXP, return to story
    - Lose: +1 injury, return to story (different node)

### Minute 10-12: Story Encounter 2 (Shop) ✅
28. New story: meet a shopkeeper NPC ✅
29. Dialogue choices (normal conversation) ✅
30. Choice: "Enter the shop" → opens shop screen ✅
31. Shop shows: HP Potion (10g), Rusty Sword (25g) ✅
32. Player buys items → gold decreases ✅
33. Equip sword from inventory → damage increases ✅
34. Exit shop → story continues → auto-save ✅

### Minute 12-13: End of MVP ✅
35. Story ends with "To be continued..." or loops back ✅
36. If player dies (3 injuries) → Game Over screen → restart ✅

---

## Systems Checklist

### ✅ Must Work in MVP

| System | Details |
|--------|---------|
| Game state manager | Screen transitions, state stack |
| Main menu | New Game, Continue (if save exists), Options |
| Character creation | Avatar → Class → Stats → World Skill → Confirm |
| Tooltip system | Click anything for info panel |
| Story engine | Load JSON, display text, show choices, branch |
| Tag system | Set tags, check tags, filter choices |
| Stat checks | Compare player/companion stats to requirements |
| World skill checks | Check player + companion world skills |
| Combat: ATB | Fill bars, trigger turns, pause/resume |
| Combat: auto-attack | Melee attack, damage formula |
| Combat: AI | Basic AI for companion + enemies |
| Combat: potion belt | 4 slots, instant use, destroys potion |
| Combat: win/lose | Check conditions, distribute rewards, apply injuries |
| Injury system | Track injuries, permadeath at 3 |
| Shop screen | Display items, buy with gold |
| Inventory | View items, equip weapon |
| Equipment | Sword adds stat bonus to damage |
| Gold system | Start with gold, spend/earn |
| Auto-save | Save after story/combat/shop |
| Save/load | Load on Continue |
| AssetRef | Placeholder for missing images (colored rectangle + name label) |
| AudioRef | Audio system wired with silent fallback — drop .ogg files in to enable sound |
| Options | Text speed toggle, music/SFX volume sliders |

### ❌ NOT in MVP (but some are scaffolded)

| System | Status | Notes |
|--------|--------|-------|
| Multiple classes | ⬜ Not in MVP | Knight only (ClassDef supports all classes) |
| Combat skills | 🔨 Scaffolded | SkillCooldown, CastState, SkillSlots, UseSkill action exist |
| Toggle skills | 🔨 Scaffolded | Mana field on Entity, SkillSpeed enum exists |
| Skill loadout screen | ⬜ Not in MVP | SkillSlots (4-slot) struct ready |
| Skill books | ⬜ Not in MVP | SkillExclusivity enum ready |
| Level up system | ⬜ Not in MVP | level + exp fields on Entity |
| World level scaling | ⬜ Not in MVP | level + phase_range on EnemyDef |
| Ranged combat | ⬜ Not in MVP | AttackType::Ranged + FormationRow exist |
| Formation (6v6) | ⬜ Not in MVP | FormationRow::Front/Back ready |
| Position swapping | ⬜ Not in MVP | |
| Companion quests | 🔨 Scaffolded | Party.missed_companion_stories tracking |
| Multiple story paths | 🔨 Scaffolded | StoryPool, MissionDef, mission catalog JSON |
| Music/sound | 🔨 Scaffolded | AudioRef + SilentAudioManager wired |
| Ads | 🔨 Scaffolded | show_rewarded_ad() stub |
| Mobile build | ⬜ Not attempted | |
| WASM build | ✅ Done | Builds clean, web_build/ ready |
| Bestiary | ⬜ Future | |
| Romance | ⬜ Future | |
| Secret skills | ⬜ Future | |
| Phase 0 intro (King, companion select, bar scene) | ⬜ Not coded | See doc 10 |
| Mission selection (3-choice) | 🔨 Scaffolded | MissionDef + MissionOption structs |
| WorldState / time tracking | 🔨 Scaffolded | WorldState struct with phase progression |
| Status effects | 🔨 Scaffolded | StatusEffect + 9 effect types |
| Class passives | 🔨 Scaffolded | ClassPassive + PassiveSubChoice structs |

---

## MVP Data Files

> All files below exist and are loaded. ✅ = has content, 🔨 = placeholder/minimal.

```
data/
├── stories/
│   ├── intro_god.json          ✅ God isekai intro text
│   ├── mvp_story_01.json       ✅ Forest path + combat trigger
│   └── mvp_story_02_shop.json  ✅ Shopkeeper encounter
├── skills/
│   ├── combat_skills.json      ✅ Empty array (auto-attack hardcoded)
│   └── world_skills.json       ✅ 3 world skills
├── characters/
│   ├── classes.json            ✅ Knight class definition
│   ├── companions.json         ✅ Sir Aldric companion
│   └── enemies.json            ✅ Goblin warrior + scout
├── items/
│   ├── potions.json            ✅ HP potion, Fire potion
│   ├── equipment.json          ✅ Rusty sword
│   └── shop_inventories.json   ✅ Village shop
├── missions/
│   └── mission_catalog.json    🔨 4 placeholder missions
└── config/
    ├── game_config.json        ✅ All tuning constants
    └── stat_formulas.json      ✅ Derived stat formulas (physical/spell/heal power)
```

---

## Definition of Done

The MVP is complete when:
1. `cargo run` launches the game in a portrait window
2. Player can go from menu → creation → story → combat → shop → end
3. Save/load works (close game, reopen, continue works)
4. All placeholders display correctly (no crashes on missing assets)
5. Injury system works (lose 3 times = game over)
6. `cargo test --workspace` passes
7. ARCHITECTURE.md is up to date
