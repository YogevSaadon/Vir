# Vassnian Game Systems Reference

All core systems decisions. Source: vassnian-master-session.md

---

## Stat System (3 Stats, Range 1-20)

| Stat | Primary Use | Universal Use |
|------|------------|---------------|
| STR | Heavy melee damage | Armor capacity, HP pool |
| DEX | Light melee, ranged damage | Evasion, ATB speed |
| INT | Spell damage (arcane + divine) | Cooldown reduction, mana pool |

- **Starting:** 3 each + 3 free = 12 total at level 1
- **Stat points:** 1 per odd level (1,3,5,7,9,11,13,15,17,19) = 10 total
- **No dump stats.** Every class wants all 3.
- **No crits.** Damage is predictable.

## Level System

- **Max level:** 20
- **Skill points:** 1 per even level (2,4,6,8,10,12,14,16,18,20) = 10 total
- **At max:** 10 stat points + 10 skill points total

## Numbers Philosophy
HP Lv1: ~6. HP Lv20: ~200. Low numbers, no bloat.

---

## 8 Classes

| Class | Role | Row | Primary | Secondary |
|-------|------|-----|---------|-----------|
| Knight | Tank/Defense/Shield | Front | STR | DEX |
| Warrior | Melee DPS | Front | STR | DEX |
| Ranger | Ranged/Animal Companion | Back | DEX | INT |
| Rogue | Stealth/Burst/Flexible | Front or Back | DEX | INT |
| Mage | Magic DPS/Control | Back | INT | DEX |
| Priest | Healing/Support | Back | INT | STR |
| Paladin | Spell Warrior | Front | STR+INT | DEX |
| Bard | Support/Buff-Debuff | Back | INT | DEX |

---

## Combat System

### Formation
2 front + 2 back (4-person party)
- Front row: can melee attack
- Back row: can only attack with ranged
- Special skills can break this (Rogue Backstab, Mage Energy Sword, etc.)

### Mana
- Scales with INT
- **DOES NOT RECHARGE IN COMBAT** — every spell is permanent resource drain
- Full mana restore after combat
- Passive toggles lock max mana while active

### HP and Death
- Full HP restore after combat (win)
- 0 HP = downed, can be revived by Priest mid-fight
- All party downed = lose fight
- **No injury system** — only lose by full party wipe

### Armor (LoL-style diminishing returns)
**Reduction = Armor / (Armor + 10)**

| Armor | Reduction |
|-------|-----------|
| 0 | 0% |
| 2 | 17% |
| 5 | 33% |
| 10 | 50% |
| 15 | 60% |
| 20 | 67% |

### Magic Resistance
Same formula as armor. MR is RARE (rings, Bard aura, special gear).

### Shields (Knight only)
Flat reduction BEFORE armor. Do NOT add armor (avoid stacking).

| Shield | Flat Reduction |
|--------|---------------|
| Basic | 1 |
| Iron | 2 |
| Tower | 3 |
| Endgame | 5 |

### Weapons
Anyone can equip any weapon. Weapon passives boost damage with that type. Without passive = no stat bonus.

### Ambush
- Stealth world skill = party ambushes enemies (ATB advantage)
- Perception world skill = detect enemy ambushes

---

## Combat Skills

### Structure Per Class
- **6 active skills** (pick 4 for skill bar before combat)
- **4 passive skills** (toggles, cost max mana, set before combat)
- **All skills have 3 levels**
- **Increasing stat requirements** per level
- 10 skills x 3 levels = 30 possible, but only 10 skill points

### Free Basic Attacks at Lv3
- Magic Missile: Mage, Bard
- Double Attack: Warrior, Knight, Ranger, Paladin
- Double Shot: Ranger

### Cross-Class Learning (Godsent Only)
- Find skill books and Godsent shards in world
- Companions CANNOT learn cross-class

---

## Class Skills (Complete + Gaps)

### Mage — COMPLETE
| Actives | Passives |
|---------|----------|
| Magic Missile (free Lv3) | Energy Sword (light INT melee) |
| Fireball (cast time, AOE) | Mage Shield (armor from INT) |
| Cone of Cold (slow; Lv3=AOE) | Arcane Focus (spell dmg %) |
| Blink (instant, attack anyone) | Mana Well (bonus max mana) |
| Counterspell (cancel channel) | |
| Arcane Burst (single nuke) | |

