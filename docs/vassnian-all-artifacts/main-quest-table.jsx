import { useState } from "react";

const ACTS = [
  { id: "0", label: "Act 0 — Arrival", color: "#6abf6a", bg: "#0a180a" },
  { id: "1", label: "Act 1 — Growth", color: "#bfa86a", bg: "#18140a" },
  { id: "1.5", label: "Act 1.5 — Betrayal", color: "#bf4a4a", bg: "#1a0808" },
  { id: "2", label: "Act 2 — Shadow World", color: "#6a6abf", bg: "#0a0a18" },
  { id: "2.5", label: "Act 2.5 — Emergence", color: "#bf8abf", bg: "#180a18" },
  { id: "3", label: "Act 3 — Revenge", color: "#bf6a6a", bg: "#180a0a" },
  { id: "4", label: "Act 4 — Gorath", color: "#d4a574", bg: "#1a1208" },
];

const BOSS_THEMES = {
  "1": [
    { id: "goblin_king", label: "Goblin King", icon: "\u{1F451}" },
    { id: "bandit_warlord", label: "Bandit Warlord", icon: "\u2694" },
    { id: "corrupted_beast", label: "Corrupted Beast", icon: "\u{1F43B}" },
    { id: "cult_priest", label: "Cult High Priest", icon: "\u{1F52E}" },
  ],
  "2": [
    { id: "shadow_dragon", label: "Shadow Dragon", icon: "\u{1F409}" },
    { id: "deepkin_tyrant", label: "Deepkin Tyrant", icon: "\u{1F451}" },
    { id: "exiled_archmage", label: "Exiled Archmage", icon: "\u{1F9D9}" },
    { id: "ancient_construct", label: "Ancient Construct", icon: "\u2699" },
  ],
  "3": [
    { id: "commander", label: "The Commander", icon: "\u{1F6E1}" },
    { id: "prodigy", label: "The Prodigy", icon: "\u2728" },
    { id: "showman", label: "The Showman", icon: "\u{1F3AD}" },
    { id: "shepherd", label: "The Shepherd", icon: "\u{1F54A}" },
  ],
};

