# Vassnian — UI Screen Specifications

> **Status key:** ✅ = implemented in code, ⬜ = not yet coded

## Design Principles
- **Portrait orientation** (mobile-first, tall layout)
- **Two-color placeholder palette** for MVP (e.g., dark background + light text)
- **Readable default font** — nothing fancy
- **Tooltip on everything** — tap/click any element to get info panel
- **No loading screens** — instant transitions
- **Touch-friendly** — all buttons large enough for mobile thumbs (min 44px)

---

## Reference Resolution
```
Design target: 390 x 844 (iPhone 14 portrait)
Scale up/down proportionally for other screens
PC window: same aspect ratio, larger
```

---

## Screen 1: Main Menu ✅

```
┌──────────────────────┐
│                      │
│                      │
│        VASSNIAN        │
│   Dark Fantasy RPG    │
│                      │
│                      │
│   ┌──────────────┐   │
│   │  NEW GAME    │   │
│   └──────────────┘   │
│   ┌──────────────┐   │
│   │  CONTINUE    │   │  ← grayed out if no save
│   └──────────────┘   │
│   ┌──────────────┐   │
│   │  OPTIONS     │   │
│   └──────────────┘   │
│                      │
│                      │
└──────────────────────┘
```

**Behavior:**
- New Game → God intro sequence → Character Creation
- Continue → Load auto-save → Resume at last saved point
- Options → Options overlay (text speed setting)

---

## Screen 2: God Intro ✅

```
┌──────────────────────┐
│                      │
│   ┌──────────────┐   │
│   │  God Avatar  │   │
│   │ [placeholder] │   │
│   └──────────────┘   │
│                      │
│  ┌────────────────┐  │
│  │                │  │
│  │  "You are not  │  │
│  │  from this     │  │
│  │  world..."     │  │
│  │                │  │
│  └────────────────┘  │
│                      │
│       [NEXT ▶]       │
│                      │
└──────────────────────┘
```

**Behavior:**
- Series of text boxes (2-3) from the God
- Text speed based on Options setting (typewriter / instant)
- Tap NEXT or tap anywhere to advance
- After last text → transition to Character Creation

---

## Screen 3: Character Creation — Avatar Select ✅

```
┌──────────────────────┐
│  Choose Your Avatar  │
│                      │
│  ┌────┐ ┌────┐      │
│  │ A1 │ │ A2 │      │
│  └────┘ └────┘      │
│  ┌────┐ ┌────┐      │
│  │ A3 │ │ A4 │      │
│  └────┘ └────┘      │
│                      │
│  Selected: [A1]      │
│                      │
│     [CONFIRM ▶]      │
│                      │
└──────────────────────┘
```

**Behavior:**
- Grid of avatar images (placeholders)
- Tap to select (highlighted border)
- Confirm → next step

---

## Screen 4: Character Creation — Class Select ✅

```
┌──────────────────────┐
│   Choose Your Class  │
│                      │
│  ┌──────────────┐    │
│  │   KNIGHT     │ ←  │  ← only option for MVP
│  │  [portrait]  │    │
│  │  Defender    │    │
│  └──────────────┘    │
│                      │
│  ┌────────────────┐  │
│  │ INFO PANEL:    │  │  ← appears on tap
│  │ HP: High      │  │
│  │ DEF: Very High│  │
│  │ ATK: Medium   │  │
│  │ Role: Tank    │  │
│  │ "A wall of    │  │
│  │  steel..."    │  │
│  └────────────────┘  │
│                      │
│     [CONFIRM ▶]      │
│                      │
└──────────────────────┘
```

**Behavior:**
- MVP: only Knight shown (others grayed out / locked icon)
- Tap class → info panel appears below with full details
- Confirm → next step

---

## Screen 5: Character Creation — Stat Point-Buy ✅

```
┌──────────────────────┐
│   Distribute Stats   │
│   Points left: 2     │
│                      │
│  STR  [1] [−] [+] ⓘ │
│  VIT  [1] [−] [+] ⓘ │
│  INT  [1] [−] [+] ⓘ │
│  FAI  [1] [−] [+] ⓘ │
│  SPD  [1] [−] [+] ⓘ │
│  DEX  [1] [−] [+] ⓘ │
│  LCK  [1] [−] [+] ⓘ │
│                      │
│  ── Derived Stats ── │
│  HP: 12   DEF: 2     │
│  ATK: 2   SPD: 1.0   │
│  CRIT: 5% EVA: 3%    │
│                      │
│     [CONFIRM ▶]      │
│                      │
└──────────────────────┘
```

