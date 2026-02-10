//! Game state manager — handles screen transitions and global state.

use std::path::PathBuf;
use std::collections::HashMap;
use macroquad::prelude::*;

use vassnian_engine::character::entity::{Entity, EntityKind, FormationRow, AttackType};
use vassnian_engine::character::stats::{StatBlock, STAT_BASE_VALUE, FREE_STAT_POINTS};
use vassnian_engine::character::equipment::EquipmentSlots;
use vassnian_engine::inventory::belt::PotionBelt;
use vassnian_engine::inventory::items::{PotionStack, EquipmentInstance};
use vassnian_engine::story::engine::{StoryDef, StoryState};
use vassnian_engine::story::tags::TagSystem;
use vassnian_engine::combat::battle::BattleState;
use vassnian_engine::save::{SaveData, PlayerSaveData, InventorySaveData, WorldSaveData};

use vassnian_content::loader::{GameData, load_all_data, companion_to_entity, enemy_to_entity};

use crate::screens::*;
use crate::platform;

/// Current game screen.
#[derive(Debug, Clone, PartialEq)]
pub enum GameScreen {
    MainMenu,
    GodIntro,
    CharacterCreation,
    Story,
    Combat,
    Shop,
    Inventory,
    Stats,
    Options,
    InGameOptions,
    GameOver,
}

/// Character creation sub-step.
#[derive(Debug, Clone, PartialEq)]
pub enum CreationStep {
    AvatarSelect,
    ClassSelect,
    StatPointBuy,
    WorldSkillSelect,
}

/// Text display speed setting.
#[derive(Debug, Clone, PartialEq)]
pub enum TextSpeed {
    Instant,
    Medium,
    Slow,
}

/// Top-level application state holding everything.
pub struct App {
    // -- Screen state --
    pub screen: GameScreen,
    pub prev_screen: Option<GameScreen>,

    // -- Game data (loaded from JSON) --
    pub data: Option<GameData>,
    pub data_error: Option<String>,

    // -- Player state --
    pub player: Option<Entity>,
    pub player_avatar: String,
    pub companions: Vec<Entity>,
    pub equipment: EquipmentSlots,
    pub gold: i32,
    pub potions: Vec<PotionStack>,
    pub owned_equipment: Vec<EquipmentInstance>,
    pub potion_belt: PotionBelt,
    pub tags: TagSystem,
    pub completed_stories: Vec<String>,
    pub available_stories: Vec<String>,

    // -- Character creation state --
    pub creation_step: CreationStep,
    pub creation_avatar_idx: usize,
    pub creation_stats: StatBlock,
    pub creation_points_left: i32,
    pub creation_world_skill: Option<String>,

    // -- Story state --
    pub current_story: Option<StoryDef>,
    pub story_state: Option<StoryState>,
    pub story_text_progress: f32,
    pub story_text_speed: f32,
    pub story_page: usize,

    // -- Combat state --
    pub battle: Option<BattleState>,
    pub combat_on_win: Option<String>,
    pub combat_on_lose: Option<String>,
    pub combat_target_select: bool,
    pub combat_selected_target: Option<u32>,
    pub player_atb_ready: bool,

    // -- Shop state --
    pub current_shop_id: Option<String>,
    pub shop_stock: HashMap<String, i32>,

    // -- Settings --
    pub text_speed: TextSpeed,
    pub music_volume: f32,
    pub sfx_volume: f32,

    // -- Entity ID counter --
    pub next_entity_id: u32,
}