const QUESTS = [
  // ========== ACT 0 ==========
  {
    act: "0", sub: "0.1", boss: "universal",
    name: "The Awakening",
    desc: "You are pulled through the Circle of Arrival into the Kingdom of Caelmund. Dazed, confused, surrounded by mages and soldiers. The King himself greets you — desperate but dignified. You're not the only one summoned. Other Godsents arrived too. You're assigned to the King's Guild and offered your first companion — choose one of three.",
    note: "Tutorial. Class selection already done. Companion choice: pick 1 of 3 or refuse all.",
  },
  {
    act: "0", sub: "0.2", boss: "universal",
    name: "Proving Ground",
    desc: "Creatures have been spotted near the city walls — bolder than usual, attacking patrols. The King sends you on your first real mission. Clear the threat, learn to fight alongside your companion. You catch a glimpse of other Godsent parties heading out on their own missions. The world is bigger than you.",
    note: "First combat encounters. Introduces basic enemy types (Rank 1-2). Other Godsent parties visible but not interactive yet.",
  },
  // Act 0.3 — Traitor Meeting variants
  {
    act: "0", sub: "0.3", boss: "commander",
    name: "The War Table",
    desc: "The King summons all Godsent party leaders for a joint strategy briefing. The Commander leads the discussion — calm, precise, already thinking three steps ahead. He pulls you aside after and shares tactical observations about the growing threat. 'We should coordinate. Two strong parties working together are worth more than ten weak ones.' Your companion says he seems like a natural leader.",
    note: "Traitor foreshadow: Commander path. Player bonds through shared competence.",
  },
  {
    act: "0", sub: "0.3", boss: "prodigy",
    name: "Curious Minds",
    desc: "A magical anomaly is detected near the city — residual energy from the mass summoning. The Prodigy finds you and insists you investigate it together. She's brilliant, a little condescending, but genuinely excited. You solve the puzzle as a team. She says 'You're not as slow as the others. That's refreshing.' Your mage companion is fascinated by her.",
    note: "Traitor foreshadow: Prodigy path. Player bonds through intellectual respect.",
  },
  {
    act: "0", sub: "0.3", boss: "showman",
    name: "Heroes' Welcome",
    desc: "The Showman has organized a tavern celebration for all the Godsent parties. Free drinks, arm wrestling, war stories from back home. He throws his arm around you: 'To us! Ripped from our worlds, dumped in a medieval fantasy, and STILL having a better time than back home!' Your companions laugh. Everyone loves him. The night ends with a toast to victory.",
    note: "Traitor foreshadow: Showman path. Player bonds through camaraderie and fun.",
  },
  {
    act: "0", sub: "0.3", boss: "shepherd",
    name: "The Roadside Mercy",
    desc: "Returning from a mission, you find the Shepherd tending to wounded travelers attacked by beasts on the road. She's already bandaged three people and is working on a fourth. When she sees your companion's scratches from your earlier fight, she heals them without being asked. 'It costs nothing to be kind,' she says. Your priest companion calls her an inspiration.",
    note: "Traitor foreshadow: Shepherd path. Player bonds through witnessing genuine-seeming compassion.",
  },

  // ========== ACT 1 ==========
  // --- 1.1: Boss Introduction ---
  {
    act: "1", sub: "1.1", boss: "goblin_king",
    name: "Raids at Dawn",
    desc: "Villages on the eastern border are being raided — but not by random goblins. These attacks are coordinated: multiple strike points, supply lines cut first, sentries eliminated silently. You respond to the latest raid and find organized goblin war parties with matching war paint, formation tactics, and siege ladders. Someone is uniting the goblin tribes into an army.",
    note: "Goblin enemies. Player realizes this isn't random — it's military strategy from a non-human mind.",
  },
  {
    act: "1", sub: "1.1", boss: "bandit_warlord",
    name: "The Crimson Road",
    desc: "Trade caravans are vanishing. The roads between cities have become death traps. You escort a merchant convoy and are ambushed — not by desperate bandits but by disciplined soldiers wearing crimson scarves. They fight with expensive weapons and coordinated tactics. A deserter you capture whispers about a Warlord who appeared from nowhere with a glowing artifact that 'makes men kneel.'",
    note: "Bandit enemies. The artifact is key — dark power source that elevated a common thief to warlord.",
  },
  {
    act: "1", sub: "1.1", boss: "corrupted_beast",
    name: "The Hunter's Mark",
    desc: "A patrol of kingdom soldiers was torn apart in the forest. Not eaten — destroyed. The tracks don't match any known creature. Survivors describe something massive, wrong, that moved like it was hunting specifically. You investigate and find the trail — it leads toward your last known position. The beast isn't random. It's hunting Godsents. Something is driving it toward you.",
    note: "No direct fight yet. Tension, tracking, finding the aftermath. The beast appears briefly — too strong to engage. You flee.",
  },
  {
    act: "1", sub: "1.1", boss: "cult_priest",
    name: "Whispers in Stone",
    desc: "Ancient temple sites across the region are being vandalized — but not randomly. Specific stones are being defaced, specific runes scratched out. A priest explains these are seal anchor points, part of the binding magic from the Sealing War. Someone knows exactly which stones matter and is methodically weakening the barrier. You find a hidden ritual site with fresh blood and dark symbols.",
    note: "Cult enemies. Introduces the seal mythology and connects to Gorath's prison. First direct evidence of organized sabotage.",
  },

  // --- 1.2: Boss Escalation ---
  {
    act: "1", sub: "1.2", boss: "goblin_king",
    name: "The Goblin Court",
    desc: "Scouts report a massive goblin settlement in the mountain passes — not a camp, a fortress. Thousands of goblins united under a crowned king who sits on a throne of stolen weapons. They have siege towers, war beasts, and goblin sappers building tunnels toward Caelmund's farmlands. This isn't a raid anymore. It's a war. The King orders you to infiltrate and assess the threat.",
    note: "Stealth/combat mission. Player sees the scale of the goblin army. The Goblin King is glimpsed but not fought yet.",
  },
  {
    act: "1", sub: "1.2", boss: "bandit_warlord",
    name: "The Warlord's Artifact",
    desc: "You track the crimson bandits to their network of hideouts. Prisoners confirm it: the Warlord was nobody — a failed thief who found something in ancient ruins. The artifact changed him. His eyes glow when he holds it. Men who resist his commands feel pain. It's not charisma — it's magical domination. You need to find his main stronghold and understand what the artifact really is.",
    note: "The artifact is connected to Gorath's influence — dark power leaking through the weakening seal. The Warlord may not even understand what he's wielding.",
  },
  {
    act: "1", sub: "1.2", boss: "corrupted_beast",
    name: "Run",
    desc: "The beast finds you again. It crashes through the forest mid-mission, targeting your party specifically. It's larger now — darker, its body twisted further, corruption visibly spreading across its hide. You can't win this fight. Your companion screams to retreat. A desperate chase through hostile terrain, using the environment to slow it down. You escape, barely. It roars behind you. It's getting stronger.",
    note: "Scripted escape sequence. Player CANNOT win this fight. The beast gets a dramatic entrance and the player learns it's growing — feeding on Gorath's leaking power.",
  },
  {
    act: "1", sub: "1.2", boss: "cult_priest",
    name: "The Hidden Congregation",
    desc: "Following clues from the ritual sites, you discover the cult has infiltrated multiple villages. Farmers, blacksmiths, even a minor noble — all secret followers. Their High Priest communicates through encrypted messages and blood rituals. You raid a secret meeting and capture cultists, but the High Priest escapes. The captured cultists smile: 'The seal thins. He stirs. You feel it too, Godsent.'",
    note: "Cult enemies mixed with civilian infiltrators. Moral complexity — some cultists joined out of fear, not faith. The High Priest is smart and elusive.",
  },

  // --- 1.2 ALSO: Traitor Bonding ---
  {
    act: "1", sub: "1.2", boss: "commander",
    name: "Shield Brothers",
    desc: "A dangerous mission requires two Godsent parties. The Commander volunteers his team. Fighting alongside him, his tactical genius saves your party when an ambush goes wrong. At camp after, sitting by the fire, he says quietly: 'I've led soldiers my whole life. Most follow orders. You think for yourself. That's rare.' He offers his hand. 'Whatever comes — I'd trust you at my back.'",
    note: "Traitor bonding: Commander. The handshake will be remembered during the betrayal. His compliment is genuine — he respects the player, which makes the betrayal calculated, not emotional.",
  },
  {
    act: "1", sub: "1.2", boss: "prodigy",
    name: "The Forgotten Formula",
    desc: "The Prodigy requests your help with dangerous magical research — ancient texts about the Sealing War that could explain the weakening barrier. Working together in a crumbling library, you solve a puzzle she couldn't crack alone. She goes quiet, then: 'I don't say this often. You kept up with me. Most people can't.' She shares her real findings — the seal IS breaking. She seems genuinely worried.",
    note: "Traitor bonding: Prodigy. The research findings are REAL — she mixes truth with manipulation. Her concern is partly genuine; she just chose the wrong side.",
  },
  {
    act: "1", sub: "1.2", boss: "showman",
    name: "Blood and Glory",
    desc: "A massive creature is terrorizing a mountain pass. The Showman challenges you to hunt it together — 'Whoever lands the killing blow buys drinks!' The hunt is exhilarating: tracking, ambushes, close calls. He fights recklessly but brilliantly. After the kill, covered in monster blood, he grins: 'THIS is what we were summoned for!' Your companions are buzzing with adrenaline. Best day since you arrived.",
    note: "Traitor bonding: Showman. Pure fun. The player LIKES this guy. That's what makes Act 3 so hard — attacking someone your companions genuinely enjoyed being around.",
  },
  {
    act: "1", sub: "1.2", boss: "shepherd",
    name: "The Plague Village",
    desc: "A village is dying — a strange sickness spreading fast. The Shepherd arrives with medicine and organization. You protect the perimeter from creatures drawn by the weakness while she works through the night, healing one villager at a time. By dawn, she's exhausted but smiling. Children cling to her. Your priest companion says: 'She's what we should all aspire to be.' She asks for nothing in return.",
    note: "Traitor bonding: Shepherd. The most painful foreshadow — her compassion seems completely real. The player's own companions admire her. The sickness may or may not be connected to Gorath.",
  },

  // --- 1.3: Boss Climax ---
  {
    act: "1", sub: "1.3", boss: "goblin_king",
    name: "The Goblin Throne",
    desc: "Full assault on the Goblin King's mountain fortress. Fight through organized goblin hordes — shield walls, goblin archers, war beasts, sappers detonating tunnel charges. The deeper you go, the more disciplined they become. The Goblin King waits on his throne — massive, scarred, wearing a crude iron crown. He fights with terrifying intelligence: repositioning his guards, exploiting terrain, adapting to your tactics mid-battle.",
    note: "ACT 1 BOSS FIGHT. The Goblin King is a tactical fighter — his strength is his army and his mind, not raw power. Multi-phase: first his guards, then him personally.",
  },
  {
    act: "1", sub: "1.3", boss: "bandit_warlord",
    name: "The Artifact's Price",
    desc: "Storm the Warlord's hidden stronghold — a fortified canyon base with stolen wealth piled high. His crimson soldiers fight fanatically, some showing signs of magical corruption from proximity to the artifact. The Warlord awaits in his throne room, the artifact pulsing on his chest. He fights with corrupted power — unnatural strength, dark energy blasts, his eyes blazing. When defeated, the artifact cracks. Choice: destroy it or claim it?",
    note: "ACT 1 BOSS FIGHT. The Warlord is a victim of Gorath's influence as much as a villain. The artifact choice sets a story flag — claiming dark power has consequences.",
  },
  {
    act: "1", sub: "1.3", boss: "corrupted_beast",
    name: "Stand and Fight",
    desc: "The beast comes for you a third time. But you're stronger now. It crashes through the treeline — massive, barely recognizable as a natural creature. Corruption has consumed it: shadow tendrils, glowing wounds, wrong proportions. It's been feeding on Gorath's leaking power, growing into something between animal and demon. This time you don't run. The fight is brutal, primal, and personal. You end its suffering.",
    note: "ACT 1 BOSS FIGHT. Third encounter pays off — player has been running from this thing and finally faces it. Emotional beat: this was once a normal animal. Gorath's corruption made it a weapon.",
  },
  {
    act: "1", sub: "1.3", boss: "cult_priest",
    name: "The Seal Ritual",
    desc: "Intelligence reveals the cult is attempting a major ritual at the largest remaining seal anchor — if they crack it, Gorath's prison weakens catastrophically. Race to the ancient site. Cultists performing blood magic in concentric circles. The High Priest stands at the center, channeling dark energy directly from Gorath through the thinning barrier. He fights with borrowed divine power — devastating but unstable. Stop the ritual before the seal breaks.",
    note: "ACT 1 BOSS FIGHT. The High Priest channels Gorath's power — a preview of Act 4. The ritual is a timer mechanic: fail too slowly and the seal cracks further regardless.",
  },

  // ========== ACT 1.5 ==========
  {
    act: "1.5", sub: "1.5", boss: "commander",
    name: "The Trap",
    desc: "The King orders a critical joint mission — both your party and the Commander's must investigate a dangerous ancient site deep in hostile territory. The Commander suggests splitting up: 'You take the lower passage, we'll secure the perimeter.' It makes tactical sense. You go deeper. The barrier activates behind you. Through the shimmering wall, the Commander stands with his party, unhurried. 'You were a variable I needed to remove. Nothing more.' He turns and walks away.",
    note: "BETRAYAL. Cold, professional. No anger, no satisfaction. Just efficiency. The companion's reaction is devastation — they TRUSTED him.",
  },
  {
    act: "1.5", sub: "1.5", boss: "prodigy",
    name: "The Trap",
    desc: "A joint expedition to an ancient ruin — the Prodigy says she's found clues about the weakening seal. Deep inside, she asks you to examine a chamber alone: 'The resonance is strongest there. I need readings from both sides.' You enter. The barrier snaps shut. She stands on the other side, arms crossed, head tilted. 'I thought you'd figure it out sooner. I left hints. You just weren't clever enough.' She teleports away without looking back.",
    note: "BETRAYAL. Intellectual superiority. She's disappointed you didn't see it coming. The worst part: she DID leave hints — the player can remember moments that now make terrible sense.",
  },
  {
    act: "1.5", sub: "1.5", boss: "showman",
    name: "The Trap",
    desc: "The Showman suggests a joint mission: 'One last adventure before things get serious!' He's excited, energetic, cracking jokes as you go deeper into the ruins. He volunteers you for the dangerous part — 'You're tougher than me, hero!' The barrier slams shut. Through it, his grin changes. Same smile, different eyes. 'Oh come on, don't look so hurt! We had fun, didn't we? The drinks were real. The friendship?' He shrugs. 'Eh.' He laughs as he walks away.",
    note: "BETRAYAL. The cruelest version — he enjoyed your friendship AND your suffering. The companion is furious, not just hurt. They want revenge.",
  },
  {
    act: "1.5", sub: "1.5", boss: "shepherd",
    name: "The Trap",
    desc: "A relief mission — the Shepherd asks for help reaching trapped refugees in dangerous ruins. 'People are dying. I can't do it alone.' Of course you go. Deep inside, she guides you forward: 'They're just past that chamber.' The barrier seals. She stands on the other side, and for the first time, her warmth has a crack in it. 'I'm sorry. I truly am. But you don't understand what's coming. I'm protecting them — all of them. Gorath isn't what you think.' She places her hand on the barrier, then leaves.",
    note: "BETRAYAL. The most disturbing — she's sad about it. She believes she's saving people by serving Gorath. The companion doesn't know whether to hate her or pity her.",
  },

  // ========== ACT 2 ==========
  // --- 2.1: Shadow World Introduction ---
  {
    act: "2", sub: "2.1", boss: "shadow_dragon",
    name: "Dragon's Territory",
    desc: "You stumble through the aftermath of the betrayal into a vast underground cavern. Bioluminescent crystals light a dead forest of stone. Everything is burned. Massive claw marks score the walls. Locals you find huddled in a cave whisper: 'You've entered the Ashen Reach. The dragon claims everything here. It has for a thousand years.' Charred remains of those who challenged it line the path forward. The only route to civilization goes through its domain.",
    note: "Survival. Player is weakened from the betrayal. The dragon is established as ancient and territorial — not evil, just absolute.",
  },
  {
    act: "2", sub: "2.1", boss: "deepkin_tyrant",
    name: "The Sunless Court",
    desc: "You fall into the territory of the Deepkin — an ancient civilization of pale, eyeless beings who have lived underground since before Caelmund existed. Their soldiers capture you and drag you to a carved crystal city. The Tyrant sits on a throne of black stone: 'Surface-dweller. You enter our domain uninvited. Here, you are nothing. Serve the Court or be fed to the deep.' The Deepkin watch you with eyeless faces, judging.",
    note: "Political/survival. The Deepkin are alien but not monsters — they have culture, law, history. The Tyrant is a ruler, not a beast. Negotiation is possible but dangerous.",
  },
  {
    act: "2", sub: "2.1", boss: "exiled_archmage",
    name: "The Tower in the Dark",
    desc: "A massive spire rises from the cavern floor, covered in protective wards that crackle with energy. Surrounding settlements live in its shadow — literally and figuratively. The Exiled Archmage fled the surface decades ago after being condemned for forbidden experiments. Now he rules this pocket of the Shadow World through magical supremacy. His constructs patrol the streets. His voice echoes from nowhere: 'Another surface castaway. How delightful. You'll find I'm the only law that matters here.'",
    note: "The Archmage is unstable but brilliant. His tower contains knowledge that could help return to the surface. Getting it means dealing with a paranoid genius.",
  },
  {
    act: "2", sub: "2.1", boss: "ancient_construct",
    name: "The Builder's Grave",
    desc: "You discover the ruins of something impossibly old — architecture that predates every civilization you've encountered, surface or underground. The walls hum with residual energy. Automated guardians still patrol: stone and crystal constructs following orders from a dead civilization. One guardian is massive — it blocks the only passage forward, scanning everything that approaches. It doesn't attack immediately. It's waiting for something. A password? A key? A worthy inheritor?",
    note: "Mystery/exploration. The Builder civilization is ancient lore — what were they? Why did they fall? The construct is a puzzle boss as much as a combat boss.",
  },

  // --- 2.2: Shadow World Escalation ---
  {
    act: "2", sub: "2.2", boss: "shadow_dragon",
    name: "The Dragon's Bargain",
    desc: "The dragon finds you. It's ancient, intelligent, and curious — it hasn't seen a Godsent in centuries. It lands before you, enormous, and speaks: 'You're far from your sky, little hero. I could end you now.' Instead, it offers a deal: perform a task in the deeper caverns (clear a rival predator from its territory) and it will let you pass. Refuse, and burn. Your companion argues it's a trap. But the dragon's eyes hold something unexpected — respect.",
    note: "Choice: accept the bargain (morally grey — you're doing a dragon's dirty work) or find another way (harder, more dangerous, but independent). Both paths lead to the boss fight eventually.",
  },
  {
    act: "2", sub: "2.2", boss: "deepkin_tyrant",
    name: "Rebellion Below",
    desc: "Within the Deepkin Court, not everyone bows willingly. A rebel faction contacts you in secret — they oppose the Tyrant's isolationist cruelty and want change. Their leader is a Deepkin scholar who has studied surface history. 'Help us overthrow him, and we will help you return home.' But the rebels have their own agenda — they want to conquer the neighboring settlements, not free them. Allies with strings attached.",
    note: "Political intrigue underground. The rebels are useful but not good. Player must navigate competing factions. The Tyrant is cruel but stable; the rebels are ambitious but chaotic.",
  },
  {
    act: "2", sub: "2.2", boss: "exiled_archmage",
    name: "The Mad Scholar",
    desc: "You gain an audience with the Archmage. His tower is filled with incredible research — maps of the Shadow World, theories about the seal, even notes on Gorath. He COULD help you. But he's unraveling. Paranoid, talking to himself, experiments growing more dangerous. He'll share knowledge IF you retrieve components from the deep tunnels for his latest experiment. Your companion warns: whatever he's building, it's not safe. But his knowledge might be the only way home.",
    note: "The Archmage's research is genuinely valuable — but his instability is escalating. His 'experiment' may be connected to the ancient power sources in the Shadow World.",
  },
  {
    act: "2", sub: "2.2", boss: "ancient_construct",
    name: "The Core",
    desc: "You discover the Builder ruins go deeper than anyone knew. At their heart: a still-functioning power core, humming with energy that has lasted millennia. The massive construct guardian is connected to it — drawing power, following ancient directives. But the core is degrading. If it fails, the guardian goes berserk and the entire cavern system collapses. You can try to repair it (gaining the construct as an ally), override it (seizing the power), or destroy it (eliminating the threat permanently).",
    note: "Three-way choice with major consequences. The Builder technology is beyond current understanding — surface or underground. What you do here echoes into Act 3-4.",
  },

  // --- 2.3: Shadow World Boss ---
  {
    act: "2", sub: "2.3", boss: "shadow_dragon",
    name: "Dragonslayer",
    desc: "The dragon breaks its word — or you break yours. Either way, it comes to the final confrontation: an ancient wyrm in a cavern of crystal and fire. It fights with centuries of cunning — repositioning through tunnels, using its environment, targeting your weakest member. Its scales absorb magic. Its breath melts stone. Multi-phase: first aerial strafing runs in the cavern, then ground combat as you force it down, then a desperate final stand as it fights cornered and furious.",
    note: "ACT 2 BOSS. Epic dragon fight. The dragon's intelligence makes it unpredictable — it adapts to player tactics. Possible respect-based ending if player honored the bargain.",
  },
  {
    act: "2", sub: "2.3", boss: "deepkin_tyrant",
    name: "Overthrow",
    desc: "Whether allied with rebels or acting alone, you assault the Tyrant's throne room in the crystal city. Deepkin elite guards in ancient armor, traps built into the architecture, and the Tyrant himself — wielding weapons from a forgotten age, enhanced by Shadow World minerals that amplify his strength. He fights like a king: commanding guards, using the throne room's defenses, refusing to fall. The eyeless face shows no fear. 'I have ruled for centuries. You are a moment.'",
    note: "ACT 2 BOSS. The Tyrant is a tank — heavy armor, environmental advantages, elite guards. The outcome affects Deepkin society: who rules after matters for potential Act 3-4 allies.",
  },
  {
    act: "2", sub: "2.3", boss: "exiled_archmage",
    name: "The Madman's Last Spell",
    desc: "The Archmage has lost it completely. His experiment has warped the tower — reality bends inside, rooms shift, gravity reverses. He attacks with layered magic: summoned creatures, environmental warping, counterspells that nullify your abilities. Each floor of the tower is a different magical nightmare. At the top, he stands in a storm of raw magical energy, barely human anymore. 'I UNDERSTAND now! The seal, the god, ALL of it! And I will NEVER let anyone take this knowledge from me!'",
    note: "ACT 2 BOSS. A gauntlet fight through a warped tower, then the Archmage himself. His research notes survive if you win — crucial lore about Gorath and the seal.",
  },
  {
    act: "2", sub: "2.3", boss: "ancient_construct",
    name: "The Awakened",
    desc: "The massive construct activates fully — whatever you did at the core triggered its final protocol. Building-sized, made of materials that shouldn't exist, it moves with mechanical precision through corridors designed for it. Every attack is a calculated pattern. Every pattern shifts when you adapt. The Builders made this to guard against something — and now it's decided YOU are the threat. Ancient weapons, shield projections, area denial. A fight against perfect engineering.",
    note: "ACT 2 BOSS. Pattern-based boss — learn its cycles, exploit windows. The construct isn't evil — it's following programming. Possible non-lethal solution if player found Builder keys.",
  },

  // ========== ACT 2.5 ==========
  {
    act: "2.5", sub: "2.5", boss: "universal",
    name: "Return to the Light",
    desc: "You find the way out — a passage to the surface that no one has used in centuries. The climb is long, exhausting, and emotional. Your companion walks beside you in silence. When you finally break through to open sky, the light blinds you. You fall to your knees. Fresh air. Wind. Sunlight. But Caelmund has changed. Gorath's influence is everywhere — corrupted wildlife, darkened skies near the horizon, abandoned villages. And in the capital, the traitor party rules as heroes. Everyone thinks you're dead.",
    note: "Emotional transition. No combat. The world has changed while you were underground. Setup for the Act 3 shadow war against the traitor party.",
  },

  // ========== ACT 3 ==========
  // --- 3.1: Working from Shadows ---
  {
    act: "3", sub: "3.1", boss: "commander",
    name: "The General's Blindspot",
    desc: "The Commander has become the King's military right hand. Caelmund's army answers to him. Soldiers patrol every street. Accusing him publicly means fighting the entire military. You work from the shadows: finding officers who've noticed inconsistencies, soldiers who've seen him communicate with dark figures at night. One veteran tells you: 'His orders have been... strange. Patrols redirected away from certain locations. I thought it was strategy. Now I'm not sure.'",
    note: "Stealth/investigation. Player builds a case while avoiding military patrols that would recognize them. The Commander is methodical — his conspiracy is well-hidden.",
  },
  {
    act: "3", sub: "3.1", boss: "prodigy",
    name: "Unweaving the Web",
    desc: "The Prodigy controls Caelmund's magical defense network — wards, detection spells, scrying. She KNOWS you're alive the moment you enter the kingdom. You need to move through gaps in her web, recruit someone who understands her magic, find blind spots. A former apprentice of hers, dismissed for asking too many questions, offers help: 'She changed after you disappeared. The wards she built... they're not just defense. They're surveillance. She sees everything.'",
    note: "Cat-and-mouse with a genius. The Prodigy adapts her detection web every time you slip through. Escalating tension — she's hunting you while you're hunting her.",
  },
  {
    act: "3", sub: "3.1", boss: "showman",
    name: "The Legend's Lie",
    desc: "The Showman is Caelmund's beloved hero. Songs in every tavern. Children play 'Godsent' in the streets imitating him. He feeds the poor, tells stories, shakes every hand. Attacking him would make YOU the villain. You need witnesses, evidence, people brave enough to speak against a man the kingdom loves. A serving girl at his favorite tavern whispers: 'He's different when the doors close. I've heard things. But who would believe me over him?'",
    note: "Social warfare. The player must dismantle a reputation, not a fortress. Finding evidence of cruelty hidden behind charisma. The public is on HIS side.",
  },
  {
    act: "3", sub: "3.1", boss: "shepherd",
    name: "The Mercy Trap",
    desc: "The Shepherd runs refugee camps across Caelmund. Thousands of displaced people — victims of Gorath's growing influence — depend on her food, medicine, and shelter. She IS their survival. Exposing her means those camps collapse. You need to build alternative support first: convince other healers, secure supply lines, find people willing to take over her operations. A refugee child asks: 'Are you here to help like the nice lady?' The weight of what you're about to do hits hard.",
    note: "The hardest Act 3 path morally. You're not just fighting a villain — you're dismantling a support system real people depend on. Build before you destroy.",
  },

  // --- 3.2: Escalation ---
  {
    act: "3", sub: "3.2", boss: "commander",
    name: "Turncoats",
    desc: "You've identified key officers who suspect the Commander. One by one, in secret meetings in back alleys and abandoned barracks, you flip them. But the Commander notices gaps in loyalty. He tightens security, reassigns suspicious officers, sets traps. He's hunting for a conspiracy — and he's GOOD at it. A trusted officer you turned is found dead. Message received: the Commander knows someone is working against him. The clock is ticking.",
    note: "Escalation. The Commander counter-maneuvers. Each step forward risks exposure. The dead officer raises the stakes — this isn't just politics anymore.",
  },
  {
    act: "3", sub: "3.2", boss: "prodigy",
    name: "Counterspell",
    desc: "Piece by piece, you sabotage the Prodigy's ward network — disabling nodes, corrupting sensor lines, creating blind zones. She adapts faster than you expected. Every time you take down a node, she builds two more. Her former apprentice realizes: 'She's not just detecting intrusion anymore. She's mapping YOUR magical signature. Every time you touch her wards, she learns more about how you move.' You need a completely different approach or she'll pin you down.",
    note: "Magical arms race. The Prodigy is always adapting. Player needs creativity — maybe use non-magical methods, recruit mundane allies, or turn her own paranoia against her.",
  },
  {
    act: "3", sub: "3.2", boss: "showman",
    name: "The Mask Cracks",
    desc: "You've gathered testimonies — servants, soldiers, a former party member who couldn't stomach his cruelty. But releasing it means risking public backlash. 'That's lies about our hero!' The Showman senses something is wrong. His public appearances become more frequent, more generous. He doubles down on the charm offensive. Behind closed doors, he's getting paranoid, violent with his inner circle. One of his own people secretly contacts you: 'He hit Lysette last night. For questioning him. I'm done.'",
    note: "The mask is slipping but the public doesn't see it. Timing the exposure is critical — too early and people won't believe it, too late and he fortifies.",
  },
  {
    act: "3", sub: "3.2", boss: "shepherd",
    name: "Building Hope",
    desc: "You've been quietly building a parallel support network: healers, supply chains, volunteer shelters. Refugees are starting to have alternatives. The Shepherd notices the shift — fewer people coming to her camps, whispers of another group helping. She responds by INCREASING her generosity: more food, more medicine, longer hours. Her followers grow more devoted. But the strain shows — she's making decisions faster, checking over her shoulder. Nessa's surveillance spells sweep the city more frequently.",
    note: "A war of compassion. The player is literally competing to help people better than the villain does. The Shepherd's escalation reveals her need for control beneath the kindness.",
  },

  // --- 3.3: Traitor Boss Fight ---
  {
    act: "3", sub: "3.3", boss: "commander",
    name: "Checkmate",
    desc: "Your evidence is ready. Your allies are in position. You corner the Commander in his war room — the same kind of room where he first earned your trust. His loyal soldiers surround him: Ser Aldric, Miriel, Halvard. He draws his sword without surprise. 'I knew it was you. I calculated you'd survive.' He fights the way he always has: precise, tactical, commanding his team like chess pieces. But you know his patterns now. You learned them when you were allies.",
    note: "ACT 3 BOSS. Military precision fight. The Commander's strength is coordination — disrupting his formation is key. Callback to Act 1 bonding: his tactics are familiar because he showed them to you.",
  },
  {
    act: "3", sub: "3.3", boss: "prodigy",
    name: "Outsmarted",
    desc: "You lure the Prodigy to a location of YOUR choosing — somewhere her wards don't reach, her escape routes are blocked, and her team is separated. For the first time, she looks surprised. 'Clever. I underestimated you.' Then the surprise turns to fury. She fights with everything: devastating spells, teleportation, counterspells that nullify your abilities. Shade strikes from shadows. Yara snipes from range. Tormund shields her. She fights like someone who has never lost — and can't accept that she might.",
    note: "ACT 3 BOSS. The Prodigy is the hardest boss to pin down. Priority: kill Tormund (her shield), track Shade (assassin), close distance on Yara (sniper), then overwhelm the Prodigy before she adapts.",
  },
  {
    act: "3", sub: "3.3", boss: "showman",
    name: "The Curtain Falls",
    desc: "You expose him publicly — evidence, witnesses, his own people turning against him. The crowd goes silent. The Showman stands on the stage where he's performed a hundred times, and for a moment, the mask drops completely. Then he smiles — a real smile, cold and hungry. 'Fine. If I can't be loved, I'll be FEARED.' He draws both blades. Sister Corin's aura blazes. Old Harken begins chanting. Lysette's song turns dark. He fights with savage abandon — no more performance. Just violence.",
    note: "ACT 3 BOSS. Public confrontation. Kill Old Harken FIRST (resurrection). Disrupt Lysette's buffs. Break Corin's aura. The Showman gets stronger when desperate — rush him or he escalates.",
  },
  {
    act: "3", sub: "3.3", boss: "shepherd",
    name: "Shattered Halo",
    desc: "Your alternative network is ready. The refugees are safe. You confront the Shepherd at her main camp. She doesn't run. She stands among the people she's been helping — using them as a moral shield. 'Look at them. Look at what I've built. You want to tear this down?' When you present the truth, some followers leave. Others stay. She armors up with tears in her eyes. Dorek steps in front of her. Nessa's vines spread across the ground. Brother Fenn drops the cheerful mask. 'I tried to do this peacefully,' she whispers. Then heals her team to full.",
    note: "ACT 3 BOSS. War of attrition. Dorek bodyguards, Nessa controls terrain, Fenn stacks debuffs, Shepherd heals everything. The player must out-damage her healing or isolate her from her team.",
  },

  // ========== ACT 4 ==========
  {
    act: "4", sub: "4.1", boss: "universal",
    name: "The Descent",
    desc: "The seal is nearly broken. The sky over Caelmund has darkened. Gorath's influence bleeds through in visible waves — corruption spreading across the land, creatures going mad, temples losing power. You lead what allies remain into Gorath's domain: the ancient prison beneath the kingdom, now cracked open and pulsing with dark energy. The architecture is impossible — built by gods to contain a god. Every step deeper, Gorath's presence grows heavier. He knows you're coming. He's been waiting.",
    note: "Final dungeon begins. Allies from previous acts may appear based on story flags. The environment itself is hostile — Gorath's domain warps reality.",
  },
  {
    act: "4", sub: "4.2", boss: "universal",
    name: "The Last Seal",
    desc: "The final seal anchor — the original, the strongest, the one everything else was built around. It's cracking. Dark energy pours through fissures. Gorath's voice echoes in your mind for the first time: not a monster's roar but a king's command. 'You were chosen by a coward god who couldn't face me himself. I respect that you came. But you will kneel.' Your choices throughout the game determine what you bring to this moment: allies, knowledge from the Shadow World, the power of bonds forged or broken.",
    note: "Pre-boss gauntlet. Everything converges. Shadow World lore may reveal a way to permanently destroy Gorath rather than reseal him. Companion bonds determine available abilities.",
  },
  {
    act: "4", sub: "4.3", boss: "universal",
    name: "God of Conquest",
    desc: "Gorath manifests. Not a demon, not a monster — a god. An entity of ancient, terrible majesty. He fights with the weight of millennia: reality-warping attacks, corruption waves that turn your buffs into debuffs, a presence so heavy your party members struggle to stand. Multi-phase: first his avatar, then his true form as the seal shatters fully. Everything you've learned, everyone you've saved or lost, every choice — it all matters here. The Watcher god's gift activates: you were chosen for this moment.",
    note: "FINAL BOSS. Multi-phase. Companion bond levels affect available ultimate abilities. Shadow World knowledge unlocks secret mechanics. Previous act choices determine ending paths. This is everything.",
  },
];

