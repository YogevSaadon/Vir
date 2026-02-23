//! JSON data loader — reads /data/ files into engine structs

use std::collections::HashMap;
use std::path::Path;

use vassnian_engine::character::stats::{StatBlock, DerivedStatFormulas};
use vassnian_engine::character::entity::{Entity, EntityId, EntityKind, AttackType, FormationRow};
use vassnian_engine::character::leveling::LevelConfig;
use vassnian_engine::combat::scaling::ScalingConfig;
use vassnian_engine::inventory::items::{PotionDef, EquipmentDef, ShopDef};
use vassnian_engine::loot::{LootTable, LootTablesData};
use vassnian_engine::story::engine::StoryDef;
use vassnian_engine::world::mission::MissionDef;

use crate::classes::{ClassDef, ClassesData};
use crate::skills::{CombatSkillDef, CombatSkillsData};
use crate::world_skills::{WorldSkillDef, WorldSkillsData};

/// All game data loaded from JSON files.
#[derive(Debug, Clone)]
pub struct GameData {
    pub classes: Vec<ClassDef>,
    pub combat_skills: Vec<CombatSkillDef>,
    pub world_skills: Vec<WorldSkillDef>,
    pub potions: Vec<PotionDef>,
    pub equipment: Vec<EquipmentDef>,
    pub shops: HashMap<String, ShopDef>,
    pub stories: Vec<StoryDef>,
    pub companions: Vec<CompanionDef>,
    pub enemy_groups: HashMap<String, EnemyGroupDef>,
    pub missions: Vec<MissionDef>,
    pub config: GameConfig,
    pub stat_formulas: DerivedStatFormulas,
    pub level_config: LevelConfig,
    pub scaling_config: ScalingConfig,
    pub loot_tables: HashMap<String, LootTable>,
}

/// Companion definition loaded from JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompanionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub class: String,
    pub portrait: String,
    pub stats: StatBlock,
    #[serde(default)]
    pub combat_skills: Vec<String>,
    #[serde(default)]
    pub world_skills: Vec<String>,
    pub ai_type: String,
    pub join_story: String,
    pub join_node: String,
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub magic_resist: i32,
}

/// Container for companions JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompanionsData {
    pub companions: Vec<CompanionDef>,
}

/// Enemy definition within a group.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnemyDef {
    pub id: String,
    pub name: String,
    pub portrait: String,
    pub stats: StatBlock,
    pub attack_type: AttackType,
    #[serde(default)]
    pub combat_skills: Vec<String>,
    pub ai_type: String,
    #[serde(default)]
    pub exp_reward: i32,
    #[serde(default)]
    pub gold_reward: i32,
    pub position: String,
    #[serde(default = "default_enemy_level")]
    pub level: i32,
    #[serde(default = "default_enemy_types")]
    pub enemy_types: Vec<String>,
    #[serde(default = "default_phase_range")]
    pub phase_range: [i32; 2],
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub magic_resist: i32,
}

fn default_enemy_level() -> i32 { 1 }
fn default_enemy_types() -> Vec<String> { vec!["any".to_string()] }
fn default_phase_range() -> [i32; 2] { [0, 0] }

/// Group of enemies for a combat encounter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnemyGroupDef {
    pub enemies: Vec<EnemyDef>,
    #[serde(default)]
    pub loot_table: Option<String>,
}

/// Container for enemy groups JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnemyGroupsData {
    pub enemy_groups: HashMap<String, EnemyGroupDef>,
}

/// Game configuration loaded from JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    pub game_title: String,
    pub hero_title: String,
    pub atb_base_duration: f32,
    pub atb_speed_modifier: f32,
    pub potion_belt_max: usize,
    pub stat_base_value: i32,
    pub free_stat_points: i32,
    pub world_skills_at_creation: usize,
    pub starting_gold: i32,
    pub max_party_size: usize,
    pub skill_bar_slots: usize,
    #[serde(default)]
    pub mvp_mode: bool,
}