**Behavior:**
- All stats start at 1, 2 free points
- [+] adds point (if points remaining), [-] removes (min 1)
- ⓘ tap → tooltip explaining stat and what it affects
- Derived stats update in real-time
- Confirm only enabled when all points spent
- Confirm → next step

---

## Screen 6: Character Creation — World Skill Select ✅

```
┌──────────────────────┐
│  Choose World Skill  │
│   (Pick 1)           │
│                      │
│  ┌──────────────┐    │
│  │  🧗 Climbing  │ ⓘ │
│  └──────────────┘    │
│  ┌──────────────┐    │
│  │  🗣 Persuasion│ ⓘ │
│  └──────────────┘    │
│  ┌──────────────┐    │
│  │  🔓 Lockpick │ ⓘ │
│  └──────────────┘    │
│                      │
│  ┌────────────────┐  │
│  │ INFO: Climbing │  │
│  │ Scale walls &  │  │
│  │ obstacles.     │  │
│  │ Opens paths    │  │
│  │ others can't   │  │
│  │ reach.         │  │
│  └────────────────┘  │
│                      │
│     [CONFIRM ▶]      │
│                      │
└──────────────────────┘
```

**Behavior:**
- List of 3 world skills
- Tap to select (highlighted)
- ⓘ or tap → tooltip with description
- Confirm → final review or enter world

---

## Screen 6.5: Phase 0 — Arrival Sequence ⬜ (NEW — from doc 10)

> These screens are NOT yet coded. They expand the intro between character creation
> and the first mission. See doc 10 § Phase 0 for full narrative details.

### Screen 6.5a: Summoning Circle ⬜
```
┌──────────────────────┐
│                      │
│  ┌──────────────────┐│
│  │   [Summoning     ││
│  │    Circle BG]    ││
│  └──────────────────┘│
│                      │
│  ┌──────────────────┐│
│  │ You appear above ││
│  │ a great circle   ││
│  │ of light. Around ││
│  │ you, dozens of   ││
│  │ others like you  ││
│  │ materialize...   ││
│  └──────────────────┘│
│                      │
│       [NEXT >]       │
│                      │
└──────────────────────┘
```

**Behavior:**
- Story screen format — text boxes introducing the new world
- Show the player that many Godsents were summoned, not just them
- Tap to advance through 2-3 text boxes
- Transition to King's speech

### Screen 6.5b: King's Speech ⬜
```
┌──────────────────────┐
│                      │
│  ┌──────────────────┐│
│  │  [King Portrait] ││
│  │  [placeholder]   ││
│  └──────────────────┘│
│                      │
│  ┌──────────────────┐│
│  │ "Welcome, brave  ││
│  │ Godsents. Our    ││
│  │ world is under   ││
│  │ siege..."        ││
│  └──────────────────┘│
│                      │
│       [NEXT >]       │
│                      │
└──────────────────────┘
```

**Behavior:**
- King explains the world, the threat, and the Godsent mission
- Gives starting gold + class-appropriate equipment
- 3-4 text boxes
- Transition to companion selection

### Screen 6.5c: Companion Selection ⬜
```
┌──────────────────────┐
│  Choose Companion    │
│  (Pick 1)            │
│                      │
│  ┌──────────────────┐│
│  │ [Portrait]       ││
│  │ Sir Aldric       ││
│  │ Knight - Climbing ││
│  │ "A stoic veteran"││
│  └──────────────────┘│
│  ┌──────────────────┐│
│  │ [Portrait]       ││
│  │ ??? (Locked)     ││
│  │ (Demo)           ││
│  └──────────────────┘│
│  ┌──────────────────┐│
│  │ [Portrait]       ││
│  │ ??? (Locked)     ││
│  │ (Demo)           ││
│  └──────────────────┘│
│                      │
│     [CONFIRM >]      │
│                      │
└──────────────────────┘
```

**Behavior:**
- MVP: only Sir Aldric available (others locked for Demo)
- Show companion portrait, name, class, world skill
- Info panel with personality description
- Confirm → companion joins party

### Screen 6.5d: Bar Scene ⬜
```
┌──────────────────────┐
│                      │
│  ┌──────────────────┐│
│  │  [Tavern BG]     ││
│  └──────────────────┘│
│                      │
│  ┌──────────────────┐│
│  │ Sir Aldric leans ││
│  │ forward. "Let me ││
│  │ tell you about   ││
│  │ this world..."   ││
│  └──────────────────┘│
│                      │
│ ┌──────────────────┐ │
│ │ "Tell me about   │ │
│ │ the King"        │ │
│ └──────────────────┘ │
│ ┌──────────────────┐ │
│ │ "What's the      │ │
│ │ threat?"         │ │
│ └──────────────────┘ │
│                      │
└──────────────────────┘
```