function QuestCard({ q, actColor }) {
  const isBoss = q.name.includes("Throne") || q.name.includes("Artifact's Price") ||
    q.name.includes("Stand and Fight") || q.name.includes("Seal Ritual") ||
    q.name.includes("Dragonslayer") || q.name.includes("Overthrow") ||
    q.name.includes("Madman's Last") || q.name.includes("Awakened") ||
    q.name.includes("Checkmate") || q.name.includes("Outsmarted") ||
    q.name.includes("Curtain Falls") || q.name.includes("Shattered Halo") ||
    q.name.includes("God of Conquest");

  const isBetrayal = q.act === "1.5";
  const borderColor = isBetrayal ? "#bf4a4a" : isBoss ? "#d4a574" : "#1a1815";

  return (
    <div style={{
      background: "#0c0c10",
      border: `1px solid ${borderColor}`,
      borderLeft: `3px solid ${isBoss ? "#d4a574" : isBetrayal ? "#bf4a4a" : actColor}`,
      borderRadius: 4,
      padding: "12px 16px",
      marginBottom: 8,
    }}>
      <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6, flexWrap: "wrap" }}>
        <span style={{
          fontSize: 10, fontFamily: "monospace", color: "#5a5248",
          background: "#08080c", padding: "2px 6px", borderRadius: 3,
        }}>
          {q.sub}
        </span>
        {isBoss && (
          <span style={{
            fontSize: 9, color: "#0a0a0f", background: "#d4a574",
            padding: "2px 6px", borderRadius: 3, fontWeight: 700,
          }}>BOSS FIGHT</span>
        )}
        {isBetrayal && (
          <span style={{
            fontSize: 9, color: "#0a0a0f", background: "#bf4a4a",
            padding: "2px 6px", borderRadius: 3, fontWeight: 700,
          }}>BETRAYAL</span>
        )}
        {q.boss !== "universal" && (
          <span style={{
            fontSize: 9, color: actColor,
            border: `1px solid ${actColor}44`,
            padding: "2px 6px", borderRadius: 3,
          }}>
            {q.boss.replace(/_/g, " ").toUpperCase()}
          </span>
        )}
        {q.boss === "universal" && (
          <span style={{
            fontSize: 9, color: "#6a6058",
            border: "1px solid #2a2520",
            padding: "2px 6px", borderRadius: 3,
          }}>ALL PATHS</span>
        )}
        <span style={{ fontSize: 14, color: "#d4c4b0", fontWeight: 600 }}>
          {q.name}
        </span>
      </div>
      <div style={{ fontSize: 11, color: "#a09888", lineHeight: 1.7, marginBottom: 6 }}>
        {q.desc}
      </div>
      <div style={{
        fontSize: 10, color: "#6a6058", lineHeight: 1.5,
        borderTop: "1px solid #1a1815", paddingTop: 6,
        fontStyle: "italic",
      }}>
        {q.note}
      </div>
    </div>
  );
}

