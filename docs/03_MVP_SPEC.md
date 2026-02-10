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

### Minute 0-1: Intro
1. Main menu: "New Game" button
2. God speaks: isekai text (2-3 text boxes explaining you've been pulled into this world, declares you "Godsent")
3. Transition to character creation

### Minute 1-3: Character Creation
4. **Avatar select**: Pick from 3-4 placeholder portraits
5. **Class select**: Knight only (click to see tooltip with description, stats, role)
6. **Stat point-buy**: 
   - 7 stats all start at 1
   - 2 free points to distribute
   - Click any stat → tooltip explains what it does
   - Derived stats update live as you assign points
7. **World skill select**: Pick 1 from 3 (Climbing, Persuasion, Lockpicking)
   - Click any → tooltip explains
8. **Confirm** → Enter world → auto-save

### Minute 3-6: Story Encounter 1
9. Story text appears (dark forest setting)
10. Choices appear:
    - Basic choice (no requirement)
    - Stat-gated choice: "[STR 3] Push the boulder" — shows player's STR value
    - World skill choice: "[Climbing] Scale the wall" — only if player has Climbing
    - Companion world skill: "[Sir Aldric: Climbing] Let Aldric climb" — if companion has it
11. Player picks choices, branches through 3-4 nodes
12. One branch leads to combat trigger

### Minute 6-9: Combat
13. Combat screen loads: 2v2
    - Player Knight (front) + Companion Knight (front)
    - vs 2 Goblin enemies (front)
14. ATB bars fill (~4 seconds)
15. Companion acts via AI (auto-attack nearest)
16. Player's ATB fills → skill bar lights up → player taps attack + picks target
17. Potions available in belt (if player bought any already — they haven't yet)
18. Pause button works
19. Combat resolves: win or lose
    - Win: get gold + EXP, return to story
    - Lose: +1 injury, return to story (different node)

### Minute 9-11: Story Encounter 2 (Shop)
20. New story: meet a shopkeeper NPC
21. Dialogue choices (normal conversation)
22. Choice: "Enter the shop" → opens shop screen
23. Shop shows: HP Potion (10g), Rusty Sword (25g)
24. Player buys items → gold decreases
25. Equip sword from inventory → damage increases for next combat
26. Exit shop → story continues → auto-save

### Minute 11-12: End of MVP
27. Story ends with "To be continued..." or loops back
28. If player dies (3 injuries) → Game Over screen → restart

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

### ❌ NOT in MVP

| System | Why Not |
|--------|---------|
| Multiple classes | Knight only |
| Combat skills | Auto-attack only for MVP |
| Toggle skills | No mana system needed yet |
| Skill loadout screen | No skills to assign yet |
| Skill books | No cross-class learning yet |
| Level up system | Can stub, but no content |
| World level scaling | Only 1 encounter |
| Ranged combat | All melee 2v2 |
| Formation (6v6) | 2v2 front row only |
| Position swapping | Only 2 units, both front |
| Companion quests | Just 1 companion, auto-joins |
| Multiple story paths | Linear for MVP |
| Music/sound | None |
| Ads | Stub interface only |
| Mobile build | PC only |
| WASM build | PC only |
| Bestiary | Future |
| Romance | Future |
| Secret skills | Future |

---

## MVP Data Files to Create

```
data/
├── stories/
│   ├── intro_god.json          # God isekai intro text
│   ├── mvp_story_01.json       # Forest path + combat trigger
│   └── mvp_story_02_shop.json  # Shopkeeper encounter
├── skills/
│   ├── combat_skills.json      # Empty for MVP (auto-attack is hardcoded)
│   └── world_skills.json       # 3 world skills
├── characters/
│   ├── classes.json            # Knight class definition
│   ├── companions.json         # Sir Aldric companion
│   └── enemies.json            # Goblin warrior + scout
├── items/
│   ├── potions.json            # HP potion, Fire potion
│   ├── equipment.json          # Rusty sword
│   └── shop_inventories.json   # Village shop
└── config/
    └── game_config.json        # All tuning constants
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
