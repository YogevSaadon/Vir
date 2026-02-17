# Vassnian — 11 Class & Skill Additions

## New Concepts from Notes

---

## 1. Starting Class Passives

Every class gets a **starting passive** chosen during character creation. This passive:
- Grants **class-exclusive world skills** (only this class gets them)
- Grants **class-exclusive combat skills** (only this class gets them)
- May involve a **sub-choice** (e.g., Ranger picks terrain, Priest picks deity)

Everyone starts with the **same stat point-buy** (base 1 in all stats + 2 free points). The passive is what differentiates classes beyond their base stat spread.

### Starting Passive by Class:

| Class | Passive Name | Sub-Choice | Combat Bonus | World Bonus |
|-------|-------------|------------|--------------|-------------|
| **Knight** | TBD | — | TBD | TBD |
| **Barbarian** | TBD | — | TBD | TBD |
| **Ranger** | Terrain Mastery | Choose terrain type | Damage bonus in chosen terrain | Special story options in that terrain |
| **Mage** | TBD | — | TBD | TBD |
| **Priest** | Divine Favor | Choose deity | Deity-specific skills | Deity-related story options |
| **Paladin** | TBD | — | Rally (Charisma-based) | TBD |
| **Rogue** | Street Smarts | — | TBD | City options: theft, lockpicking, anti-traps |
| **Necromancer** | TBD | — | TBD | TBD |
| **Bard** | TBD | — | Shout (Charisma damage) | TBD |
| **Warrior*** | Weapon Mastery | Choose weapon type | Bonus with chosen weapon | TBD |

*Note: "Warrior" and "Knight/Barbarian" overlap — clarify if Warrior = Knight or a separate class.*

### Implementation:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPassive {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sub_choice: Option<PassiveSubChoice>,
    pub grants_world_skills: Vec<String>,    // class-exclusive world skills
    pub grants_combat_skills: Vec<String>,   // class-exclusive combat skills
    pub combat_effects: Vec<PassiveEffect>,
    pub world_effects: Vec<PassiveEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PassiveSubChoice {
    ChooseTerrain { options: Vec<Terrain> },
    ChooseDeity { options: Vec<String> },
    ChooseWeapon { options: Vec<String> },
}
```

**MVP:** Add `starting_passive` field to class definition (optional, null for MVP Knight).

---

## 2. New Stat: Charisma (?)

Multiple notes reference **Charisma** as a stat affecting Bard and Paladin. This needs a decision:

**Option A:** Add Charisma as an **8th primary stat** (8 stats total)
**Option B:** Charisma is a **derived stat** calculated from existing stats (e.g., Faith + Luck)
**Option C:** Charisma is a **class-specific resource** only Bard/Paladin use

> ⚠️ **DECISION NEEDED from developer:** Is Charisma a new primary stat, a derived stat, or something else?

For now, scaffold it as a potential 8th stat that can be enabled/disabled.

---

## 3. New Class: Bard

The Bard appears to be a **9th class** not in the original 8. Key features:

| Feature | Details |
|---------|---------|
| Passive | Increases **EXP gained** for the whole group (scales with Charisma) |
| Attack | Only has **one attack skill** plus weapon auto-attack |
| Special Skill | **Shout** — deals damage based on Charisma. Not amazing but decent. |
| Role | Support / buffer with limited offensive capability |

> ⚠️ **DECISION NEEDED:** Is Bard class #9, or does it replace one of the existing 8?

---

## 4. New Skills

### Quickstrike
| Field | Value |
|-------|-------|
| Name | Quickstrike |
| Speed | **Instant** |
| Description | Attacks with equipped weapon immediately |
| Cooldown | Long (6+ turns) — it's Instant so needs long CD |
| Notes | Uses weapon damage, not a spell. Available to multiple classes? |

### Bard: Shout
| Field | Value |
|-------|-------|
| Name | Shout |
| Speed | Normal |
| Scaling Stat | Charisma |
| Description | Deals moderate damage based on Charisma |
| Notes | Bard's only real attack besides weapon. Decent, not OP. |

### Paladin: Rally
| Field | Value |
|-------|-------|
| Name | Rally |
| Speed | Normal |
| Scaling Stat | Charisma |
| Description | Buffs party (details TBD) |
| Notes | Charisma-based party buff |

### Bard: EXP Boost (Passive)
| Field | Value |
|-------|-------|
| Name | Inspiring Presence (or similar) |
| Type | Passive |
| Effect | Increases EXP gained for whole party |
| Scaling | Charisma |

### Rogue: Gold Find (Passive)
| Field | Value |
|-------|-------|
| Name | Keen Eye (or similar) |
| Type | Passive |
| Effect | Increases gold found from missions/combat |
| Scaling | Luck |

---

## 5. Skill Exclusivity Clarification

From the notes, the main character CAN find more skills (via skill books), but:
- **Class-exclusive skills** exist that ONLY that class can use (from starting passive)
- **Universal skills** can be learned by anyone with the right stats (via skill books)

The starting passive grants a set of exclusive skills that define the class identity. Skill books found later can teach cross-class universal skills.

---

## 6. Updated Character Creation Flow (with Passive)

```
1. Avatar Selection
2. Class Selection
3. Stat Point-Buy (same for all classes: base 1, +2 free)
4. Class Passive Sub-Choice (if applicable):
   - Ranger → pick terrain
   - Priest → pick deity
   - Warrior → pick weapon type
   - Others → no sub-choice (passive is automatic)
5. World Skill Selection (pick 1 from 3)
6. Confirm → Enter World
```

**MVP:** Steps 1-3 + 5-6 only. Step 4 scaffolded but skipped for Knight.
