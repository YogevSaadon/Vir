//! Character creation screen — Avatar, Class, Stats, World Skill selection

use crate::app::{App, CreationStep};
use crate::rendering::*;
use crate::ui;
use vassnian_engine::character::stats::{DerivedStats, STAT_DESCRIPTIONS};

pub fn update(_app: &mut App) {
    // Input handled in draw via button clicks
}

pub fn draw(app: &mut App) {
    match app.creation_step.clone() {
        CreationStep::AvatarSelect => draw_avatar_select(app),
        CreationStep::ClassSelect => draw_class_select(app),
        CreationStep::StatPointBuy => draw_stat_buy(app),
        CreationStep::WorldSkillSelect => draw_world_skill(app),
    }
}

fn draw_avatar_select(app: &mut App) {
    draw_centered_text("Choose Your Avatar", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);

    let avatars = 4;
    let size = 70.0;
    let gap = 15.0;
    let start_x = (DESIGN_WIDTH - (size * 2.0 + gap)) / 2.0;
    let start_y = 120.0;

    for i in 0..avatars {
        let row = i / 2;
        let col = i % 2;
        let x = start_x + col as f32 * (size + gap);
        let y = start_y + row as f32 * (size + gap);

        let selected = app.creation_avatar_idx == i;
        let label = format!("Avatar {}", i + 1);

        if ui::selectable_item(&label, x, y, size, size, selected) {
            app.creation_avatar_idx = i;
        }
    }

    let sel_label = format!("Selected: Avatar {}", app.creation_avatar_idx + 1);
    draw_centered_text(&sel_label, 320.0, FONT_SIZE_BODY, TEXT_COLOR);

    let btn_w = 200.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;
    if ui::button("CONFIRM", btn_x, 380.0, btn_w, BUTTON_HEIGHT, true) {
        app.creation_step = CreationStep::ClassSelect;
    }
}

fn draw_class_select(app: &mut App) {
    draw_centered_text("Choose Your Class", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);

    let x = PADDING * 3.0;
    let y = 120.0;
    let w = DESIGN_WIDTH - PADDING * 6.0;
    let h = 80.0;

    // Knight (only option for MVP)
    if let Some(data) = &app.data {
        if let Some(cls) = data.classes.first() {
            draw_panel(x, y, w, h);
            draw_text_at(&cls.name, x + PADDING, y + 30.0, FONT_SIZE_HEADER, ACCENT_COLOR);
            draw_text_at(&cls.description, x + PADDING, y + 55.0, FONT_SIZE_SMALL, TEXT_DIM);

            // Info panel
            let info_y = y + h + 20.0;
            draw_panel(x, info_y, w, 180.0);
            draw_text_at("Class Info:", x + PADDING, info_y + 25.0, FONT_SIZE_BODY, ACCENT_COLOR);
            draw_text_at(&format!("Role: {:?}", cls.category), x + PADDING, info_y + 50.0, FONT_SIZE_SMALL, TEXT_COLOR);
            draw_text_at(&format!("Position: {:?}", cls.position), x + PADDING, info_y + 70.0, FONT_SIZE_SMALL, TEXT_COLOR);
            draw_text_at(&format!("Attack: {:?}", cls.attack_type), x + PADDING, info_y + 90.0, FONT_SIZE_SMALL, TEXT_COLOR);

            // Base stats preview
            let stats_y = info_y + 115.0;
            let stats = &cls.base_stats;
            draw_text_at(
                &format!("STR:{} VIT:{} INT:{} FAI:{}", stats.strength, stats.vitality, stats.intelligence, stats.faith),
                x + PADDING, stats_y, FONT_SIZE_SMALL, TEXT_COLOR,
            );
            draw_text_at(
                &format!("SPD:{} DEX:{} LCK:{}", stats.speed, stats.dexterity, stats.luck),
                x + PADDING, stats_y + 20.0, FONT_SIZE_SMALL, TEXT_COLOR,
            );

            draw_text_at(&format!("\"{}\"", cls.flavor_text), x + PADDING, stats_y + 45.0, FONT_SIZE_SMALL, TEXT_DIM);
        }
    }

    let btn_w = 200.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;
    if ui::button("CONFIRM", btn_x, 480.0, btn_w, BUTTON_HEIGHT, true) {
        // Apply class base stats to creation stats
        let base_stats = app.data.as_ref()
            .and_then(|d| d.classes.first())
            .map(|cls| cls.base_stats.clone());
        if let Some(stats) = base_stats {
            app.creation_stats = stats;
        }
        app.creation_step = CreationStep::StatPointBuy;
    }

    if ui::button("< BACK", PADDING * 2.0, 480.0, 80.0, BUTTON_HEIGHT, true) {
        app.creation_step = CreationStep::AvatarSelect;
    }
}