**Behavior:**
- Story screen format — companion tells their backstory
- Dialogue choices let player learn about the world
- After bar scene → city access → first mission selection
- Uses standard story engine (JSON story file)

---

## Screen 7: Story Screen ✅

```
┌──────────────────────┐
│  ┌──────────────────┐│
│  │                  ││
│  │   [Background    ││
│  │    Image /       ││
│  │    Placeholder]  ││
│  │                  ││
│  └──────────────────┘│
│                      │
│  ┌──────────────────┐│
│  │ You wake up in a ││
│  │ dark forest. The ││
│  │ trees loom above ││
│  │ you, branches    ││
│  │ clawing at a     ││
│  │ bruised sky.     ││
│  └──────────────────┘│
│                      │
│ ┌──────────────────┐ │
│ │ Take the left    │ │  ← basic choice
│ │ path             │ │
│ └──────────────────┘ │
│ ┌──────────────────┐ │
│ │🧗 Scale the wall │ │  ← world skill (green if have)
│ │  [You: Climbing] │ │
│ └──────────────────┘ │
│ ┌──────────────────┐ │
│ │💪 Push boulder   │ │  ← stat check
│ │  [STR 3 — You:2] │ │  ← red if not enough
│ └──────────────────┘ │
│                      │
└──────────────────────┘
```

**Behavior:**
- Top: background image (placeholder)
- Middle: story text (typewriter or instant per settings)
- Bottom: choice buttons
- Choices show:
  - No icon = basic choice (always available)
  - Stat icon = stat check — shows required value + best ally's value
    - Green text if met, red/gray if not (choice hidden or shown as locked)
  - Skill icon = world skill — shows who has it
    - "[You: Climbing]" or "[Aldric: Climbing]"
  - If requirement not met → choice is **hidden** (not shown)
- Tap choice → apply tags → go to next node
- If choice triggers combat → transition to combat screen

---

## Screen 8: Combat Screen ✅

```
┌──────────────────────┐
│  ┌──────────────────┐│
│  │   [Background]   ││
│  └──────────────────┘│
│                      │
│    ENEMIES           │
│  ┌──────┐ ┌──────┐  │
│  │Goblin│ │Goblin│  │
│  │ W.   │ │ S.   │  │
│  │HP██░░│ │HP████│  │
│  │ATB═══│ │ATB══ │  │
│  └──────┘ └──────┘  │
│                      │
│    YOUR PARTY        │
│  ┌──────┐ ┌──────┐  │
│  │Player│ │Aldric│  │
│  │Knight│ │Knight│  │
│  │HP████│ │HP████│  │
│  │ATB══ │ │ATB═══│  │
│  └──────┘ └──────┘  │
│                      │
│  ── Skill Bar ──     │
│  [ATK] [ ] [ ] [ ]  │  ← 4 slots, only 1 filled for MVP
│                      │
│  ── Potion Belt ──   │
│  [HP] [🔥] [ ] [ ]  │  ← 4 slots
│                      │
│  [⏸ PAUSE]           │
│                      │
└──────────────────────┘
```

**Behavior:**
- All ATB bars fill simultaneously in real-time
- When enemy/companion ATB fills → they act automatically (AI)
- When player ATB fills → skill bar highlights, player must tap a skill then a target
  - Tap skill → tap enemy to target
  - If no target needed (e.g., self-buff) → executes immediately
- Potion belt: tap potion → tap target → instant use (no turn cost)
  - Potion disappears from belt after use
- Pause: freezes all ATB, shows pause overlay
- Visual feedback:
  - Portrait shakes on hit
  - HP bar color (green → yellow → red)
  - Flash on damage/heal
  - White highlight on selected target
- Win: all enemies at 0 HP → victory screen → rewards → back to story
- Lose: all allies at 0 HP → defeat → +1 injury → back to story

---

## Screen 9: Shop Screen ✅

```
┌──────────────────────┐
│  Village Shop        │
│  Gold: 50            │
│                      │
│  ┌──────────────────┐│
│  │ 🧪 HP Potion    ││
│  │ Heals 30 HP     ││
│  │ 10g    [BUY]    ││
│  └──────────────────┘│
│  ┌──────────────────┐│
│  │ ⚔️ Rusty Sword   ││
│  │ +2 STR          ││
│  │ 25g    [BUY]    ││
│  └──────────────────┘│
│                      │
│                      │
│                      │
│     [← LEAVE]        │
│                      │
└──────────────────────┘
```