impl App {
    /// Creates a new App and loads game data.
    pub fn new() -> Self {
        let data_dir = PathBuf::from("data");
        let (data, data_error) = match load_all_data(&data_dir) {
            Ok(d) => (Some(d), None),
            Err(e) => (None, Some(e)),
        };

        Self {
            screen: GameScreen::MainMenu,
            prev_screen: None,
            data,
            data_error,
            player: None,
            player_avatar: String::new(),
            companions: Vec::new(),
            equipment: EquipmentSlots::default(),
            gold: 50,
            potions: Vec::new(),
            owned_equipment: Vec::new(),
            potion_belt: PotionBelt::new(),
            tags: TagSystem::new(),
            completed_stories: Vec::new(),
            available_stories: vec!["intro_god".to_string()],
            creation_step: CreationStep::AvatarSelect,
            creation_avatar_idx: 0,
            creation_stats: StatBlock::all(STAT_BASE_VALUE),
            creation_points_left: FREE_STAT_POINTS,
            creation_world_skill: None,
            current_story: None,
            story_state: None,
            story_text_progress: 0.0,
            story_text_speed: 0.0,
            story_page: 0,
            battle: None,
            combat_on_win: None,
            combat_on_lose: None,
            combat_target_select: false,
            combat_selected_target: None,
            player_atb_ready: false,
            current_shop_id: None,
            shop_stock: HashMap::new(),
            text_speed: TextSpeed::Instant,
            music_volume: 0.7,
            sfx_volume: 1.0,
            next_entity_id: 1,
        }
    }

    /// Generates a unique entity ID.
    pub fn next_id(&mut self) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    /// Updates game logic for the current frame.
    pub fn update(&mut self) {
        match self.screen {
            GameScreen::MainMenu => main_menu::update(self),
            GameScreen::GodIntro => story::update(self),
            GameScreen::CharacterCreation => character_creation::update(self),
            GameScreen::Story => story::update(self),
            GameScreen::Combat => combat::update(self),
            GameScreen::Shop => shop::update(self),
            GameScreen::Inventory => inventory::update(self),
            GameScreen::Stats => {},
            GameScreen::Options => options::update(self),
            GameScreen::InGameOptions => options::update(self),
            GameScreen::GameOver => game_over::update(self),
        }
    }

    /// Draws the current frame.
    pub fn draw(&mut self) {
        clear_background(crate::rendering::BG_COLOR);

        // Show data load error if any
        if let Some(ref err) = self.data_error {
            draw_text(
                &format!("Data Error: {}", err),
                10.0, 30.0, 16.0,
                Color::from_rgba(255, 100, 100, 255),
            );
            return;
        }

        let screen = self.screen.clone();
        match screen {
            GameScreen::MainMenu => main_menu::draw(self),
            GameScreen::GodIntro => story::draw(self),
            GameScreen::CharacterCreation => character_creation::draw(self),
            GameScreen::Story => story::draw(self),
            GameScreen::Combat => combat::draw(self),
            GameScreen::Shop => shop::draw(self),
            GameScreen::Inventory => inventory::draw(self),
            GameScreen::Stats => stats::draw(self),
            GameScreen::Options => options::draw(self),
            GameScreen::InGameOptions => options::draw_in_game(self),
            GameScreen::GameOver => game_over::draw(self),
        }
    }

    /// Transitions to a new screen.
    pub fn go_to(&mut self, screen: GameScreen) {
        self.prev_screen = Some(self.screen.clone());
        self.screen = screen;
    }

    /// Goes back to the previous screen.
    pub fn go_back(&mut self) {
        if let Some(prev) = self.prev_screen.take() {
            self.screen = prev;
        }
    }

    /// Starts a new game — creates player entity and begins god intro.
    pub fn start_new_game(&mut self) {
        self.creation_step = CreationStep::AvatarSelect;
        self.creation_avatar_idx = 0;
        self.creation_stats = StatBlock::all(STAT_BASE_VALUE);
        self.creation_points_left = FREE_STAT_POINTS;
        self.creation_world_skill = None;
        self.tags = TagSystem::new();
        self.tags.set("game_started");
        self.completed_stories = Vec::new();
        self.available_stories = vec!["intro_god".to_string()];
        self.gold = 50;
        self.potions = Vec::new();
        self.owned_equipment = Vec::new();
        self.equipment = EquipmentSlots::default();
        self.potion_belt = PotionBelt::new();
        self.companions = Vec::new();
        self.player = None;

        // Start god intro story
        self.start_story("intro_god");
        self.go_to(GameScreen::GodIntro);
    }

