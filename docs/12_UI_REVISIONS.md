# Vassnian — 12 UI Revisions

## Changes from Original Spec (04_UI_SCREENS.md)

These revisions **replace** the inventory and shop screens in 04_UI_SCREENS.md. The original list-based inventory is now a **grid/squares** system.

> **Build order:** Build regular Inventory FIRST, then Shop UI on top of it.

---

## Revised: Inventory Screen

```
┌──────────────────────────┐
│  ── CHARACTER ──         │
│  ┌────────────────────┐  │
│  │  [Avatar]  Knight  │  │
│  │  HP: 18  ATK: 5    │  │
│  │                    │  │
│  │  Equipment:        │  │
│  │  [⚔️] [🛡️] [💍]   │  │  ← weapon / armor / accessory slots (squares)
│  │                    │  │
│  └────────────────────┘  │
│                          │
│  ── BAG ──          🔽  │
│  Categories: [All] [Equip] [Potions] [Items] │
│  ┌──┐┌──┐┌──┐┌──┐┌──┐  │
│  │⚔️││🧪││🧪││🔑││  │  │  ← grid of squares (scrollable)
│  └──┘└──┘└──┘└──┘└──┘  │
│  ┌──┐┌──┐┌──┐┌──┐┌──┐  │
│  │  ││  ││  ││  ││  │  │
│  └──┘└──┘└──┘└──┘└──┘  │
│                          │
│  ┌────────────────────┐  │
│  │ Rusty Sword        │  │  ← info panel (appears on click)
│  │ +2 STR  Lv.1       │  │
│  │ [EQUIP] [DROP]     │  │
│  └────────────────────┘  │
│                          │
│  ── POTION BELT ──      │  ← always fixed at bottom
│  [🧪] [🔥] [ ] [ ]     │
│                          │
│  ── PARTY ──            │
│  (👤)(👤)(👤)(👤)       │  ← click face to switch character
│                          │
└──────────────────────────┘
```

### Behavior:
- **Top half:** Selected character's info + equipped items (as squares)
- **Bottom half:** Bag contents as a grid of squares
- **Categories sidebar:** Filter bag view by: All, Equipment, Potions, Items (keys etc.)
- **Click any bag item →** info panel shows: name, stats, description, level
  - If in bag: shows [EQUIP] button (or [USE] for potions)
  - If equipped on character: shows [UNEQUIP] button
- **Potion belt:** Always visible at bottom. Tap potion in bag → adds to belt. Tap belt slot → removes back to bag.
- **Party bar:** Character faces at bottom. Click face to switch which character's equipment you're viewing.

---

## Revised: Shop Screen

```
┌──────────────────────────┐
│  Village Shop    Gold: 25│
│                          │
│  ── SHOP STOCK ──       │
│  Categories: [Equip] [Potions] [Items] │
│  ┌──┐┌──┐┌──┐┌──┐┌──┐  │
│  │⚔️││🧪││🧪││🛡️││  │  │  ← shop inventory (squares)
│  └──┘└──┘└──┘└──┘└──┘  │
│                          │
│  ── YOUR BAG ──         │
│  ┌──┐┌──┐┌──┐┌──┐┌──┐  │
│  │⚔️││🧪││🔑││  ││  │  │  ← your inventory (squares)
│  └──┘└──┘└──┘└──┘└──┘  │
│                          │
│  ┌────────────────────┐  │
│  │ Rusty Sword        │  │  ← info panel (appears on click)
│  │ +2 STR  Lv.1  25g  │  │
│  │ [BUY] or [SELL]    │  │
│  │ [EQUIP]            │  │
│  └────────────────────┘  │
│                          │
│  ── CHARACTER EQUIP ──  │  ← selected character's equipment
│  [⚔️] [🛡️] [💍]        │
│                          │
│  ── PARTY ──            │
│  (👤)(👤)(🏪)           │  ← faces + shop icon
│                          │
│  [← LEAVE SHOP]         │
└──────────────────────────┘
```

### Behavior:
- **Top:** Shop stock as grid of squares
- **Middle:** Your bag as grid of squares
- **Bottom:** Selected character's equipment + party bar
- **Party bar:** Click a character face → view their equipment (can unequip from here)
- **Shop icon** in party bar represents the shop itself
- **Categories (side/top):** Potions, Equipment, Items
  - When **Potions** category is selected: instead of bag, the **potion belt** appears (buy/sell potions directly to/from belt)
- **Click shop item →** info panel: name, stats, price → [BUY] button
- **Click bag item →** info panel: name, stats, sell price → [SELL] button, [EQUIP] button
- **Click equipped item →** [UNEQUIP] button
- **BUY** grayed out if not enough gold
- **SELL** shows sell price

### Potion Mode (when Potions category selected):
```
┌──────────────────────────┐
│  ── SHOP POTIONS ──     │
│  ┌──┐┌──┐┌──┐          │
│  │🧪││🔥││💚│ ...       │  ← potions for sale
│  └──┘└──┘└──┘          │
│                          │
│  ── YOUR BELT ──        │
│  [🧪] [🔥] [ ] [ ]     │  ← your potion belt (replaces bag view)
│                          │
│  Buy directly to belt    │
│  Sell directly from belt │
└──────────────────────────┘
```

---

## Item Info Panel (Universal)

Clicking ANY item square (in inventory, shop, or equipped) shows an info panel:

```
┌────────────────────────┐
│ ⚔️ Rusty Sword         │
│ Level: 1  Rarity: Common│
│ +2 Strength            │
│                        │
│ "A dull blade, but     │
│  better than fists."   │
│                        │
│ [EQUIP] [SELL: 10g]    │
└────────────────────────┘
```

Contents depend on item type:
- **Weapon:** damage, stat bonuses
- **Potion:** heal/damage amount, target
- **Key/Quest Item:** description only
- **Crafting Material:** description + where to use it

---

## City Services Screens (Future — Not MVP)

In the main city, two NPCs unlock over time:

### Blacksmith
- Initially empty (no options)
- Finding certain items in missions unlocks crafting recipes
- In Phase 3: can build things from rare Shadow World materials
- UI: similar to shop but shows "recipes" instead of stock

### Mage
- Initially empty
- Finding crafting materials (magic orbs, etc.) unlocks options
- Can create special items from materials
- UI: similar to blacksmith

### Trainer (In Missions)
- Some missions have trainers who teach skills for gold
- UI: skill list with prices, click to learn (if requirements met)
- Not a shop screen — appears as a story choice that opens a skill purchase panel

---

## Updated Options Screen

```
┌──────────────────────────┐
│  Options                 │
│                          │
│  Text Speed:             │
│  [Instant] [Medium] [Slow]│
│                          │
│  Music Volume:           │
│  [████████░░] 70%        │
│                          │
│  SFX Volume:             │
│  [██████████] 100%       │
│                          │
│     [← BACK]             │
└──────────────────────────┘
```
