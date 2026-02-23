# Vassnian / Numbers & Formulas (DRAFT)
## All values are starting points — balance in gameplay later.

---

# GAME STRUCTURE

| Act | Sub-Acts | Level Range | Boss |
|-----|----------|-------------|------|
| Act 0 | None (intro/town only) | 1 | None |
| Act 1 | 1.1, 1.2, 1.3 | 1-5 | Act 1 Boss (~Lv5) |
| Act 2 | 2.1, 2.2, 2.3 | 6-10 | Act 2 Boss (~Lv10) |
| Act 3 | 3.1, 3.2, 3.3 | 11-15 | Act 3 Boss (~Lv15) |
| Act 4 | 4.1, 4.2, 4.3 | 16-20 | FINAL BOSS (~Lv20) |

12 mini-bosses + 4 act bosses = **16 boss fights total.**

---

# STAT FORMULAS

## HP = 6 + (STR × Level)

| Example | STR | Level | HP |
|---------|-----|-------|-----|
| Mage Lv1 | 3 | 1 | 9 |
| Warrior Lv1 | 6 | 1 | 12 |
| Paladin Lv1 | 5 | 1 | 11 |
| Mage Lv10 | 4 | 10 | 46 |
| Warrior Lv10 | 9 | 10 | 96 |
| Mage Lv20 | 5 | 20 | 106 |
| Warrior Lv20 | 12 | 20 | 246 |
| Knight Lv20 | 13 | 20 | 266 |

Constitution passive adds flat HP on top (e.g., +10/+20/+30 per level).

## Mana = INT + Level

Small pool. Mana doesn't recharge in combat, so every point matters.

| Example | INT | Level | Mana |
|---------|-----|-------|------|
| Warrior Lv1 | 3 | 1 | 4 |
| Mage Lv1 | 6 | 1 | 7 |
| Bard Lv1 | 5 | 1 | 6 |
| Warrior Lv10 | 3 | 10 | 13 |
| Mage Lv10 | 10 | 10 | 20 |
| Warrior Lv20 | 4 | 20 | 24 |
| Mage Lv20 | 13 | 20 | 33 |
| Priest Lv20 | 12 | 20 | 32 |

Mana Well passive adds flat bonus (e.g., +3/+5/+8).
Passive toggles lock 2-4 mana each while active.

## ATB Speed = 10 + DEX

Higher = your turn comes faster. Everyone gets a turn, DEX determines how quickly.

| Example | DEX | ATB Speed |
|---------|-----|-----------|
| Knight (DEX 3) | 3 | 13 |
| Warrior (DEX 5) | 5 | 15 |
| Rogue (DEX 6) | 6 | 16 |
| Ranger (DEX 6) | 6 | 16 |
| Max DEX build (DEX 13) | 13 | 23 |

## Evasion (from DEX, passive required)

**Not dodge chance** (no randomness). Works as flat physical damage reduction.
Only active with the Evasion passive (Ranger, Rogue).

Evasion value = DEX / 2 (round down)

| Level | DEX | Evasion (damage reduced) |
|-------|-----|--------------------------|
| Lv1 | 6 | 3 |
| Lv10 | 9 | 4 |
| Lv20 | 13 | 6 |

Small but meaningful — shaves damage off every hit. Stacks with armor if wearing light armor.

---

# DAMAGE FORMULAS

## Physical Damage = Weapon Base + Stat

- **STR weapons** (swords, axes, maces, 2-handers): Weapon Base + STR
- **DEX weapons** (daggers, bows, light blades): Weapon Base + DEX
- **INT weapons** (Energy Sword passive only): Weapon Base + INT
- Without the matching weapon passive, damage = Weapon Base only (no stat bonus).

## Skill Damage = Skill Base + (Stat × Multiplier)

Each skill has its own base damage and multiplier at each level.

Example — Fireball:
| Level | Base | Multiplier | INT 6 | INT 10 | INT 13 |
|-------|------|------------|-------|--------|--------|
| Lv1 | 4 | ×1 | 10 | 14 | 17 |
| Lv2 | 6 | ×1.5 | 15 | 21 | 25 |
| Lv3 | 8 | ×2 | 20 | 28 | 34 |