/// Container for potions JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PotionsData {
    pub potions: Vec<PotionDef>,
}

/// Container for equipment JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquipmentData {
    pub equipment: Vec<EquipmentDef>,
}

/// Container for shop inventories JSON.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShopsData {
    pub shops: HashMap<String, ShopDef>,
}

/// Loads all game data from the given data directory.
pub fn load_all_data(data_dir: &Path) -> Result<GameData, String> {
    let classes = load_json::<ClassesData>(
        &data_dir.join("characters/classes.json")
    )?.classes;

    let combat_skills = load_json::<CombatSkillsData>(
        &data_dir.join("skills/combat_skills.json")
    )?.combat_skills;

    let world_skills = load_json::<WorldSkillsData>(
        &data_dir.join("skills/world_skills.json")
    )?.world_skills;

    let potions = load_json::<PotionsData>(
        &data_dir.join("items/potions.json")
    )?.potions;

    let equipment = load_json::<EquipmentData>(
        &data_dir.join("items/equipment.json")
    )?.equipment;

    let shops = load_json::<ShopsData>(
        &data_dir.join("items/shop_inventories.json")
    )?.shops;

    let companions = load_json::<CompanionsData>(
        &data_dir.join("characters/companions.json")
    )?.companions;

    let enemy_groups = load_json::<EnemyGroupsData>(
        &data_dir.join("characters/enemies.json")
    )?.enemy_groups;

    let config = load_json::<GameConfig>(
        &data_dir.join("config/game_config.json")
    )?;

    let stat_formulas = load_json::<DerivedStatFormulas>(
        &data_dir.join("config/stat_formulas.json")
    )?;

    let level_config = load_json::<LevelConfig>(
        &data_dir.join("config/level_config.json")
    )?;

    let scaling_config = load_json::<ScalingConfig>(
        &data_dir.join("config/scaling_config.json")
    )?;

    let loot_tables_data = load_json::<LootTablesData>(
        &data_dir.join("config/loot_tables.json")
    )?;
    let loot_tables: HashMap<String, LootTable> = loot_tables_data.tables
        .into_iter()
        .map(|(k, v)| { let id = k.clone(); (k, v.with_id(id)) })
        .collect();

    // Load all story files
    let stories = load_stories(&data_dir.join("stories"))?;

    let missions = load_json::<Vec<MissionDef>>(
        &data_dir.join("missions/mission_catalog.json")
    )?;

    Ok(GameData {
        classes,
        combat_skills,
        world_skills,
        potions,
        equipment,
        shops,
        stories,
        companions,
        enemy_groups,
        missions,
        config,
        stat_formulas,
        level_config,
        scaling_config,
        loot_tables,
    })
}

/// Loads a single JSON file into a typed struct.
fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

/// Loads all story JSON files from a directory.
fn load_stories(dir: &Path) -> Result<Vec<StoryDef>, String> {
    let mut stories = Vec::new();

    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read stories dir: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "json") {
            let story: StoryDef = load_json(&path)?;
            stories.push(story);
        }
    }

    Ok(stories)
}

/// Parses a JSON string into a typed struct (for embedded data).
fn parse_json<T: serde::de::DeserializeOwned>(json: &str, label: &str) -> Result<T, String> {
    serde_json::from_str(json)
        .map_err(|e| format!("Failed to parse {}: {}", label, e))
}