fn draw_stat_buy(app: &mut App) {
    draw_centered_text("Distribute Stats", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_centered_text(
        &format!("Points left: {}", app.creation_points_left),
        85.0, FONT_SIZE_BODY,
        if app.creation_points_left > 0 { ACCENT_COLOR } else { TEXT_DIM },
    );

    let x = PADDING * 2.0;
    let mut y = 120.0;
    let row_h = 38.0;
    let btn_size = 30.0;

    // Collect stat info first to avoid borrow issues
    let stat_info: Vec<(String, String, String, i32, i32)> = STAT_DESCRIPTIONS.iter().map(|(abbr, name, desc)| {
        let value = app.creation_stats.get(name).unwrap_or(1);
        let base = app.data.as_ref()
            .and_then(|d| d.classes.first())
            .and_then(|c| c.base_stats.get(name))
            .unwrap_or(1);
        (abbr.to_string(), name.to_string(), desc.to_string(), value, base)
    }).collect();

    let points_left = app.creation_points_left;

    for (abbr, name, desc, value, base) in &stat_info {
        // Stat name and value
        draw_text_at(&format!("{}: {}", abbr, value), x, y + 22.0, FONT_SIZE_BODY, TEXT_COLOR);

        // Minus button
        let can_minus = *value > *base;
        if ui::small_button("-", x + 120.0, y, btn_size, can_minus) {
            let new_val = value - 1;
            app.creation_stats.set(name, new_val);
            app.creation_points_left += 1;
        }

        // Plus button
        let can_plus = points_left > 0;
        if ui::small_button("+", x + 160.0, y, btn_size, can_plus) {
            let new_val = value + 1;
            app.creation_stats.set(name, new_val);
            app.creation_points_left -= 1;
        }

        // Info icon area - show description on hover/tooltip area
        draw_text_at(desc, x + 200.0, y + 20.0, FONT_SIZE_SMALL - 2.0, TEXT_DIM);

        y += row_h;
    }

    // Derived stats
    y += 15.0;
    draw_text_at("-- Derived Stats --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
    y += 28.0;

    let derived = DerivedStats::from_stats(&app.creation_stats);
    draw_text_at(&format!("HP: {}  ATK: {}  DEF: {}", derived.max_hp, derived.physical_damage, derived.defense), x, y, FONT_SIZE_SMALL, TEXT_COLOR);
    y += 22.0;
    draw_text_at(&format!("CRIT: {:.0}%  EVA: {:.0}%  SPD: {:.2}", derived.crit_chance * 100.0, derived.evasion * 100.0, derived.atb_speed), x, y, FONT_SIZE_SMALL, TEXT_COLOR);

    // Buttons
    let btn_w = 200.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;
    let all_spent = app.creation_points_left == 0;
    if ui::button("CONFIRM", btn_x, 480.0, btn_w, BUTTON_HEIGHT, all_spent) {
        app.creation_step = CreationStep::WorldSkillSelect;
    }

    if ui::button("< BACK", PADDING * 2.0, 480.0, 80.0, BUTTON_HEIGHT, true) {
        // Reset points when going back
        let base_stats = app.data.as_ref()
            .and_then(|d| d.classes.first())
            .map(|cls| cls.base_stats.clone());
        if let Some(stats) = base_stats {
            app.creation_stats = stats;
            app.creation_points_left = vassnian_engine::character::stats::FREE_STAT_POINTS;
        }
        app.creation_step = CreationStep::ClassSelect;
    }
}

fn draw_world_skill(app: &mut App) {
    draw_centered_text("Choose World Skill", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_centered_text("(Pick 1)", 80.0, FONT_SIZE_SMALL, TEXT_DIM);

    let x = PADDING * 3.0;
    let w = DESIGN_WIDTH - PADDING * 6.0;
    let h = 50.0;
    let gap = 10.0;
    let mut y = 120.0;

    // Extract skills to avoid borrow issues
    let skills: Vec<(String, String, String)> = app.data.as_ref()
        .map(|d| d.world_skills.iter().map(|s| (s.id.clone(), s.name.clone(), s.description.clone())).collect())
        .unwrap_or_default();

    let selected_skill = app.creation_world_skill.clone();
    let mut selected_desc: Option<String> = None;

    for (skill_id, skill_name, skill_desc) in &skills {
        let selected = selected_skill.as_deref() == Some(skill_id.as_str());

        if ui::selectable_item(skill_name, x, y, w, h, selected) {
            app.creation_world_skill = Some(skill_id.clone());
        }

        if selected {
            selected_desc = Some(skill_desc.clone());
        }

        y += h + gap;
    }

    // Description panel — always visible at a fixed position below the skill list
    let desc_y = 320.0;
    let desc_h = 80.0;
    draw_panel(x, desc_y, w, desc_h);
    if let Some(desc) = &selected_desc {
        draw_text_at("Skill Info:", x + PADDING, desc_y + 22.0, FONT_SIZE_SMALL, ACCENT_COLOR);
        draw_text_at(desc, x + PADDING, desc_y + 48.0, FONT_SIZE_SMALL, TEXT_COLOR);
    } else {
        draw_centered_text("Select a skill to see its description", desc_y + 45.0, FONT_SIZE_SMALL, TEXT_DIM);
    }

    let btn_w = 200.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;
    let has_selection = app.creation_world_skill.is_some();
    if ui::button("CONFIRM", btn_x, 480.0, btn_w, BUTTON_HEIGHT, has_selection) {
        app.finalize_character();
    }

    if ui::button("< BACK", PADDING * 2.0, 480.0, 80.0, BUTTON_HEIGHT, true) {
        app.creation_step = CreationStep::StatPointBuy;
    }
}