Example — Double Attack (each hit):
| Level | Base | Multiplier | STR 6 | STR 10 | STR 13 |
|-------|------|------------|-------|--------|--------|
| Lv1 | 1 | ×0.5 | 4 | 6 | 7 |
| Lv2 | 2 | ×0.6 | 5 | 8 | 9 |
| Lv3 (free) | 2 | ×0.7 | 6 | 9 | 11 |

Two hits, so total damage is doubled. At Lv3 with STR 10: 9×2 = 18 for 0 mana.

## Heal Amount = Heal Base + INT

Same formula as damage but restores HP instead.

---

# DEFENSE FORMULAS

## Armor (LoL-style diminishing returns)

**Damage Reduction = Armor / (Armor + 10)**

| Armor | Reduction |
|-------|-----------|
| 0 | 0% |
| 2 | 17% |
| 5 | 33% |
| 10 | 50% |
| 15 | 60% |
| 20 | 67% |
| 30 | 75% |

Damage taken = Raw Damage × (1 - Reduction)

So 20 raw damage vs 10 armor = 20 × 0.5 = 10 damage taken.

Armor comes from equipment (heavy armor, medium armor, light armor).

## Magic Resistance (same formula, rare)

**Magic Reduction = MR / (MR + 10)**

Same math as armor. MR is rare — comes from Bard aura, rings, special gear.
Most enemies have 0 MR. Most players have 0 MR unless they invest.

## Shield (Knight only, flat reduction)

Shields give a flat number subtracted from physical damage BEFORE armor.

| Shield | Flat Reduction |
|--------|---------------|
| Basic Shield | 1 |
| Iron Shield | 2 |
| Tower Shield | 3 |
| Endgame Shield | 5 |

Example: 20 raw damage → Shield blocks 3 → 17 goes to armor → 10 armor = 50% → 8.5 final damage.

Shields don't affect spells (only physical).

---

# MANA COSTS (Draft Ranges)

## Spell Costs by Tier

| Tier | Mana Cost | Examples |
|------|-----------|---------|
| Free (Lv3) | 0 | Magic Missile Lv3, Double Attack Lv3, Double Shot Lv3 |
| Cheap | 1-2 | Buffs, debuffs, basic heals |
| Medium | 3-4 | Fireball, Smite, AOE heal, Stealth |
| Expensive | 5-6 | Arcane Burst, Revive, Arrow Storm |
| Very Expensive | 7-8 | Ultimate-tier skills |

With a Mage at 33 max mana, 2 passives locked (4 mana each = 8 locked), that leaves 25 usable mana:
- 6-8 medium spells, OR
- 4 expensive + 2 cheap, OR
- Mix of free basics + a few big spells

Feels tight. Every cast matters. Good.

## Passive Toggle Mana Lock