/// Loads all game data from embedded strings (for WASM builds where filesystem is unavailable).
pub fn load_embedded_data() -> Result<GameData, String> {
    let classes = parse_json::<ClassesData>(
        include_str!("../../data/characters/classes.json"), "classes"
    )?.classes;

    let combat_skills = parse_json::<CombatSkillsData>(
        include_str!("../../data/skills/combat_skills.json"), "combat_skills"
    )?.combat_skills;

    let world_skills = parse_json::<WorldSkillsData>(
        include_str!("../../data/skills/world_skills.json"), "world_skills"
    )?.world_skills;

    let potions = parse_json::<PotionsData>(
        include_str!("../../data/items/potions.json"), "potions"
    )?.potions;

    let equipment = parse_json::<EquipmentData>(
        include_str!("../../data/items/equipment.json"), "equipment"
    )?.equipment;

    let shops = parse_json::<ShopsData>(
        include_str!("../../data/items/shop_inventories.json"), "shops"
    )?.shops;

    let companions = parse_json::<CompanionsData>(
        include_str!("../../data/characters/companions.json"), "companions"
    )?.companions;

    let enemy_groups = parse_json::<EnemyGroupsData>(
        include_str!("../../data/characters/enemies.json"), "enemies"
    )?.enemy_groups;

    let config = parse_json::<GameConfig>(
        include_str!("../../data/config/game_config.json"), "config"
    )?;

    let stat_formulas = parse_json::<DerivedStatFormulas>(
        include_str!("../../data/config/stat_formulas.json"), "stat_formulas"
    )?;

    let level_config = parse_json::<LevelConfig>(
        include_str!("../../data/config/level_config.json"), "level_config"
    )?;

    let scaling_config = parse_json::<ScalingConfig>(
        include_str!("../../data/config/scaling_config.json"), "scaling_config"
    )?;

    let loot_tables_data = parse_json::<LootTablesData>(
        include_str!("../../data/config/loot_tables.json"), "loot_tables"
    )?;
    let loot_tables: HashMap<String, LootTable> = loot_tables_data.tables
        .into_iter()
        .map(|(k, v)| { let id = k.clone(); (k, v.with_id(id)) })
        .collect();

    // Embedded stories
    let mut stories = Vec::new();
    for (json, label) in [
        (include_str!("../../data/stories/intro_god.json"), "intro_god"),
        (include_str!("../../data/stories/mvp_story_01.json"), "mvp_story_01"),
        (include_str!("../../data/stories/mvp_story_02_shop.json"), "mvp_story_02_shop"),
    ] {
        stories.push(parse_json::<StoryDef>(json, label)?);
    }

    let missions = parse_json::<Vec<MissionDef>>(
        include_str!("../../data/missions/mission_catalog.json"), "missions"
    )?;

    Ok(GameData {
        classes,
        combat_skills,
        world_skills,
        potions,
        equipment,
        shops,
        stories,
        companions,
        enemy_groups,
        missions,
        config,
        stat_formulas,
        level_config,
        scaling_config,
        loot_tables,
    })
}

/// Creates an Entity from a CompanionDef.
pub fn companion_to_entity(comp: &CompanionDef, id: EntityId) -> Entity {
    let mut entity = Entity::new(
        id,
        comp.name.clone(),
        EntityKind::Companion,
        comp.class.clone(),
        comp.stats.clone(),
    );
    entity.portrait = comp.portrait.clone();
    entity.combat_skills = comp.combat_skills.clone();
    entity.world_skills = comp.world_skills.clone();
    entity.ai_type = comp.ai_type.clone();
    entity.armor = comp.armor;
    entity.magic_resist = comp.magic_resist;
    entity
}

/// Creates an Entity from an EnemyDef.
pub fn enemy_to_entity(enemy: &EnemyDef, id: EntityId) -> Entity {
    let mut entity = Entity::new(
        id,
        enemy.name.clone(),
        EntityKind::Enemy,
        String::new(),
        enemy.stats.clone(),
    );
    entity.portrait = enemy.portrait.clone();
    entity.attack_type = enemy.attack_type.clone();
    entity.combat_skills = enemy.combat_skills.clone();
    entity.ai_type = enemy.ai_type.clone();
    entity.armor = enemy.armor;
    entity.magic_resist = enemy.magic_resist;
    entity.position = match enemy.position.as_str() {
        "back" => FormationRow::Back,
        _ => FormationRow::Front,
    };
    entity
}