### Warrior — needs 1 active
| Actives | Passives |
|---------|----------|
| Cleave (hit multiple front) | 1-Handed Weapons |
| Double Attack (free Lv3) | 2-Handed Weapons |
| Damage Buff (instant) | Constitution (max HP) |
| Slash Through (front+back) | Execute (kills low HP) |
| Smash (nuke, resets on kill) | |
| ??? | |

### Knight — needs 1 active
| Actives | Passives |
|---------|----------|
| Taunt (force attacks on you) | 1-Handed Weapons |
| Stun Attack | Shield (Knight exclusive) |
| Double Attack (free Lv3) | Constitution (max HP) |
| Second Wind (self heal) | Retaliate (counter on hit) |
| Shield Bash (push to back) | |
| ??? | |

### Paladin — needs 2 actives
| Actives | Passives |
|---------|----------|
| Smite (INT-scaling, high mana) | 1-Handed Weapons |
| Heal (shared with Priest) | 2-Handed Weapons |
| Blink (shared with Mage) | Mana Well |
| Double Attack (free Lv3) | Rally Aura (HP/dmg/both) |
| ??? | |
| ??? | |

### Ranger — COMPLETE
| Actives | Passives |
|---------|----------|
| Double Shot (free Lv3) | Animal Companion (exclusive) |
| Mark Enemy (bonus dmg) | 1-Handed Weapons |
| Arrow Storm (AOE ranged) | Bow |
| Double Attack (free Lv3) | Evasion |
| Revive Companion | |
| Melee Skill (TBD) | |

#### Animal Companions
| Animal | Position | Role |
|--------|----------|------|
| Bear | Front | Tank — absorbs damage |
| Bird | N/A (flies) | Support — marks, ranged boost |
| Tiger | Back | DPS — attacks alone |
| Wolf | Front | DPS — multi-hit, can't defend |

### Rogue — needs 1 active
| Actives | Passives |
|---------|----------|
| Stealth (untargetable) | 1-Handed Weapons |
| Poison Attack | Bow |
| Armor Ignore (active) | Evasion |
| Smoke Bomb (stun) | Backstab (back row attack) |
| Hook (pull enemy) | |
| ??? | |

### Priest — needs 1 active + 2 passives
| Actives | Passives |
|---------|----------|
| Single Target Heal | Arcane Focus |
| AOE Heal | Mana Well |
| Attack Spell (single) | ??? |
| Revive (long CD) | ??? |
| Cure Conditions | |
| ??? | |

### Bard — needs 2 actives + 2 passives
| Actives | Passives |
|---------|----------|
| Magic Missile (free Lv3) | Mana Well |
| Damage Debuff | Aura of Magic Resistance |
| Damage Buff | ??? |
| Counterspell | ??? |
| ??? | |
| ??? | |

### Remaining Gaps: 8 actives + 4 passives = 12 total

---

## World Skills (13 locked, target 18)

1 skill per character. Party of 4 = 4 of 13 covered. Binary (have or don't).

| # | Skill | What It Gives |
|---|-------|---------------|
| 1 | Acrobatics | Climb, jump, leap gaps, reach high places |
| 2 | Lockpicking | Open locked chests, doors, cells |
| 3 | Swimming | Cross rivers, underwater caves, reach islands |
| 4 | Arcane Knowledge | Bypass magic barriers, identify enchantments |
| 5 | History | Find vaults in ruins, identify artifacts, catch lies |
| 6 | Religion | Shrine buffs, temple access, recognize cults |
| 7 | Nature | Wilderness shortcuts, identify plants, read weather |
| 8 | Perception | PASSIVE. See ambushes, spot oddities |
| 9 | Search | ACTIVE. Find hidden chests, secret doors |
| 10 | Tracking | Follow trails to lairs, read footprints |
| 11 | Stealth | Ambush enemies, sneak past, eavesdrop |
| 12 | Persuasion | Better rewards, NPC help, better prices |
| 13 | Disguise | Pass as someone else, enter restricted areas |
| 14-18 | TBD | Added when stories reveal gaps |

### Story Writer Format
> [Skill: Acrobatics] There's a rocky ledge above. At the top, an old chest. Inside: [ITEM].
Without the skill, text doesn't appear. Player doesn't know they missed it.

---

## Companions Distribution

| Act | Count | Source |
|-----|-------|--------|
| 0 (King's Guild) | 5 | Knight, Warrior, Paladin, Mage, Priest |
| 1 (Surface) | 6 | Warrior#2, Knight#2, Priest#2, Bard, Ranger, Rogue |
| 2 (Shadow World) | 2 | Mage#2, Paladin#2 |
| 3 | 0 | — |

Each companion has 1 world skill.
