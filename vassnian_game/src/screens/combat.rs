//! Combat screen — ATB-based 2v2 combat with auto-attack

use crate::app::{App, GameScreen};
use crate::rendering::*;
use crate::ui;
use vassnian_engine::combat::battle::{CombatResult, CombatAction};
use vassnian_engine::combat::damage::calculate_melee_damage;
use vassnian_engine::character::entity::EntityKind;
use vassnian_engine::character::stats::DerivedStats;
use vassnian_engine::ai::basic::basic_ai_decide;

pub fn update(app: &mut App) {
    let dt = macroquad::prelude::get_frame_time();

    let battle = match &mut app.battle {
        Some(b) => b,
        None => return,
    };

    if battle.result != CombatResult::InProgress {
        return;
    }

    // Update ATB bars
    let ready_ids = battle.update_atb(dt);

    for unit_id in ready_ids {
        // Check if battle is still in progress
        if battle.result != CombatResult::InProgress {
            break;
        }

        let is_player = battle.get_unit(unit_id)
            .map(|u| u.entity.kind == EntityKind::Player)
            .unwrap_or(false);

        if is_player {
            // Player's turn — wait for input
            app.player_atb_ready = true;
        } else {
            // AI turn — auto-attack
            let action = basic_ai_decide(unit_id, battle);
            execute_action(battle, unit_id, action);
        }
    }
}

fn execute_action(
    battle: &mut vassnian_engine::combat::battle::BattleState,
    attacker_id: u32,
    action: CombatAction,
) {
    match action {
        CombatAction::Attack { target_id } => {
            // Get attacker and target stats
            let attacker_derived = battle.get_unit(attacker_id)
                .map(|u| DerivedStats::from_stats(&u.entity.stats))
                .unwrap_or_default();
            let defender_derived = battle.get_unit(target_id)
                .map(|u| DerivedStats::from_stats(&u.entity.stats))
                .unwrap_or_default();

            let result = calculate_melee_damage(&attacker_derived, &defender_derived);

            let attacker_name = battle.get_unit(attacker_id)
                .map(|u| u.entity.name.clone())
                .unwrap_or_default();
            let target_name = battle.get_unit(target_id)
                .map(|u| u.entity.name.clone())
                .unwrap_or_default();

            if result.is_evade {
                battle.log(format!("{} attacks {} — MISS!", attacker_name, target_name));
            } else {
                // Apply damage
                if let Some(target) = battle.get_unit_mut(target_id) {
                    target.entity.take_damage(result.damage);
                    if !target.entity.is_alive() {
                        target.is_active = false;
                    }
                }

                let crit_text = if result.is_crit { " CRIT!" } else { "" };
                battle.log(format!(
                    "{} attacks {} for {} damage{}",
                    attacker_name, target_name, result.damage, crit_text
                ));
            }

            // Reset attacker ATB
            if let Some(unit) = battle.get_unit_mut(attacker_id) {
                unit.atb.reset();
            }

            // Check win/lose
            battle.check_result();
        }
        CombatAction::UseSkill { skill_id, target_id } => {
            // TODO: Full skill execution — resolve damage/heal/buff based on skill def
            let attacker_name = battle.get_unit(attacker_id)
                .map(|u| u.entity.name.clone())
                .unwrap_or_default();
            battle.log(format!("{} uses {} (not yet implemented)", attacker_name, skill_id));
            let _ = target_id; // suppress unused warning
            if let Some(unit) = battle.get_unit_mut(attacker_id) {
                unit.atb.reset();
            }
        }
        CombatAction::UsePotion { .. } => {
            // MVP: potions handled via UI
        }
        CombatAction::Wait => {
            if let Some(unit) = battle.get_unit_mut(attacker_id) {
                unit.atb.reset();
            }
        }
    }
}

