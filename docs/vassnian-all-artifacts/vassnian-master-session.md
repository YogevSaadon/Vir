# Vassnian / Master Session Document
## All Decisions - Systems Design Session

---

# PART 1: CORE SYSTEMS (LOCKED)

## Stat System (3 Stats, Range 1-20)

| Stat | Primary Use | Universal Use |
|------|------------|---------------|
| STR | Heavy melee damage | Armor capacity, HP pool |
| DEX | Light melee, ranged damage | Evasion, ATB speed |
| INT | Spell damage (arcane + divine) | Cooldown reduction, mana pool |

**Starting Stats:** 3 in each + 3 free points to allocate = 12 total at level 1.
**Stat points:** 1 per odd level (1, 3, 5, 7, 9, 11, 13, 15, 17, 19) = 10 total.
**Player assigns freely** to any of the 3 stats.
**No dump stats.** Every class wants all 3 in different priority orders.
**No crits.** Damage is predictable.

## Level System

**Max level:** 20
**Skill points:** 1 per even level (2, 4, 6, 8, 10, 12, 14, 16, 18, 20) = 10 total.
**At max level:** 10 stat points + 10 skill points total.

## Numbers Philosophy

**Low numbers.** HP at level 1: ~6. HP at level 20: ~200. No bloat.

---

# PART 2: COMBAT SYSTEM

## Formation

2 front row + 2 back row (4-person party).
- Front row: can melee attack.
- Back row: can only attack with ranged.
- Special skills/passives can break this rule (Rogue Backstab, Mage Energy Sword + cross-class combo, Ranger light weapons from back).

## Mana

- Scales with INT.
- **DOES NOT RECHARGE IN COMBAT.** Every spell is a permanent resource drain for that fight.
- Full mana restore after combat.
- Passive toggles lock max mana while active.

## HP and Death

- Full HP restore after combat (win).
- 0 HP = downed. Can be revived by Priest's Revive skill mid-fight.
- All party downed = you lose the fight.
- **No injury system.** The only way to lose is a full party wipe.

## Armor System (League of Legends style)

- Diminishing returns on armor.
- 10 armor = 50% physical damage reduction.
- More armor = less % per additional point.

## Magic Resistance

- Works same as armor but for spell damage.
- Magic resistance is RARE (from rings, special gear, Bard aura).

## Shields

- Shields do NOT add armor (stacking would be too strong).
- Shields give a separate form of damage reduction.
- Makes shields uniquely valuable for tanky characters.

## Weapons

- **Anyone can equip any weapon.** Mage can use a bow. Priest can use a sword.
- Weapon passive skills boost damage with that weapon type.
- Without the passive, you can equip it but it's weak.

## Ambush Mechanic

- Stealth world skill lets your party ambush enemies.
- Ambush = your party starts with ATB advantage in combat.
- Perception world skill can detect enemy ambushes (reversing their advantage).

---

# PART 3: COMBAT SKILLS

## Skill Structure Per Class

- **6 active skills** (pick 4 for skill bar before combat).
- **4 passive skills** (toggles, cost max mana, set before combat).
- **All skills have 3 levels** (Lv1, Lv2, Lv3).
- **Increasing stat requirements** per level (e.g., Lv1 needs 5 STR, Lv2 needs 8 STR, Lv3 needs 12 STR).
- **Some skills require certain base stats** to even unlock.
- Skill points shared pool for actives AND passives.
- 10 skills x 3 levels = 30 possible levels, but only 10 skill points. Real choices.

## Free Basic Attacks at Lv3

- Magic Missile → Mage, Bard
- Double Attack → Warrior, Knight, Ranger, Paladin
- Double Shot → Ranger

## Cross-Class Learning (Godsent Only)

- Player finds skill books and Godsent shards in the world.
- Learn individual skills from other classes.
- Companions CANNOT learn cross-class.

---

# PART 4: CLASS SKILLS STATUS

## MAGE — COMPLETE

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Magic Missile (low dmg, free at Lv3) | 1 | Energy Sword (light INT melee, weightless) |
| 2 | Fireball (cast time, AOE fire) | 2 | Mage Shield (armor based on INT) |
| 3 | Cone of Cold (cast time, slow; Lv3 = AOE slow) | 3 | Arcane Focus (spell damage %) |
| 4 | Blink (instant, attack anyone on board) | 4 | Mana Well (bonus max mana) |
| 5 | Counterspell (instant, cancel channel; Lv2 = less CD/mana; Lv3 = pre-cast counter) | | |
| 6 | Arcane Burst (single target nuke, high mana) | | |