    /// Finalizes character creation and enters the world.
    pub fn finalize_character(&mut self) {
        // Extract what we need from data first to avoid borrow conflicts
        let portrait = self.data.as_ref()
            .and_then(|d| d.classes.first())
            .map(|cls| cls.portrait.clone())
            .unwrap_or_default();

        let stats = self.creation_stats.clone();
        let id = self.next_id();

        let mut player = Entity::new(
            id,
            "Godsent".to_string(),
            EntityKind::Player,
            "knight".to_string(),
            stats,
        );

        player.portrait = portrait;
        player.attack_type = AttackType::Melee;
        player.position = FormationRow::Front;

        if let Some(ref skill) = self.creation_world_skill {
            player.world_skills.push(skill.clone());
        }

        self.player = Some(player);
        self.player_avatar = format!("avatars/avatar_{:02}.png", self.creation_avatar_idx + 1);

        self.tags.set("chose_knight");
        self.tags.set("character_created");

        self.available_stories.push("mvp_story_01".to_string());
        self.start_story("mvp_story_01");
        self.go_to(GameScreen::Story);

        self.auto_save();
    }

    /// Starts a story by ID.
    pub fn start_story(&mut self, story_id: &str) {
        if let Some(data) = &self.data {
            if let Some(story) = data.stories.iter().find(|s| s.story_id == story_id) {
                let state = StoryState::new(story);
                self.current_story = Some(story.clone());
                self.story_state = Some(state);
                self.story_text_progress = 0.0;
                self.story_page = 0;
            }
        }
    }

    /// Starts combat with an enemy group.
    pub fn start_combat(&mut self, enemy_group: &str, on_win: &str, on_lose: &str) {
        // Extract enemy data first to avoid borrow conflicts with self.next_id()
        let enemy_defs = self.data.as_ref()
            .and_then(|data| data.enemy_groups.get(enemy_group))
            .map(|group| group.enemies.clone());

        if let Some(enemy_defs) = enemy_defs {
            let mut allies = Vec::new();

            // Add player
            if let Some(ref player) = self.player {
                let mut p = player.clone();
                // Apply equipment bonuses
                let bonuses = self.equipment.total_bonuses();
                p.stats = p.stats.add(&bonuses);
                p.recalculate_derived();
                p.current_hp = p.derived.max_hp;
                allies.push(p);
            }

            // Add companions
            for comp in &self.companions {
                let mut c = comp.clone();
                c.current_hp = c.derived.max_hp;
                allies.push(c);
            }

            // Create enemies (now safe to call self.next_id())
            let enemies: Vec<Entity> = enemy_defs.iter().map(|e| {
                let id = self.next_id();
                enemy_to_entity(e, id)
            }).collect();

            self.battle = Some(BattleState::new(allies, enemies));
            self.combat_on_win = Some(on_win.to_string());
            self.combat_on_lose = Some(on_lose.to_string());
            self.combat_target_select = false;
            self.combat_selected_target = None;
            self.player_atb_ready = false;
            self.go_to(GameScreen::Combat);
        }
    }

    /// Opens a shop by ID.
    pub fn open_shop(&mut self, shop_id: &str) {
        if let Some(data) = &self.data {
            if data.shops.contains_key(shop_id) {
                self.current_shop_id = Some(shop_id.to_string());
                // Initialize stock from data if not yet tracked
                if self.shop_stock.is_empty() {
                    if let Some(shop) = data.shops.get(shop_id) {
                        for item in &shop.items {
                            self.shop_stock.insert(
                                item.item_id.clone(),
                                item.stock,
                            );
                        }
                    }
                }
                self.go_to(GameScreen::Shop);
            }
        }
    }

    /// Adds a companion by ID from data.
    pub fn add_companion(&mut self, companion_id: &str) {
        // Extract companion def first to avoid borrow conflict with self.next_id()
        let comp_def = self.data.as_ref()
            .and_then(|data| data.companions.iter().find(|c| c.id == companion_id).cloned());

        if let Some(comp) = comp_def {
            let id = self.next_id();
            let entity = companion_to_entity(&comp, id);
            self.companions.push(entity);
        }
    }

    /// Auto-saves the game.
    pub fn auto_save(&self) {
        let save_data = self.build_save_data();
        let path = platform::save_path();
        if let Err(e) = vassnian_engine::save::save_to_file(&save_data, &path) {
            eprintln!("Auto-save failed: {}", e);
        }
    }

