# Vassnian — Raw Notes (Full English Translation)

This file is the COMPLETE translation of both note files. Every single note is here.
After organizing into proper docs, this file can be archived.

---

## SOURCE: Computer Notes (פתקי מחשב)

### Meta / Organization Instructions
1. Organize phone notes into proper documents. Create section 8.5 (MVP supplements) and section 7.5 (POC supplements).
2. Create a prompt explaining how to use these docs after copying them to the docs folder.
3. Before splitting into sections, the agent should EXTRACT everything from this file into a separate doc, clean it up, and ONLY THEN split and categorize.
4. The goal of the POC/MVP is to make everything WORK — no variety of types, no images, no balancing. Just a system that's easy to add to later. All UI looks correct and the architecture matches the vision.
5. Ask the agent to split new info into notes — section 9, or section 4.1 if continuing existing sections.

### Engine / Architecture Questions
6. Question for agent: Are the derived combat stats already built? Meaning: if I want Intelligence to affect a stat called "Spell Power" that strengthens all skills (by adding it to their damage function), is that simple? Also: is there a base class for skills so it's easy to add new ones? Maybe a skill tree too — for caster classes (mana cost) and non-caster classes.

### Skill / Class Additions
7. **Quickstrike** skill — attacks with weapon as an Instant skill.
8. **Bard** has a passive that increases EXP gained for the whole group (Charisma). **Rogue** has increased gold find (Luck).
9. **Paladin** will have a Rally skill based on Charisma. **Bard** will have a special Shout skill that deals damage based on Charisma — not amazing but decent, since he only has one attack plus weapon.
10. Maybe every class has a **starting passive**. Everyone starts with the same stat choices. The main character CAN find more skills, but each class also has a starting passive:
    - **Ranger** chooses a terrain type at character creation → gets damage bonuses + special story choices in that terrain
    - **Priest** chooses a deity
    - **Warrior** chooses a weapon type
    - **Rogue** gets city options for theft, lockpicking, anti-traps, etc.
    - Passive brings certain world skills exclusive to that class AND certain combat skills exclusive to that class

### Mission / Choice System
11. Choices in the game are important. Every time you go on a mission, you choose from 3 options with different difficulty levels. Before departure, each mission has a description for each short story, and you choose. You understand difficulty level by the time cost.

### Game Phase Structure
12. The game is divided into four phases (plus sub-phases):

**Phase 0 — Arrival:**
- God summons you and tells you you're crossing worlds
- He lets you choose your character (avatar, class, stats, skills)
- You appear in the new world above a summoning circle
- You look around and see many other Godsents
- The King explains the world, why he summoned heroes, and what he offers
- He gives you gold and lets you choose from 3 companions
- You leave in pairs to the first city and the world
- Your companion and you sit in a bar, they tell you their story and what they think you should do
- Enter shops, then go on missions
- Can always return to city (costs time)
- Every mission costs time; after certain time passes → next phase
- Sometimes meet other Godsent groups on missions — the gods summoned many heroes, each gets some equipment and a hero from the kingdom to help

**Phase 1 — Growth:**
- Level up, earn gold and items, recruit party members
- Get stronger, do missions at varying difficulties

**Phase 1.5 — Shadow World Entry Story:**
- Story event: the King sends you and another party (the two best parties) to investigate something — happens right after first boss
- You go with the other party, and they BETRAY you
- They switched sides, want to eliminate competition, and trap you in the Shadow World
- After you escape, they become the boss of Phase 3

**Phase 2 — The Shadow World (Underground):**
- Very dangerous, lots of opportunities
- Can't return to city, but occasionally reach underground cities
- Instead of "Tower" call it "Shadow World" / "Underground World"
- Known for many dangers and opportunities, dark mages go there, bad things come out occasionally

**Phase 2.5 — Emergence:**
- Exit the Shadow World, return to main city
- Talk to people who thought you were dead

**Phase 3 — Revenge & Preparation:**
- Against all odds, after certain time, you exit
- Can return to main city, build things at the blacksmith with rare/cool items found inside
- Find the traitor party (they're the boss of Phase 3)
- OR journey to the final boss location
- Get stronger, help people, see the impact of the invasion on the kingdom

**Phase 4 — Final Confrontation:**
- Mini-stories in the battle against the final boss
- Could be: climbing the Necromancer's tower, entering the Demon Realm to defeat the Demon Lord, ascending the Dragon's fortress, etc.
- Final boss not decided yet

**Phase 4.5 — Epilogue:**
- Based on world tags chosen throughout the game

### Cross-Phase Stories
13. Stories that start in Phase 1 and end in later phases. Someone you helped in Phase 1 appears in the Shadow World. Someone who beat you early on — after exiting the Shadow World you beat them. A demon you showed mercy to returns in the final part to help or betray you.

### Companion Stories
14. Companion stories run across all phases. Over time they get items specific to them and maybe special things, and get stronger.
15. When a companion joins, you get a short acquaintance story and learn about them through gameplay.
16. When a companion story appears in the mission selection (only one companion story at a time): before each short story, there are 3 short descriptions to choose from. If it's a companion story, their name/portrait appears next to it. If you skip it, you miss that story (it's gone).

### Mission List
17. Create a small organized mission list file — short descriptions, which phase they belong to, what's there, how hard they are, whether there's a special item. A data file cataloging all missions with relatively short descriptions.