## WARRIOR — needs 1 active

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Cleave (hit multiple front row) | 1 | 1-Handed Weapons |
| 2 | Double Attack (free Lv3) | 2 | 2-Handed Weapons |
| 3 | Damage Buff (instant, boosts next turns) | 3 | Constitution (max HP) |
| 4 | Slash Through (hits front + back row behind) | 4 | Execute (kills low HP enemies) |
| 5 | Smash (single nuke, resets on kill) | | |
| 6 | ??? | | |

## KNIGHT — needs 1 active

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Taunt (force enemies to attack you) | 1 | 1-Handed Weapons |
| 2 | Stun Attack (stuns enemy) | 2 | Shield (Knight exclusive) |
| 3 | Double Attack (free Lv3) | 3 | Constitution (max HP) |
| 4 | Second Wind (self heal) | 4 | Retaliate (counter-attack when hit) |
| 5 | Shield Bash (push front row enemy to back) | | |
| 6 | ??? | | |

## PALADIN — needs 2 actives

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Smite (big INT-scaling dmg, high mana) | 1 | 1-Handed Weapons |
| 2 | Heal (shared with Priest) | 2 | 2-Handed Weapons |
| 3 | Blink (shared with Mage) | 3 | Mana Well |
| 4 | Double Attack (free Lv3) | 4 | Rally Aura (Lv1: team HP, Lv2: team dmg, Lv3: both) |
| 5 | ??? | | |
| 6 | ??? | | |

## RANGER — COMPLETE

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Double Shot (free Lv3) | 1 | Animal Companion (exclusive, unlocks mission) |
| 2 | Mark Enemy (bonus damage on target) | 2 | 1-Handed Weapons |
| 3 | Arrow Storm (AOE ranged) | 3 | Bow |
| 4 | Double Attack (free Lv3) | 4 | Evasion |
| 5 | Revive Companion | | |
| 6 | Melee Skill (TBD name) | | |

### Ranger Animal Companions

| Animal | Position | Role |
|--------|----------|------|
| Bear | Front | Tank — absorbs damage for Ranger |
| Bird | N/A (flies) | Support — marks enemies, improves ranged damage |
| Tiger | Back | DPS — attacks alone from back row |
| Wolf | Front | DPS — attacks multiple times, can't defend |