    /// Loads a saved game.
    pub fn load_game(&mut self) -> bool {
        let path = platform::save_path();
        match vassnian_engine::save::load_from_file(&path) {
            Ok(save) => {
                self.restore_from_save(save);
                true
            }
            Err(_) => false,
        }
    }

    /// Checks if a save file exists.
    pub fn has_save(&self) -> bool {
        vassnian_engine::save::save_exists(&platform::save_path())
    }

    /// Builds save data from current state.
    fn build_save_data(&self) -> SaveData {
        let player_data = if let Some(ref p) = self.player {
            PlayerSaveData {
                name: p.name.clone(),
                avatar: self.player_avatar.clone(),
                class: p.class_id.clone(),
                level: p.level,
                exp: p.exp,
                stats: p.stats.clone(),
                combat_skills: p.combat_skills.clone(),
                world_skills: p.world_skills.clone(),
                equipped: self.equipment.clone(),
                injuries: p.injuries,
            }
        } else {
            PlayerSaveData {
                name: "Godsent".to_string(),
                avatar: String::new(),
                class: "knight".to_string(),
                level: 1, exp: 0,
                stats: StatBlock::all(1),
                combat_skills: Vec::new(),
                world_skills: Vec::new(),
                equipped: EquipmentSlots::default(),
                injuries: 0,
            }
        };

        SaveData {
            version: 1,
            player: player_data,
            companions: self.companions.iter().map(|c| c.name.clone()).collect(),
            inventory: InventorySaveData {
                gold: self.gold,
                potions: self.potions.clone(),
                equipment: self.owned_equipment.clone(),
                potion_belt: self.potion_belt.clone(),
            },
            world_state: WorldSaveData {
                tags: self.tags.all_tags(),
                completed_stories: self.completed_stories.clone(),
                available_stories: self.available_stories.clone(),
                world_level: 1,
            },
            current_story: self.story_state.as_ref().map(|s| s.story_id.clone()),
            current_node: self.story_state.as_ref().map(|s| s.current_node.clone()),
        }
    }

    /// Restores game state from save data.
    fn restore_from_save(&mut self, save: SaveData) {
        // Restore player
        let id = self.next_id();
        let mut player = Entity::new(
            id,
            save.player.name.clone(),
            EntityKind::Player,
            save.player.class.clone(),
            save.player.stats.clone(),
        );
        player.level = save.player.level;
        player.exp = save.player.exp;
        player.combat_skills = save.player.combat_skills;
        player.world_skills = save.player.world_skills;
        player.injuries = save.player.injuries;
        self.player = Some(player);
        self.player_avatar = save.player.avatar;
        self.equipment = save.player.equipped;

        // Restore inventory
        self.gold = save.inventory.gold;
        self.potions = save.inventory.potions;
        self.owned_equipment = save.inventory.equipment;
        self.potion_belt = save.inventory.potion_belt;

        // Restore world state
        self.tags = TagSystem::from_vec(save.world_state.tags);
        self.completed_stories = save.world_state.completed_stories;
        self.available_stories = save.world_state.available_stories;

        // Restore companions — extract defs first to avoid borrow conflict
        self.companions.clear();
        let comp_defs: Vec<_> = save.companions.iter().filter_map(|comp_name| {
            self.data.as_ref()
                .and_then(|data| data.companions.iter().find(|c| c.name == *comp_name).cloned())
        }).collect();

        for comp in &comp_defs {
            let cid = self.next_id();
            let entity = companion_to_entity(comp, cid);
            self.companions.push(entity);
        }

        // Resume story if any
        if let Some(story_id) = save.current_story {
            self.start_story(&story_id);
            if let (Some(ref mut state), Some(node)) = (&mut self.story_state, save.current_node) {
                state.current_node = node;
            }
            self.go_to(GameScreen::Story);
        } else {
            // No active story — go to the first available
            if let Some(sid) = self.available_stories.first().cloned() {
                self.start_story(&sid);
                self.go_to(GameScreen::Story);
            }
        }
    }
}
