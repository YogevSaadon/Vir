# Vassnian Numbers & Formulas

All values are DRAFT — balance through playtesting.

---

## Game Structure

| Act | Sub-Acts | Level Range | Boss |
|-----|----------|-------------|------|
| 0 | None (intro) | 1 | None |
| 1 | 1.1, 1.2, 1.3 | 1-5 | Act Boss (~Lv5) |
| 2 | 2.1, 2.2, 2.3 | 6-10 | Act Boss (~Lv10) |
| 3 | 3.1, 3.2, 3.3 | 11-15 | Act Boss (~Lv15) |
| 4 | 4.1, 4.2, 4.3 | 16-20 | FINAL BOSS (~Lv20) |

**13 bosses total:** 4 Act 1 + 4 Act 2 + 4 Act 3 (traitors) + Gorath

---

## Stat Formulas

### HP = 6 + (STR x Level)

| Example | STR | Level | HP |
|---------|-----|-------|-----|
| Mage Lv1 | 3 | 1 | 9 |
| Warrior Lv1 | 6 | 1 | 12 |
| Mage Lv10 | 4 | 10 | 46 |
| Warrior Lv10 | 9 | 10 | 96 |
| Mage Lv20 | 5 | 20 | 106 |
| Warrior Lv20 | 12 | 20 | 246 |
| Knight Lv20 | 13 | 20 | 266 |

Constitution passive adds flat HP on top.

### Mana = INT + Level

| Example | INT | Level | Mana |
|---------|-----|-------|------|
| Warrior Lv1 | 3 | 1 | 4 |
| Mage Lv1 | 6 | 1 | 7 |
| Mage Lv10 | 10 | 10 | 20 |
| Mage Lv20 | 13 | 20 | 33 |

Mana Well passive adds flat bonus (+3/+5/+8). Passive toggles lock 2-4 mana each.

### ATB Speed = 10 + DEX

| Example | DEX | ATB |
|---------|-----|-----|
| Knight (DEX 3) | 3 | 13 |
| Rogue (DEX 6) | 6 | 16 |
| Max DEX build | 13 | 23 |

### Evasion (passive required) = DEX / 2

Flat physical damage reduction. Only with Evasion passive (Ranger, Rogue).

---

## Damage Formulas

### Physical Damage = Weapon Base + Stat
- STR weapons (swords, axes, maces, 2H): Base + STR
- DEX weapons (daggers, bows): Base + DEX
- INT weapons (Energy Sword only): Base + INT
- Without matching passive: Base only

### Skill Damage = Skill Base + (Stat x Multiplier)

**Fireball example:**
| Level | Base | Mult | INT 6 | INT 10 | INT 13 |
|-------|------|------|-------|--------|--------|
| Lv1 | 4 | x1 | 10 | 14 | 17 |
| Lv2 | 6 | x1.5 | 15 | 21 | 25 |
| Lv3 | 8 | x2 | 20 | 28 | 34 |

**Double Attack (each hit):**
| Level | Base | Mult | STR 6 | STR 10 | STR 13 |
|-------|------|------|-------|--------|--------|
| Lv1 | 1 | x0.5 | 4 | 6 | 7 |
| Lv2 | 2 | x0.6 | 5 | 8 | 9 |
| Lv3 | 2 | x0.7 | 6 | 9 | 11 |

### Heal Amount = Heal Base + INT

---

## Defense Formulas

### Armor (LoL diminishing returns)
**Reduction = Armor / (Armor + 10)**

| Armor | Reduction |
|-------|-----------|
| 0 | 0% |
| 2 | 17% |
| 5 | 33% |
| 10 | 50% |
| 15 | 60% |
| 20 | 67% |
| 30 | 75% |

### Magic Resistance
Same formula. MR is rare.

### Shield (flat, before armor)
Example: 20 raw -> Shield blocks 3 -> 17 to armor -> 10 armor = 50% -> 8.5 final

---

## Mana Costs

