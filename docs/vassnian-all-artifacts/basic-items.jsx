import { useState } from "react";

/*
  BASIC ITEMS CATALOG
  Non-unique equipment available in shops and drops.
  Organized by act tier and item type.
  Prices and stats based on vassnian-numbers.md power bands.
  
  WEAPON BASE ranges per act: Act 1 (1-3), Act 2 (4-6), Act 3 (7-9), Act 4 (10-12)
  ARMOR ranges per act: Act 1 (1-4), Act 2 (4-8), Act 3 (8-14), Act 4 (14-20)
  SHIELD flat reduction: Basic (1), Iron (2), Tower (3), Endgame (5)
*/

const ITEM_DATA = {
  weapons: {
    label: "Weapons",
    color: "#bf6a6a",
    note: "Damage = Weapon Base + Stat. Without matching passive, no stat bonus.",
    items: [
      // ACT 1
      { name: "Rusty Sword", type: "1H Sword", stat: "STR", base: 1, act: 1, price: 10, note: "Starter weapon" },
      { name: "Short Sword", type: "1H Sword", stat: "STR", base: 2, act: 1, price: 30, note: "Standard Act 1" },
      { name: "Fine Blade", type: "1H Sword", stat: "STR", base: 3, act: 1, price: 60, note: "Act 1 best shop" },
      { name: "Worn Dagger", type: "1H Dagger", stat: "DEX", base: 1, act: 1, price: 10, note: "Starter" },
      { name: "Steel Dagger", type: "1H Dagger", stat: "DEX", base: 2, act: 1, price: 25, note: "" },
      { name: "Assassin's Knife", type: "1H Dagger", stat: "DEX", base: 3, act: 1, price: 55, note: "Act 1 best" },
      { name: "Hunting Bow", type: "Bow", stat: "DEX", base: 1, act: 1, price: 15, note: "Starter ranged" },
      { name: "Longbow", type: "Bow", stat: "DEX", base: 2, act: 1, price: 35, note: "" },
      { name: "Composite Bow", type: "Bow", stat: "DEX", base: 3, act: 1, price: 65, note: "Act 1 best ranged" },
      { name: "Gnarled Staff", type: "Staff", stat: "INT", base: 1, act: 1, price: 10, note: "Starter caster" },
      { name: "Oak Staff", type: "Staff", stat: "INT", base: 2, act: 1, price: 30, note: "" },
      { name: "Arcane Staff", type: "Staff", stat: "INT", base: 3, act: 1, price: 60, note: "Act 1 best caster" },
      { name: "Woodcutter's Axe", type: "2H Axe", stat: "STR", base: 2, act: 1, price: 20, note: "Starter 2H" },
      { name: "Battle Axe", type: "2H Axe", stat: "STR", base: 3, act: 1, price: 50, note: "" },
      { name: "Iron Greatsword", type: "2H Sword", stat: "STR", base: 3, act: 1, price: 55, note: "Act 1 best 2H" },
      { name: "Wooden Mace", type: "1H Mace", stat: "STR", base: 1, act: 1, price: 10, note: "Priest starter" },
      { name: "Iron Mace", type: "1H Mace", stat: "STR", base: 2, act: 1, price: 30, note: "" },
      // ACT 2
      { name: "War Sword", type: "1H Sword", stat: "STR", base: 4, act: 2, price: 120, note: "Act 2 entry" },
      { name: "Enchanted Blade", type: "1H Sword", stat: "STR", base: 5, act: 2, price: 200, note: "Mid Act 2" },
      { name: "Shadow Steel Sword", type: "1H Sword", stat: "STR", base: 6, act: 2, price: 320, note: "Act 2 best" },
      { name: "Shadow Dagger", type: "1H Dagger", stat: "DEX", base: 4, act: 2, price: 110, note: "" },
      { name: "Deepkin Fang", type: "1H Dagger", stat: "DEX", base: 6, act: 2, price: 300, note: "Act 2 best" },
      { name: "Crystal Bow", type: "Bow", stat: "DEX", base: 5, act: 2, price: 220, note: "Shadow World craft" },
      { name: "Glowstone Staff", type: "Staff", stat: "INT", base: 5, act: 2, price: 210, note: "" },
      { name: "Deepkin Crystal Staff", type: "Staff", stat: "INT", base: 6, act: 2, price: 330, note: "Act 2 best" },
      { name: "Shadow Greataxe", type: "2H Axe", stat: "STR", base: 6, act: 2, price: 310, note: "" },
      // ACT 3
      { name: "Caelmund Steel Sword", type: "1H Sword", stat: "STR", base: 7, act: 3, price: 500, note: "Act 3 entry" },
      { name: "Blessed Blade", type: "1H Sword", stat: "STR", base: 8, act: 3, price: 750, note: "" },
      { name: "Godsteel Sword", type: "1H Sword", stat: "STR", base: 9, act: 3, price: 1100, note: "Act 3 best" },
      { name: "Night Fang", type: "1H Dagger", stat: "DEX", base: 8, act: 3, price: 700, note: "" },
      { name: "Warden's Bow", type: "Bow", stat: "DEX", base: 8, act: 3, price: 720, note: "" },
      { name: "Sealkeeper's Staff", type: "Staff", stat: "INT", base: 9, act: 3, price: 1050, note: "Act 3 best caster" },
      // ACT 4
      { name: "Legendary Blade", type: "1H Sword", stat: "STR", base: 10, act: 4, price: 2000, note: "Act 4 entry" },
      { name: "Artifact Sword", type: "1H Sword", stat: "STR", base: 11, act: 4, price: 3500, note: "" },
      { name: "Final Weapon", type: "1H Sword", stat: "STR", base: 12, act: 4, price: 5000, note: "Act 4 best (shop)" },
    ],
  },
  armor: {
    label: "Armor",
    color: "#6a8abf",
    note: "Armor value \u2192 Damage Reduction = Armor/(Armor+10). Light for DEX classes, Heavy for STR.",
    items: [
      // ACT 1
      { name: "Cloth Robes", type: "Light", armor: 1, act: 1, price: 15, note: "Mage/Priest/Bard starter" },
      { name: "Leather Armor", type: "Medium", armor: 2, act: 1, price: 30, note: "Ranger/Rogue" },
      { name: "Chain Mail", type: "Heavy", armor: 3, act: 1, price: 50, note: "Knight/Warrior/Paladin" },
      { name: "Iron Plate", type: "Heavy", armor: 4, act: 1, price: 80, note: "Act 1 best heavy" },
      // ACT 2
      { name: "Shadow Silk Robes", type: "Light", armor: 4, act: 2, price: 140, note: "" },
      { name: "Deepkin Hide", type: "Medium", armor: 6, act: 2, price: 250, note: "Shadow World material" },
      { name: "Crystal Plate", type: "Heavy", armor: 8, act: 2, price: 400, note: "Act 2 best" },
      // ACT 3
      { name: "Warded Robes", type: "Light", armor: 8, act: 3, price: 600, note: "" },
      { name: "Reinforced Leather", type: "Medium", armor: 10, act: 3, price: 800, note: "" },
      { name: "Blessed Plate", type: "Heavy", armor: 14, act: 3, price: 1200, note: "Act 3 best" },
      // ACT 4
      { name: "Archmage's Robes", type: "Light", armor: 14, act: 4, price: 2200, note: "" },
      { name: "Dragon Leather", type: "Medium", armor: 16, act: 4, price: 3000, note: "" },
      { name: "Godsent Plate", type: "Heavy", armor: 20, act: 4, price: 5000, note: "Act 4 best" },
    ],
  },
  shields: {
    label: "Shields",
    color: "#bfa86a",
    note: "Knight exclusive passive. Flat reduction BEFORE armor calculation.",
    items: [
      { name: "Wooden Shield", type: "Shield", armor: 1, act: 1, price: 20, note: "Starter" },
      { name: "Iron Shield", type: "Shield", armor: 2, act: 1, price: 50, note: "" },
      { name: "Tower Shield", type: "Shield", armor: 3, act: 2, price: 200, note: "" },
      { name: "Crystal Shield", type: "Shield", armor: 4, act: 3, price: 800, note: "" },
      { name: "Godsent Shield", type: "Shield", armor: 5, act: 4, price: 3000, note: "Endgame" },
    ],
  },
  accessories: {
    label: "Accessories",
    color: "#8a6abf",
    note: "Rings, amulets, cloaks. Stat bonuses, MR, special effects.",
    items: [
      { name: "Copper Ring", type: "Ring", armor: 0, act: 1, price: 25, note: "+1 to a stat" },
      { name: "Silver Ring", type: "Ring", armor: 0, act: 2, price: 150, note: "+2 to a stat" },
      { name: "Gold Ring", type: "Ring", armor: 0, act: 3, price: 600, note: "+3 to a stat" },
      { name: "Traveler's Cloak", type: "Cloak", armor: 0, act: 1, price: 40, note: "Minor evasion help" },
      { name: "Ward Amulet", type: "Amulet", armor: 0, act: 2, price: 200, note: "+2 Magic Resistance" },
      { name: "Sealstone Amulet", type: "Amulet", armor: 0, act: 3, price: 700, note: "+4 Magic Resistance" },
    ],
  },
  potions: {
    label: "Potions",
    color: "#6abf6a",
    note: "4-slot potion belt. Used in combat. Potions consumed on use.",
    items: [
      { name: "Minor Health Potion", type: "HP", armor: 0, act: 1, price: 5, note: "Restores ~10 HP" },
      { name: "Health Potion", type: "HP", armor: 0, act: 2, price: 20, note: "Restores ~30 HP" },
      { name: "Greater Health Potion", type: "HP", armor: 0, act: 3, price: 60, note: "Restores ~80 HP" },
      { name: "Supreme Health Potion", type: "HP", armor: 0, act: 4, price: 150, note: "Restores ~150 HP" },
      { name: "Minor Mana Potion", type: "Mana", armor: 0, act: 1, price: 8, note: "Restores ~3 Mana" },
      { name: "Mana Potion", type: "Mana", armor: 0, act: 2, price: 25, note: "Restores ~6 Mana" },
      { name: "Greater Mana Potion", type: "Mana", armor: 0, act: 3, price: 70, note: "Restores ~10 Mana" },
      { name: "Antidote", type: "Cure", armor: 0, act: 1, price: 10, note: "Cures poison" },
      { name: "Holy Water", type: "Cure", armor: 0, act: 2, price: 30, note: "Cures curse" },
      { name: "Buff Elixir (STR)", type: "Buff", armor: 0, act: 2, price: 40, note: "+2 STR for 1 fight" },
      { name: "Buff Elixir (DEX)", type: "Buff", armor: 0, act: 2, price: 40, note: "+2 DEX for 1 fight" },
      { name: "Buff Elixir (INT)", type: "Buff", armor: 0, act: 2, price: 40, note: "+2 INT for 1 fight" },
    ],
  },
};