export default function MainQuestTable() {
  const [selectedAct, setSelectedAct] = useState(null);
  const [selectedBoss, setSelectedBoss] = useState({});

  const getFilteredQuests = () => {
    let filtered = QUESTS;

    if (selectedAct) {
      filtered = filtered.filter(q => q.act === selectedAct);
    }

    // Apply boss filters per act
    filtered = filtered.filter(q => {
      if (q.boss === "universal") return true;
      const actBossFilter = selectedBoss[q.act];
      if (!actBossFilter) return true;
      return q.boss === actBossFilter;
    });

    return filtered;
  };

  const quests = getFilteredQuests();
  const actForColor = (actId) => ACTS.find(a => a.id === actId)?.color || "#5a5248";

  const stats = {
    total: QUESTS.length,
    universal: QUESTS.filter(q => q.boss === "universal").length,
    variants: QUESTS.filter(q => q.boss !== "universal").length,
    bossFights: 13,
  };

  return (
    <div style={{
      minHeight: "100vh",
      background: "#0a0a0f",
      color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 920, margin: "0 auto" }}>
        {/* Header */}
        <div style={{
          textAlign: "center", marginBottom: 20,
          borderBottom: "1px solid #2a2520", paddingBottom: 16,
        }}>
          <h1 style={{
            fontSize: 24, color: "#d4a574", fontWeight: 400,
            letterSpacing: 4, textTransform: "uppercase", margin: 0,
          }}>
            \u2694 Vassnian Main Quest Table \u2694
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            {stats.total} Total Quests \u2022 {stats.universal} Universal \u2022 {stats.variants} Boss-Specific Variants \u2022 {stats.bossFights} Boss Fights \u2022 Act 0 \u2192 Gorath
          </p>
        </div>

        {/* Act Filter */}
        <div style={{ marginBottom: 12 }}>
          <div style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 2, marginBottom: 6 }}>
            Filter by Act
          </div>
          <div style={{ display: "flex", gap: 4, flexWrap: "wrap" }}>
            <button
              onClick={() => setSelectedAct(null)}
              style={{
                background: !selectedAct ? "#1a1815" : "transparent",
                border: `1px solid ${!selectedAct ? "#2a2520" : "#1a1815"}`,
                color: !selectedAct ? "#d4a574" : "#4a4238",
                padding: "5px 12px", borderRadius: 3,
                cursor: "pointer", fontSize: 10, fontFamily: "inherit",
              }}
            >ALL ACTS</button>
            {ACTS.map(a => (
              <button
                key={a.id}
                onClick={() => { setSelectedAct(a.id); }}
                style={{
                  background: selectedAct === a.id ? a.bg : "transparent",
                  border: `1px solid ${selectedAct === a.id ? a.color + "44" : "#1a1815"}`,
                  color: selectedAct === a.id ? a.color : "#4a4238",
                  padding: "5px 12px", borderRadius: 3,
                  cursor: "pointer", fontSize: 10, fontFamily: "inherit",
                }}
              >{a.label}</button>
            ))}
          </div>
        </div>

        {/* Boss Path Filter */}
        {["1", "2", "3"].map(actId => {
          const themes = BOSS_THEMES[actId];
          if (!themes) return null;
          if (selectedAct && selectedAct !== actId && selectedAct !== "1.5") return null;
          // Also show act 3 bosses when viewing act 0 or 1.5 (traitor foreshadow)
          if (actId === "3" && selectedAct && !["0", "1", "1.5", "3"].includes(selectedAct)) return null;
          const actLabel = actId === "1" ? "Act 1 Boss" : actId === "2" ? "Act 2 Boss" : "Act 3 Traitor";

          return (
            <div key={actId} style={{ marginBottom: 8 }}>
              <div style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 2, marginBottom: 4 }}>
                {actLabel}
              </div>
              <div style={{ display: "flex", gap: 4, flexWrap: "wrap" }}>
                <button
                  onClick={() => setSelectedBoss(prev => ({ ...prev, [actId]: null, ...(actId === "3" ? { "0": null, "1": null, "1.5": null } : {}) }))}
                  style={{
                    background: !selectedBoss[actId] ? "#1a1815" : "transparent",
                    border: `1px solid ${!selectedBoss[actId] ? "#2a2520" : "#1a1815"}`,
                    color: !selectedBoss[actId] ? "#d4a574" : "#4a4238",
                    padding: "4px 10px", borderRadius: 3,
                    cursor: "pointer", fontSize: 10, fontFamily: "inherit",
                  }}
                >ALL</button>
                {themes.map(t => (
                  <button
                    key={t.id}
                    onClick={() => setSelectedBoss(prev => ({
                      ...prev,
                      [actId]: t.id,
                      // Traitor selection also filters Act 0, 1, and 1.5
                      ...(actId === "3" ? { "0": t.id, "1": t.id, "1.5": t.id } : {}),
                    }))}
                    style={{
                      background: selectedBoss[actId] === t.id ? "#1a1815" : "transparent",
                      border: `1px solid ${selectedBoss[actId] === t.id ? "#2a2520" : "#1a1815"}`,
                      color: selectedBoss[actId] === t.id ? ACTS.find(a => a.id === actId)?.color : "#4a4238",
                      padding: "4px 10px", borderRadius: 3,
                      cursor: "pointer", fontSize: 10, fontFamily: "inherit",
                    }}
                  >{t.icon} {t.label}</button>
                ))}
              </div>
            </div>
          );
        })}

        {/* Quest Count */}
        <div style={{
          fontSize: 10, color: "#4a4238", marginTop: 12, marginBottom: 16,
          borderTop: "1px solid #1a1815", paddingTop: 8,
        }}>
          Showing {quests.length} quests
        </div>

        {/* Quest List grouped by Act */}
        {ACTS.map(act => {
          const actQuests = quests.filter(q => q.act === act.id);
          if (actQuests.length === 0) return null;

          return (
            <div key={act.id} style={{ marginBottom: 24 }}>
              <div style={{
                fontSize: 13, color: act.color, fontWeight: 600,
                borderBottom: `1px solid ${act.color}33`,
                paddingBottom: 6, marginBottom: 10,
                display: "flex", alignItems: "center", gap: 8,
              }}>
                <span style={{
                  width: 8, height: 8, borderRadius: "50%",
                  background: act.color,
                }} />
                {act.label}
                <span style={{ fontSize: 10, color: "#4a4238", fontWeight: 400 }}>
                  ({actQuests.length} quests)
                </span>
              </div>
              {actQuests.map((q, i) => (
                <QuestCard key={`${q.act}-${q.sub}-${q.boss}-${i}`} q={q} actColor={act.color} />
              ))}
            </div>
          );
        })}

        {/* Footer */}
        <div style={{
          marginTop: 24, padding: 14,
          background: "#0f0e0c", border: "1px solid #1a1815",
          borderRadius: 4, textAlign: "center",
        }}>
          <div style={{ fontSize: 11, color: "#5a5248", lineHeight: 1.8 }}>
            One full playthrough sees: 3 universal + 4 traitor-specific + 3 Act 1 boss + 3 Act 2 boss + 3 Act 3 traitor + 3 Act 4 = ~19 main quests
          </div>
          <div style={{ fontSize: 10, color: "#3a3530", marginTop: 4 }}>
            Total unique quests across all paths: {stats.total} \u2022 Replay value: 4\u00D74\u00D74 = 64 possible boss combinations
          </div>
        </div>
      </div>
    </div>
  );
}