### Items System
18. There are regular items (random drops from missions, buyable in shops) and special items (mini-boss weapons, quest rewards like helping a wandering hermit). Also crafting materials — e.g., a strange magic orb that if brought to someone in the city, they can make something from it.

### City Services
19. In the city there's a Mage and a Blacksmith — at start nothing is available, but finding certain items unlocks options. In Phase 3 you can build things at the blacksmith from rare items found in the Shadow World.

### Terrain System
20. Each mission/short story has a terrain type by its type and level. Can place creatures randomly by terrain. Ranger gets a skill that gives bonuses in a chosen terrain — both in world story and in combat. Chosen at character creation.

### Traitor Party System
21. The traitor party changes each playthrough. In Phase 2 at first you just see them. In Phase 3 they have good items the final boss gave them — so build that boss fight accordingly. Different party types each time depending on which boss is in Phase 3 betrays you in Phase 2.

### Boss System
22. For each of the first 3 phases there's a selection of bosses — only the last (Phase 4) is always the same.
23. Each phase has a start story and an end story (boss).

### Story Setup System
24. At new game start, there's a story list that sets up in advance — all different bosses and mandatory events (like the King at start, chooseable companions). The rest are random. Once a story with a continuation is completed, its sequels immediately take slots randomly and kick out other stories. If a mission fails and has no continuation, the continuations disappear and random ones take their place.

### Other Godsent Groups
25. Godsent group stories throughout the game: groups you help that come back to help you, maybe a strong group you meet in the final part, a group that died mid-mission, a dead group you saw earlier whose corpses you find in the Shadow World and loot, another group stuck in the Shadow World you meet in an underground city. They activate certain world tags if you do missions with them. Multi-part mission chains.

### City Access Rules
26. When outside the Shadow World, there's always a SHOP button — can always go to city (costs time). In the Shadow World: shop button only appears during mission parts where you're in an underground city or meet a merchant. City in Shadow World = part of a mission, shop button appears only when trade is possible.

### Trainers
27. Some missions have trainers who teach skills for a price.

### Story Scripts
28. Need an option to add special scripts to stories. Examples:
    - At the start, 3 random NPCs appear and you choose 1 (small script)
    - A story where you lose the battle but were supposed to lose and stay alive instead of dying

### Story Ideas (for summary file)
29. Solo story: In Phase 2 (Shadow World) — a story where the player gets separated from the group by a trap, or some test during the journey that separates you. Or an arena in an underground city in the Shadow World.
30. Cool story: Fight a memory-affecting monster, then the mission starts from the middle (because your memory was hit), and has timeline jumps.

### Shadow World Naming
31. Instead of "Tower" call it "Shadow World" / "Underground World" — known for dangers and opportunities, dark mages go there, bad things emerge from it occasionally.

### Important Warning
32. Don't write actual stories until there's a bestiary, the world is built, and an AI story builder is made. Only then start with the summary file + story data. For now, just have summaries in a data file — at minimum the definite stuff: start and end of each phase, plus some ideas.

---

## SOURCE: Phone Notes (פתקי טלפון)

### Inventory UI
33. Inventory is GRID/SQUARES based. Clicking items. Inventory is half-screen character + half-screen bag. Bottom bar shows party member faces — click face to switch character. Potion belt is always fixed at bottom.

### Shop UI
34. Shop screen: half shop, half bag. Can click a character from the character bar to open their equipment and unequip items from them.
    - Icons for each character and for shop
    - Categories on the side: Potions, Equipment, Items (like keys)
    - In shop you can click potions tab to buy/sell potions — when in potion mode, instead of inventory the potion belt appears
    - Clicking an item in inventory shows "Unequip" button
    - In bag: "Equip" button. In shop: "Sell" or "Equip" button
    - Clicking any square shows a stat description box for the item
35. Do shop UI AFTER regular inventory is done.

### Item Class System
36. Items have: description, level, and formula (stat math).
    - Key = description only
    - Weapon = how much damage it does
    - Potion = how much it heals
    - Level is for later random generation in shops

### Opening Story (Phone Revision)
37. At start: God summons you, you choose avatar, then you appear with the King's summoners along with many other Godsents. Each gets minimal equipment based on class, gold, chooses a partner, then sets out.

### Shop Access
38. Shop is a screen you can enter and exit when you're in a shop area.

### Enemy Levels & Types
39. Make levels for creatures and items so you can randomly assign to missions/battles. Also maybe a type per monster (e.g., "forest monster") and type per phase — forest monsters appear in forests, etc.

### Shop Layout (Detailed)
40. Shop in squares format:
    - Party member faces at bottom, one is selected
    - Selected character's equipment shows as one row of squares
    - Above that: the bag (two rows of squares)
    - Above that: the shop inventory
    - Side categories: Potions, Equipment, Items (keys)
    - Can also click potions in the side tab to buy/sell potions — potion belt replaces inventory in this mode

### Image System
41. Changed mind: each story has different image(s) — need a system of loading, releasing, and holding images in memory.

### Solo Cultivation Story
42. Story where the player leaves the group in the Shadow World to go to some underground area for a cultivation/training arc.

### Companion Tags
43. Need tags for which companions are present — they bring their own stories and can participate in dialogue during missions.