const ACT_COLORS = { 1: "#bfa86a", 2: "#6a6abf", 3: "#bf6a6a", 4: "#d4a574" };

export default function BasicItems() {
  const [category, setCategory] = useState("weapons");
  const [filterAct, setFilterAct] = useState("all");

  const cat = ITEM_DATA[category];
  const filtered = filterAct === "all" ? cat.items : cat.items.filter(i => i.act === parseInt(filterAct));

  return (
    <div style={{
      minHeight: "100vh", background: "#0a0a0f", color: "#c8c0b8",
      fontFamily: "'Palatino Linotype', 'Book Antiqua', Palatino, serif",
      padding: "24px",
    }}>
      <div style={{ maxWidth: 900, margin: "0 auto" }}>
        <div style={{ textAlign: "center", marginBottom: 20, borderBottom: "1px solid #2a2520", paddingBottom: 16 }}>
          <h1 style={{ fontSize: 22, color: "#6a8abf", fontWeight: 400, letterSpacing: 4, textTransform: "uppercase", margin: 0 }}>
            \u{1F6E1} Basic Items Catalog \u{1F6E1}
          </h1>
          <p style={{ color: "#6a6058", fontSize: 11, marginTop: 6 }}>
            Shop &amp; drop equipment by act \u2022 Baseline for unique item comparison
          </p>
        </div>

        {/* Category Tabs */}
        <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginBottom: 8 }}>
          {Object.entries(ITEM_DATA).map(([key, val]) => (
            <button key={key} onClick={() => setCategory(key)} style={{
              background: category === key ? "#1a1815" : "transparent",
              border: `1px solid ${category === key ? val.color + "44" : "#1a1815"}`,
              color: category === key ? val.color : "#4a4238",
              padding: "6px 14px", borderRadius: 3, cursor: "pointer",
              fontSize: 11, fontFamily: "inherit",
            }}>{val.label} ({val.items.length})</button>
          ))}
        </div>

        {/* Act Filter */}
        <div style={{ display: "flex", gap: 4, flexWrap: "wrap", marginBottom: 12 }}>
          {["all", "1", "2", "3", "4"].map(a => (
            <button key={a} onClick={() => setFilterAct(a)} style={{
              background: filterAct === a ? "#1a1815" : "transparent",
              border: `1px solid ${filterAct === a ? "#2a2520" : "#1a1815"}`,
              color: filterAct === a ? "#d4a574" : "#4a4238",
              padding: "4px 10px", borderRadius: 3, cursor: "pointer",
              fontSize: 10, fontFamily: "inherit",
            }}>{a === "all" ? "All" : `Act ${a}`}</button>
          ))}
        </div>

        {/* Note */}
        <div style={{
          fontSize: 10, color: "#6a6058", fontStyle: "italic",
          marginBottom: 12, padding: "8px 12px",
          background: "#0c0c10", borderRadius: 4,
        }}>{cat.note}</div>

        {/* Item Table */}
        <div style={{ overflowX: "auto" }}>
          <table style={{ width: "100%", borderCollapse: "collapse", fontSize: 11 }}>
            <thead>
              <tr>
                {["Name", "Type", category === "weapons" ? "Stat" : "", category === "weapons" ? "Base DMG" : "Armor", "Act", "Price", "Note"].filter(Boolean).map((h, i) => (
                  <th key={i} style={{
                    background: "#1a1815", color: cat.color,
                    padding: "8px 10px", textAlign: "left",
                    borderBottom: "1px solid #2a2520", fontWeight: 600,
                  }}>{h}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {filtered.map((item, i) => (
                <tr key={i} style={{ background: i % 2 === 0 ? "#0a0a0f" : "#0f0e0c" }}>
                  <td style={{ padding: "8px 10px", color: "#d4c4b0", fontWeight: 600, borderBottom: "1px solid #1a1815" }}>{item.name}</td>
                  <td style={{ padding: "8px 10px", color: "#8a8078", borderBottom: "1px solid #1a1815" }}>{item.type}</td>
                  {category === "weapons" && <td style={{ padding: "8px 10px", color: "#6a8abf", borderBottom: "1px solid #1a1815" }}>{item.stat}</td>}
                  <td style={{ padding: "8px 10px", color: cat.color, fontWeight: 600, borderBottom: "1px solid #1a1815" }}>
                    {category === "weapons" ? item.base : (item.armor || "\u2014")}
                  </td>
                  <td style={{ padding: "8px 10px", borderBottom: "1px solid #1a1815" }}>
                    <span style={{ color: ACT_COLORS[item.act], fontSize: 10 }}>Act {item.act}</span>
                  </td>
                  <td style={{ padding: "8px 10px", color: "#bfa86a", borderBottom: "1px solid #1a1815" }}>{item.price}g</td>
                  <td style={{ padding: "8px 10px", color: "#6a6058", borderBottom: "1px solid #1a1815", fontStyle: "italic" }}>{item.note}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {/* Price Curve Summary */}
        <div style={{
          marginTop: 20, padding: 14,
          background: "#0f0e0c", border: "1px solid #1a1815",
          borderRadius: 4,
        }}>
          <div style={{ fontSize: 10, color: "#5a5248", textTransform: "uppercase", letterSpacing: 2, marginBottom: 6 }}>
            Price Curve (approximate)
          </div>
          <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr 1fr", gap: 8, fontSize: 10 }}>
            {[
              { act: 1, weapon: "10-80g", armor: "15-80g", potion: "5-10g" },
              { act: 2, weapon: "110-330g", armor: "140-400g", potion: "20-40g" },
              { act: 3, weapon: "500-1100g", armor: "600-1200g", potion: "60-70g" },
              { act: 4, weapon: "2000-5000g", armor: "2200-5000g", potion: "150g" },
            ].map((a, i) => (
              <div key={i} style={{
                background: "#0a0a0f", border: `1px solid ${ACT_COLORS[a.act]}33`,
                borderRadius: 3, padding: "8px 10px",
              }}>
                <div style={{ color: ACT_COLORS[a.act], fontWeight: 600, marginBottom: 4 }}>Act {a.act}</div>
                <div style={{ color: "#8a8078" }}>Weapons: {a.weapon}</div>
                <div style={{ color: "#8a8078" }}>Armor: {a.armor}</div>
                <div style={{ color: "#8a8078" }}>Potions: {a.potion}</div>
              </div>
            ))}
          </div>
        </div>

        <div style={{ textAlign: "center", marginTop: 16, fontSize: 10, color: "#3a3530" }}>
          All prices are draft \u2022 Balance through playtesting \u2022 Unique items tracked in unique-items.jsx
        </div>
      </div>
    </div>
  );
}
