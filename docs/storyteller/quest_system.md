# Vassnian Quest System

5 Quest Types, 10 Tag Types, Dynamic Array, Priority-Based Placement.

---

## Quest Types (by priority)

| Priority | Type | Chainable | Conditional | Description |
|----------|------|-----------|-------------|-------------|
| P1 | **Main Quest** | No | No | Core story. Locked positions. Boss selection determines which set. |
| P2 | **Recruitment** | No | Yes | Only when party has empty slots. Disappears when full. |
| P3 | **Companion** | Yes | Yes | Personal chains for recruited companions. Displaces one-shots. |
| P4 | **Continuation** | Yes | No | Multi-part arcs not tied to companions. Part 1 as filler, success queues next. |
| P5 | **One-Shot** | No | No | Standalone. Fill remaining slots. First to be displaced. |

---

## Tag System (10 types)

| Tag | Description | Example |
|-----|-------------|---------|
| companion_present | Check if specific companion in party | Orc reacts to orc NPCs |
| companion_missing | Check empty party slots | Recruitment quests available |
| world_skill | Check party world skills | Lockpicking opens secret door |
| has_item | Check inventory for item | Dark artifact triggers event |
| choice_flag | Track previous quest outcomes | Saved village = villagers help later |
| banter | Pull companion pair dialogue | Cat knight + wolf paladin rivalry |
| personality_react | Companion trait reaction | Dwarf priest orders drinks at bar |
| chain_outcome | Check previous chain result | Shadow party chain success = part 2 |
| stat_check | Party stats vs threshold | Avg level >= 3 = elite path |
| boss_theme | Tie main quests to boss selection | Goblin king = goblin-themed quests |

---

## Quest Template Structure

```
id: unique_id
type: main | recruitment | companion | continuation | oneshot
name: display name
act: 1-4
sub_act_range: [1.1, 1.3]
chain_id: null | chain_group_id
chain_part: null | 1, 2, 3...
rank_range: [min, max] enemy ranks
boss_theme: null | boss_id
companion_id: null | companion_id
tags: [array of tag objects]
rewards: { xp, gold, items, companion_unlock, skill_unlock, story_flag }
on_success: { place_chain_part, set_flag }
on_failure: { remove_chain, set_flag }
description: story summary
```

---

## Array Fill Algorithm (6 steps)

1. **Lock Main Quests** — Fixed positions, boss determines which set. NEVER move.
2. **Check Companion Slots** — Empty slots = place recruitment quests.
3. **Place Active Chain Parts** — Continuation/companion chains in progress.
4. **Introduce New Chain Part-1s** — Fresh chain hooks.
5. **Fill with One-Shots** — Remaining slots.
6. **React to Events** — Mid sub-act: recruit = insert chain (kick one-shots), fail chain = remove future parts.

### Displacement Rules
- One-shots displaced first (P5)
- Then unused chain part-1s (P4)
- Active chains and recruitment protected
- Main quests NEVER move
- Chain failure: all remaining parts removed, slots freed
- Chain success: next part queued for next sub-act