pub fn draw(app: &mut App) {
    // Extract battle display info we need for rendering
    let battle_info = match &app.battle {
        Some(b) => {
            let enemies: Vec<(String, i32, i32, f32, bool, u32)> = b.enemies.iter().map(|u| {
                (u.entity.name.clone(), u.entity.current_hp, u.entity.derived.max_hp,
                 u.atb.percentage(), u.entity.is_alive(), u.entity.id)
            }).collect();
            let allies: Vec<(String, i32, i32, f32, bool)> = b.allies.iter().map(|u| {
                (u.entity.name.clone(), u.entity.current_hp, u.entity.derived.max_hp,
                 u.atb.percentage(), u.entity.is_alive())
            }).collect();
            let result = b.result.clone();
            let paused = b.paused;
            let log: Vec<String> = b.combat_log.clone();
            let player_id = b.player().map(|p| p.entity.id);
            Some((enemies, allies, result, paused, log, player_id))
        }
        None => None,
    };

    let (enemies, allies, result, paused, log, player_id) = match battle_info {
        Some(info) => info,
        None => return,
    };

    // Background placeholder
    draw_placeholder(0.0, 0.0, DESIGN_WIDTH, 120.0, "Battle Background", BG_PANEL);

    // Draw enemies
    draw_text_at("ENEMIES", PADDING * 2.0, 140.0, FONT_SIZE_SMALL, HP_RED);
    let enemy_start_x = PADDING * 2.0;
    let unit_w = 150.0;
    let unit_h = 90.0;
    let gap = 20.0;

    for (i, (name, hp, max_hp, atb_pct, alive, eid)) in enemies.iter().enumerate() {
        let x = enemy_start_x + i as f32 * (unit_w + gap);
        let y = 155.0;
        draw_unit_card(x, y, unit_w, unit_h, name,
            *hp, *max_hp,
            *atb_pct, *alive,
            app.combat_target_select && app.combat_selected_target == Some(*eid),
        );

        // Click to select target
        if app.player_atb_ready && app.combat_target_select && *alive {
            let (sx, sy) = to_screen(x, y);
            let sw = scaled(unit_w);
            let sh = scaled(unit_h);
            let mouse = macroquad::prelude::mouse_position();
            if mouse.0 >= sx && mouse.0 <= sx + sw && mouse.1 >= sy && mouse.1 <= sy + sh
                && macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left)
            {
                if let Some(pid) = player_id {
                    let target_id = *eid;
                    if let Some(ref mut b) = app.battle {
                        execute_action(b, pid, CombatAction::Attack { target_id });
                    }
                    app.player_atb_ready = false;
                    app.combat_target_select = false;
                    app.combat_selected_target = None;
                }
            }
        }
    }

    // Draw allies
    draw_text_at("YOUR PARTY", PADDING * 2.0, 280.0, FONT_SIZE_SMALL, HP_GREEN);
    for (i, (name, hp, max_hp, atb_pct, alive)) in allies.iter().enumerate() {
        let x = enemy_start_x + i as f32 * (unit_w + gap);
        let y = 295.0;
        draw_unit_card(x, y, unit_w, unit_h, name,
            *hp, *max_hp,
            *atb_pct, *alive, false,
        );
    }

    // Skill bar
    let skill_y = 410.0;
    draw_text_at("-- Skill Bar --", PADDING * 2.0, skill_y, FONT_SIZE_SMALL, TEXT_DIM);

    if app.player_atb_ready && result == CombatResult::InProgress {
        if !app.combat_target_select {
            // Show ATK button
            if ui::button("ATK", PADDING * 2.0, skill_y + 20.0, 70.0, BUTTON_HEIGHT, true) {
                app.combat_target_select = true;
            }
            draw_text_at("Your turn! Select an action.", PADDING * 2.0 + 80.0, skill_y + 45.0, FONT_SIZE_SMALL, ACCENT_COLOR);
        } else {
            draw_text_at("Select a target!", PADDING * 2.0, skill_y + 45.0, FONT_SIZE_BODY, ACCENT_COLOR);
            if ui::button("CANCEL", PADDING * 2.0, skill_y + 20.0, 80.0, SMALL_BUTTON_HEIGHT, true) {
                app.combat_target_select = false;
            }
        }
    }

    // Potion belt
    let belt_y = 490.0;
    draw_text_at("-- Potion Belt --", PADDING * 2.0, belt_y, FONT_SIZE_SMALL, TEXT_DIM);
    for i in 0..4 {
        let bx = PADDING * 2.0 + i as f32 * 70.0;
        let by = belt_y + 15.0;
        let label = app.potion_belt.slots.get(i)
            .and_then(|s| s.as_ref())
            .map(|id| {
                if id.contains("hp") { "HP" }
                else if id.contains("fire") { "Fire" }
                else { "?" }
            })
            .unwrap_or("---");
        let has_potion = app.potion_belt.slots.get(i).map_or(false, |s| s.is_some());

        if ui::button(label, bx, by, 60.0, SMALL_BUTTON_HEIGHT, has_potion) {
            // Use potion (MVP: heals player for now)
            if let Some(potion_id) = app.potion_belt.use_slot(i) {
                if potion_id.contains("hp") {
                    if let Some(ref mut b) = app.battle {
                        if let Some(player) = b.player_mut() {
                            player.entity.heal(30);
                            b.log(format!("Used Health Potion! Healed 30 HP."));
                        }
                    }
                } else if potion_id.contains("fire") {
                    // Damage first living enemy
                    if let Some(ref mut b) = app.battle {
                        let target = b.living_enemies().first().map(|e| e.entity.id);
                        if let Some(tid) = target {
                            if let Some(enemy) = b.get_unit_mut(tid) {
                                enemy.entity.take_damage(25);
                                if !enemy.entity.is_alive() {
                                    enemy.is_active = false;
                                }
                                b.log(format!("Used Fire Potion! Dealt 25 damage."));
                            }
                            b.check_result();
                        }
                    }
                }
            }
        }
    }

    // Pause button
    let pause_text = if paused { "RESUME" } else { "PAUSE" };
    if ui::button(pause_text, PADDING * 2.0, 560.0, 100.0, SMALL_BUTTON_HEIGHT, true) {
        if let Some(ref mut b) = app.battle {
            b.toggle_pause();
        }
    }

    // Combat log (last 3 messages)
    let log_y = 610.0;
    draw_text_at("-- Combat Log --", PADDING * 2.0, log_y, FONT_SIZE_SMALL, TEXT_DIM);
    let log_count = log.len();
    let start = if log_count > 3 { log_count - 3 } else { 0 };
    for (i, msg) in log[start..].iter().enumerate() {
        draw_text_at(msg, PADDING * 2.0, log_y + 18.0 + i as f32 * 18.0, FONT_SIZE_SMALL, TEXT_COLOR);
    }

    // Victory / Defeat
    if result == CombatResult::Victory {
        // Calculate rewards from actual enemy defs (scaled)
        let (total_exp, total_gold) = calc_combat_rewards(app);

        let panel_h = if app.level_up_message.is_some() { 150.0 } else { 120.0 };
        draw_panel(40.0, 350.0, 310.0, panel_h);
        draw_centered_text("VICTORY!", 385.0, FONT_SIZE_HEADER, ACCENT_COLOR);
        draw_centered_text(&format!("+{} gold  +{} EXP", total_gold, total_exp), 415.0, FONT_SIZE_BODY, TEXT_COLOR);

        if let Some(ref msg) = app.level_up_message {
            draw_centered_text(msg, 440.0, FONT_SIZE_BODY, ACCENT_COLOR);
        }

        let btn_y = if app.level_up_message.is_some() { 465.0 } else { 440.0 };
        if ui::button("CONTINUE", 95.0, btn_y, 200.0, BUTTON_HEIGHT, true) {
            app.gold += total_gold;
            app.apply_exp(total_exp);
            // Return to story at on_win node
            let win_node = app.combat_on_win.clone();
            if let Some(ref wn) = win_node {
                if let Some(ref mut ss) = app.story_state {
                    ss.advance(wn);
                }
            }
            app.battle = None;
            app.go_to(GameScreen::Story);
            app.story_text_progress = 0.0;
            app.auto_save();
        }
    }

    if result == CombatResult::Defeat {
        draw_panel(40.0, 350.0, 310.0, 140.0);
        draw_centered_text("DEFEAT", 385.0, FONT_SIZE_HEADER, HP_RED);

        // Add injury
        let injuries = app.player.as_ref().map_or(0, |p| p.injuries);
        draw_centered_text(
            &format!("Injuries: {}/3", injuries + 1),
            415.0, FONT_SIZE_BODY, TEXT_COLOR,
        );

        if ui::button("CONTINUE", 95.0, 450.0, 200.0, BUTTON_HEIGHT, true) {
            // Apply injury
            let permadeath = if let Some(ref mut p) = app.player {
                p.add_injury()
            } else {
                false
            };

            if permadeath {
                app.battle = None;
                app.go_to(GameScreen::GameOver);
            } else {
                // Return to story at on_lose node
                let lose_node = app.combat_on_lose.clone();
                if let Some(ref ln) = lose_node {
                    if let Some(ref mut ss) = app.story_state {
                        ss.advance(ln);
                    }
                }
                app.battle = None;
                app.go_to(GameScreen::Story);
                app.story_text_progress = 0.0;
                app.auto_save();
            }
        }
    }
}