**Behavior:**
- Shows shop name + player gold
- Each item: icon, name, description, price, buy button
- Tap item → tooltip with full details
- BUY → deduct gold, add to inventory
- BUY grayed out if not enough gold
- Leave → return to story
- Stock tracking (sword: 1 in stock, can't buy twice)

---

## Screen 10: Inventory Screen ✅ (Revised to grid UI — see doc 12)

```
┌──────────────────────┐
│  Inventory           │
│  Gold: 25            │
│                      │
│  ── Equipment ──     │
│  Weapon: [Rusty Sword] │
│  Armor:  [empty]     │
│  Acc:    [empty]     │
│                      │
│  ── Potions ──       │
│  🧪 HP Potion x2    │
│  🔥 Fire Potion x1  │
│                      │
│  ── Potion Belt ──   │
│  [HP] [ ] [ ] [ ]   │
│  Drag potions here   │
│                      │
│     [← BACK]         │
│                      │
└──────────────────────┘
```

**Behavior:**
- Shows equipped items, potions, belt
- Tap equipment slot → list of equippable items → tap to equip
- Tap potion → add to belt (if belt has room)
- Tap belt slot → remove potion back to inventory
- Accessible from story screen (menu button) and between combats

---

## Screen 11: Stats Screen ✅

```
┌──────────────────────┐
│  Player Name         │
│  [Avatar] Knight Lv1 │
│                      │
│  ── Primary Stats ── │
│  STR: 3    VIT: 1  ⓘ│
│  INT: 1    FAI: 1  ⓘ│
│  SPD: 1    DEX: 1  ⓘ│
│  LCK: 1           ⓘ│
│                      │
│  ── Derived Stats ── │
│  HP: 18/18  MP: 10   │
│  ATK: 5    DEF: 4    │
│  CRIT: 5%  EVA: 3%   │
│                      │
│  ── World Skills ──  │
│  🧗 Climbing         │
│                      │
│  ── Combat Skills ── │
│  (none)              │
│                      │
│  Injuries: 0/3 💀    │
│                      │
│     [← BACK]         │
│                      │
└──────────────────────┘
```

---

## Screen 12: Options Menu ✅

```
┌──────────────────────┐
│  Options             │
│                      │
│  Text Speed:         │
│  [Instant] [Medium]  │
│  [Slow]              │
│                      │
│  Music Volume:       │
│  [████████░░] 70%    │
│                      │
│  SFX Volume:         │
│  [██████████] 100%   │
│                      │
│     [← BACK]         │
│                      │
└──────────────────────┘
```

---

## Screen 13: Game Over ✅

```
┌──────────────────────┐
│                      │
│                      │
│     YOU HAVE DIED    │
│                      │
│   Injuries: 3/3 💀   │
│                      │
│  Your journey ends   │
│  here... for now.    │
│                      │
│   ┌──────────────┐   │
│   │  NEW GAME    │   │
│   └──────────────┘   │
│   ┌──────────────┐   │
│   │  MAIN MENU   │   │
│   └──────────────┘   │
│                      │
│                      │
└──────────────────────┘
```

---

## Navigation Map (Updated)

```
Main Menu ✅
├── New Game → God Intro ✅ → Character Creation ✅
│   ├── Avatar Select ✅
│   ├── Class Select ✅
│   ├── Stat Point-Buy ✅
│   └── World Skill Select ✅
│       └── Phase 0 Arrival Sequence ⬜ (NEW)
│           ├── Summoning Circle ⬜
│           ├── King's Speech ⬜
│           ├── Companion Selection ⬜ (MVP: Aldric auto-joins ✅)
│           └── Bar Scene ⬜
│               └── City / First Mission → Story Screen ✅
│                   ├── Choices → branch within story ✅
│                   ├── → Combat Screen ✅
│                   │   ├── Win → rewards → Story Screen ✅
│                   │   └── Lose → +injury → Story Screen ✅
│                   │       └── 3 injuries → Game Over ✅
│                   ├── → Shop Screen ✅
│                   │   └── Leave → Story Screen ✅
│                   └── Menu (overlay) ✅
│                       ├── Inventory ✅
│                       ├── Stats ✅
│                       └── Options ✅
├── Continue → Resume Story/Combat ✅
└── Options → Options Screen ✅
```

> **Note:** The Phase 0 arrival sequence (summoning circle, King, companion
> selection, bar scene) is the NEW intro flow from doc 10. Currently the game
> skips straight from character creation to story encounters. The Phase 0
> screens use the standard story engine and can be implemented as JSON story
> files when ready.