| Tier | Cost | Examples |
|------|------|---------|
| Free (Lv3) | 0 | Magic Missile, Double Attack, Double Shot |
| Cheap | 1-2 | Buffs, debuffs, basic heals |
| Medium | 3-4 | Fireball, Smite, AOE heal, Stealth |
| Expensive | 5-6 | Arcane Burst, Revive, Arrow Storm |
| Very Expensive | 7-8 | Ultimate-tier skills |

### Passive Toggle Mana Lock

| Tier | Mana Locked |
|------|-------------|
| Utility (1H, Bow) | 2 |
| Defensive (Shield, Evasion, Constitution) | 3 |
| Power (Arcane Focus, Execute, Backstab) | 4 |
| Special (Animal Companion, Rally Aura) | 3 |
| Resource (Mana Well) | 0 (adds mana) |

---

## Equipment Value Ranges

### Weapons
| Act | Base | Example |
|-----|------|---------|
| 1 (Lv1-5) | 1-3 | Rusty Sword(1), Short Sword(2), Fine Blade(3) |
| 2 (Lv6-10) | 4-6 | War Sword(4), Enchanted(5), Rare(6) |
| 3 (Lv11-15) | 7-9 | Shadow Steel(7), Blessed(8), Boss Drop(9) |
| 4 (Lv16-20) | 10-12 | Legendary(10), Artifact(11), Final(12) |

### Armor
| Act | Light | Medium | Heavy |
|-----|-------|--------|-------|
| 1 | 1 | 2 | 3-4 |
| 2 | 4 | 5-6 | 7-8 |
| 3 | 8 | 10 | 12-14 |
| 4 | 14 | 16 | 18-20 |

---

## Enemy Power Bands

### Act 1 (Player Lv 1-5)
| Sub-Act | HP | DMG | Armor | Mini-Boss HP |
|---------|----|-----|-------|-------------|
| 1.1 | 6-10 | 2-3 | 0 | 25 |
| 1.2 | 10-16 | 3-5 | 1-2 | 40 |
| 1.3 | 16-22 | 5-7 | 2-3 | 55 |
| Boss | — | — | — | 80 |

### Act 2 (Player Lv 6-10)
| Sub-Act | HP | DMG | Armor | Mini-Boss HP |
|---------|----|-----|-------|-------------|
| 2.1 | 25-35 | 8-10 | 3-4 | 100 |
| 2.2 | 35-50 | 10-14 | 4-6 | 140 |
| 2.3 | 50-65 | 14-18 | 6-8 | 180 |
| Boss | — | — | — | 250 |

### Act 3 (Player Lv 11-15)
| Sub-Act | HP | DMG | Armor | Mini-Boss HP |
|---------|----|-----|-------|-------------|
| 3.1 | 65-85 | 18-22 | 8-10 | 280 |
| 3.2 | 85-110 | 22-28 | 10-12 | 350 |
| 3.3 | 110-140 | 28-34 | 12-14 | 420 |
| Boss | — | — | — | 550 |

### Act 4 (Player Lv 16-20)
| Sub-Act | HP | DMG | Armor | Mini-Boss HP |
|---------|----|-----|-------|-------------|
| 4.1 | 140-180 | 34-40 | 14-16 | 500 |
| 4.2 | 180-220 | 40-48 | 16-18 | 600 |
| 4.3 | 220-260 | 48-55 | 18-20 | 700 |
| FINAL | — | — | — | 1000+ |

---

## Formula Summary

| What | Formula |
|------|---------|
| HP | 6 + (STR x Level) |
| Mana | INT + Level |
| ATB Speed | 10 + DEX |
| Physical Damage | Weapon Base + STR or DEX |
| Spell Damage | Skill Base + (INT x Multiplier) |
| Heal Amount | Heal Base + INT |
| Armor Reduction | Armor / (Armor + 10) |
| Magic Resistance | MR / (MR + 10) |
| Shield | Flat subtraction before armor |
| Evasion | DEX / 2 flat (passive req.) |