/// Calculates combat rewards (exp, gold) from enemy defs with scaling applied.
fn calc_combat_rewards(app: &App) -> (i32, i32) {
    let battle = match &app.battle {
        Some(b) => b,
        None => return (0, 0),
    };

    let mut total_exp = 0i32;
    let mut total_gold = 0i32;

    // Sum base exp/gold from enemies in the battle
    for enemy_unit in &battle.enemies {
        // Look up the original enemy def for base rewards
        let (base_exp, base_gold) = app.data.as_ref()
            .and_then(|data| {
                for group in data.enemy_groups.values() {
                    for edef in &group.enemies {
                        if edef.name == enemy_unit.entity.name {
                            return Some((edef.exp_reward, edef.gold_reward));
                        }
                    }
                }
                None
            })
            .unwrap_or((10, 5)); // fallback if not found

        // Apply scaling to rewards
        let (scaled_exp, scaled_gold) = app.data.as_ref()
            .map(|data| {
                vassnian_engine::combat::scaling::scale_rewards(
                    base_exp, base_gold, 1, app.current_mission_level, &data.scaling_config,
                )
            })
            .unwrap_or((base_exp, base_gold));

        total_exp += scaled_exp;
        total_gold += scaled_gold;
    }

    (total_exp, total_gold)
}