| Passive Tier | Mana Locked |
|-------------|-------------|
| Utility (1-Handed, Bow) | 2 |
| Defensive (Shield, Evasion, Constitution) | 3 |
| Power (Arcane Focus, Execute, Backstab) | 4 |
| Special (Animal Companion, Rally Aura) | 3 |
| Resource (Mana Well) | 0 (Mana Well ADDS mana, doesn't lock it) |

So a Rogue with 1-Handed (2) + Evasion (3) + Backstab (4) = 9 mana locked.
At Lv10 with INT 5: Mana = 15, minus 9 locked = 6 for skills. Tight but workable.

---

# EQUIPMENT VALUE RANGES BY ACT

## Weapons

| Act | Weapon Base | Example |
|-----|------------|---------|
| Act 1 (Lv1-5) | 1-3 | Rusty Sword (1), Short Sword (2), Fine Blade (3) |
| Act 2 (Lv6-10) | 4-6 | War Sword (4), Enchanted Blade (5), Rare Drop (6) |
| Act 3 (Lv11-15) | 7-9 | Shadow Steel (7), Blessed Weapon (8), Boss Drop (9) |
| Act 4 (Lv16-20) | 10-12 | Legendary (10), Artifact (11), Final Weapon (12) |

## Armor

| Act | Armor Value | Light | Medium | Heavy |
|-----|------------|-------|--------|-------|
| Act 1 | 1-4 | 1 | 2 | 3-4 |
| Act 2 | 4-8 | 4 | 5-6 | 7-8 |
| Act 3 | 8-14 | 8 | 10 | 12-14 |
| Act 4 | 14-20 | 14 | 16 | 18-20 |

---

# ENEMY POWER BANDS BY SUB-ACT

## Act 1 (Player Lv 1-5)

| Sub-Act | Enemy HP | Enemy Damage | Enemy Armor | Mini-Boss HP |
|---------|----------|--------------|-------------|-------------|
| 1.1 | 6-10 | 2-3 | 0 | 25 |
| 1.2 | 10-16 | 3-5 | 1-2 | 40 |
| 1.3 | 16-22 | 5-7 | 2-3 | 55 |
| Act 1 Boss | — | — | — | 80 |

## Act 2 (Player Lv 6-10)

| Sub-Act | Enemy HP | Enemy Damage | Enemy Armor | Mini-Boss HP |
|---------|----------|--------------|-------------|-------------|
| 2.1 | 25-35 | 8-10 | 3-4 | 100 |
| 2.2 | 35-50 | 10-14 | 4-6 | 140 |
| 2.3 | 50-65 | 14-18 | 6-8 | 180 |
| Act 2 Boss | — | — | — | 250 |

## Act 3 (Player Lv 11-15)

| Sub-Act | Enemy HP | Enemy Damage | Enemy Armor | Mini-Boss HP |
|---------|----------|--------------|-------------|-------------|
| 3.1 | 65-85 | 18-22 | 8-10 | 280 |
| 3.2 | 85-110 | 22-28 | 10-12 | 350 |
| 3.3 | 110-140 | 28-34 | 12-14 | 420 |
| Act 3 Boss | — | — | — | 550 |

## Act 4 (Player Lv 16-20)

| Sub-Act | Enemy HP | Enemy Damage | Enemy Armor | Mini-Boss HP |
|---------|----------|--------------|-------------|-------------|
| 4.1 | 140-180 | 34-40 | 14-16 | 500 |
| 4.2 | 180-220 | 40-48 | 16-18 | 600 |
| 4.3 | 220-260 | 48-55 | 18-20 | 700 |
| FINAL BOSS | — | — | — | 1000+ |

---

# SAMPLE COMBAT: Level 1 Party vs 1.1 Slimes

**Party:**
- Warrior (STR 6, DEX 3, INT 3): HP 12, Mana 4, Attack 8 (Short Sword 2 + STR 6)
- Mage (STR 3, DEX 3, INT 6): HP 9, Mana 7, Attack 9 (Magic Missile 3 + INT 6)
- Knight (STR 5, DEX 3, INT 4): HP 11, Mana 5, Attack 7 (Short Sword 2 + STR 5)
- Priest (STR 3, DEX 3, INT 6): HP 9, Mana 7, Attack 5 (Staff, no weapon passive)

**Enemies:** 3 Slimes (8 HP each, 2 damage, 0 armor)

**Round 1:**
- Warrior attacks Slime A: 8 damage → dead
- Mage casts Magic Missile on Slime B: 9 damage → dead
- Knight attacks Slime C: 7 damage → 1 HP left
- Priest attacks Slime C: 5 damage → dead

Clean wipe, no one takes damage. **1.1 feels easy. Good.**
If slimes go first: party takes 6 total damage (2×3). Nobody is in danger.

---

# FORMULA SUMMARY

| What | Formula |
|------|---------|
| HP | 6 + (STR × Level) |
| Mana | INT + Level |
| ATB Speed | 10 + DEX |
| Physical Damage | Weapon Base + STR or DEX |
| Spell Damage | Skill Base + (INT × Multiplier) |
| Heal Amount | Heal Base + INT |
| Armor Reduction | Armor / (Armor + 10) |
| Magic Resistance | MR / (MR + 10) |
| Shield | Flat subtraction before armor |
| Evasion | DEX / 2 flat reduction (passive req.) |

All values are DRAFT. Balance through playtesting.