Animal companion is EXTRA unit (doesn't take party slot). Ranger NPCs start with 1 point automatically.

## ROGUE — needs 1 active

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Stealth (untargetable until you attack) | 1 | 1-Handed Weapons |
| 2 | Poison Attack | 2 | Bow |
| 3 | Armor Ignore (active, punch through armor) | 3 | Evasion |
| 4 | Smoke Bomb (stun: Lv1 = 1 target, Lv2 = 2, Lv3 = whole row) | 4 | Backstab (Lv1: attack from back row, Lv2+: hit enemy back row) |
| 5 | Hook (pull back row enemy to front, swap) | | |
| 6 | ??? | | |

## PRIEST — needs 1 active + 2 passives

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Single Target Heal | 1 | Arcane Focus (shared with Mage) |
| 2 | AOE Heal | 2 | Mana Well (shared) |
| 3 | Attack Spell (single target) | 3 | ??? |
| 4 | Revive (long cooldown) | 4 | ??? |
| 5 | Cure Conditions (remove poison/curse/debuffs) | | |
| 6 | ??? | | |

Priest can equip 1-Handed weapons WITHOUT a passive (no damage bonus, just access).

## BARD — needs 2 actives + 2 passives

| # | Actives | # | Passives |
|---|---------|---|----------|
| 1 | Magic Missile (free Lv3, shared with Mage) | 1 | Mana Well (shared) |
| 2 | Damage Debuff (reduce enemy damage) | 2 | Aura of Magic Resistance (party magic resist) |
| 3 | Damage Buff (boost ally damage) | 3 | ??? |
| 4 | Counterspell (shared with Mage) | 4 | ??? |
| 5 | ??? | | |
| 6 | ??? | | |

## Shared Skill Map

| Skill | Type | Classes |
|-------|------|---------|
| 1-Handed Weapons | Passive | Knight, Warrior, Paladin, Ranger, Rogue |
| 2-Handed Weapons | Passive | Warrior, Paladin |
| Shield | Passive | Knight (exclusive) |
| Bow | Passive | Ranger, Rogue |
| Evasion | Passive | Ranger, Rogue |
| Constitution | Passive | Knight, Warrior |
| Mana Well | Passive | Mage, Paladin, Bard, Priest |
| Arcane Focus | Passive | Mage, Priest |
| Rally Aura | Passive | Paladin (exclusive) |
| Animal Companion | Passive | Ranger (exclusive) |
| Armor Ignore | Active | Rogue (exclusive) |
| Backstab | Passive | Rogue (exclusive) |
| Execute | Passive | Warrior (exclusive) |
| Retaliate | Passive | Knight (exclusive) |
| Magic Missile | Active | Mage, Bard |
| Double Attack | Active | Warrior, Knight, Ranger, Paladin |
| Double Shot | Active | Ranger |
| Heal | Active | Priest, Paladin |
| Blink | Active | Mage, Paladin |
| Counterspell | Active | Mage, Bard |

## Remaining Gaps

| Class | Missing Actives | Missing Passives | Total |
|-------|----------------|-----------------|-------|
| Mage | 0 | 0 | **DONE** |
| Warrior | 1 | 0 | 1 |
| Knight | 1 | 0 | 1 |
| Paladin | 2 | 0 | 2 |
| Ranger | 0 | 0 | **DONE** |
| Rogue | 1 | 0 | 1 |
| Priest | 1 | 2 | 3 |
| Bard | 2 | 2 | 4 |
| **TOTAL** | **8** | **4** | **12** |

---

# PART 5: WORLD SKILLS (13 locked, target 18)

1 skill per character. Party of 4 = 4 of 13 covered.
Every skill = you GET something in the story. Binary (have it or don't).
Godsent can find more skills through the game.

## Locked Skills (13)

| # | Skill | What It Gives You |
|---|-------|------------------|
| 1 | Acrobatics | Climb, jump, leap gaps, reach high places, physical movement |
| 2 | Lockpicking | Open locked chests, doors, cells |
| 3 | Swimming | Cross rivers, underwater caves, reach islands |
| 4 | Arcane Knowledge | Bypass magic barriers, identify enchantments, read runes |
| 5 | History | Find vaults in ruins, identify artifacts, catch lies about the past |
| 6 | Religion | Shrine buffs, temple access, recognize cults |
| 7 | Nature | Wilderness shortcuts, identify useful plants, read weather |
| 8 | Perception | PASSIVE. See ambushes (combat ATB advantage), notice enemies, spot oddities |
| 9 | Search | ACTIVE. Find hidden chests, secret compartments, concealed doors |
| 10 | Tracking | Follow trails to lairs (loot), read footprints, reverse ambushes |
| 11 | Stealth | Ambush enemies (ATB advantage), sneak past encounters, eavesdrop |
| 12 | Persuasion | Better rewards, NPC help, better prices, talk past guards |
| 13 | Disguise | Pass as someone else, enter restricted areas |
| 14-18 | TBD | To be added when writing stories reveals gaps |

## Story Writer Format

> [Skill: Acrobatics] There's a rocky ledge above the path. At the top, you find an old chest. Inside: [ITEM].

Without the skill, this text doesn't appear. Player doesn't know they missed it.

---

# PART 6: COMPANIONS

## Distribution

| | Count |
|---|------|
| Total companions | 13 |
| Act 0 (King's Guild) | 5 |
| Act 1 (surface world) | 6 |
| Act 2 (Shadow World) | 2 |
| Act 3 | 0 |

## Per Class

| Class | Companions |
|-------|-----------|
| Knight | 2 |
| Warrior | 2 |
| Paladin | 2 |
| Mage | 2 |
| Priest | 2 |
| Bard | 1 |
| Ranger | 1 |
| Rogue | 1 |

Each companion has 1 world skill.

---

# PART 7: 8 CLASSES OVERVIEW

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

# WHAT'S STILL NEEDED

1. Fill 12 remaining combat skill gaps (8 actives + 4 passives)
2. Find 5 more world skills (target 18, have 13)
3. Assign world skills to specific companions
4. Design companion characters (names, personalities, recruitment quests)
5. Stat ranges and scaling formulas
6. Mana base amount and INT scaling
7. Passive mana lock amounts
8. Skill stat requirement numbers per class
9. Economy system (gold, shops, prices)
10. Time system
11. Bestiary
12. Bosses
13. Items and equipment
14. Tag system for story consequences