/// Draws a unit card with HP and ATB bars.
fn draw_unit_card(
    x: f32, y: f32, w: f32, h: f32,
    name: &str, hp: i32, max_hp: i32,
    atb_pct: f32, alive: bool, selected: bool,
) {
    draw_panel(x, y, w, h);

    let (sx, sy) = to_screen(x, y);
    if !alive {
        macroquad::prelude::draw_rectangle(sx, sy, scaled(w), scaled(h), BUTTON_DISABLED);
    }

    let name_color = if alive { TEXT_COLOR } else { TEXT_DIM };
    draw_text_at(name, x + 8.0, y + 20.0, FONT_SIZE_SMALL, name_color);

    if alive {
        // HP bar
        let hp_pct = hp as f32 / max_hp.max(1) as f32;
        draw_text_at(&format!("HP:{}/{}", hp, max_hp), x + 8.0, y + 38.0, FONT_SIZE_SMALL - 2.0, TEXT_DIM);
        ui::draw_bar(x + 8.0, y + 44.0, w - 16.0, 10.0, hp_pct, hp_color(hp_pct), BG_COLOR);

        // ATB bar
        draw_text_at("ATB", x + 8.0, y + 64.0, FONT_SIZE_SMALL - 2.0, TEXT_DIM);
        ui::draw_bar(x + 8.0, y + 68.0, w - 16.0, 8.0, atb_pct, ATB_COLOR, BG_COLOR);
    } else {
        draw_text_at("DEFEATED", x + 8.0, y + 50.0, FONT_SIZE_SMALL, HP_RED);
    }

    if selected {
        let (sx, sy) = to_screen(x, y);
        macroquad::prelude::draw_rectangle_lines(sx, sy, scaled(w), scaled(h), 3.0, SELECTED_BORDER);
    }
}
