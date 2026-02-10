//! Stats screen — displays player stats, skills, and injuries

use crate::app::App;
use crate::rendering::*;
use crate::ui;
use vassnian_engine::character::stats::DerivedStats;

pub fn draw(app: &mut App) {
    draw_centered_text("Character Stats", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);

    if let Some(ref player) = app.player {
        let x = PADDING * 2.0;
        let mut y = 100.0;

        // Name and class
        draw_text_at(&format!("{} — Knight Lv{}", player.name, player.level), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 40.0;

        // Primary stats
        draw_text_at("-- Primary Stats --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
        y += 30.0;

        for (name, value) in player.stats.iter() {
            draw_text_at(&format!("{}: {}", name, value), x, y, FONT_SIZE_BODY, TEXT_COLOR);
            y += 24.0;
        }

        y += 10.0;

        // Derived stats
        let derived = DerivedStats::from_stats(&player.stats);
        draw_text_at("-- Derived Stats --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
        y += 30.0;

        draw_text_at(&format!("HP: {}/{}", player.current_hp, derived.max_hp), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 24.0;
        draw_text_at(&format!("ATK: {}", derived.physical_damage), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 24.0;
        draw_text_at(&format!("DEF: {}", derived.defense), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 24.0;
        draw_text_at(&format!("CRIT: {:.0}%", derived.crit_chance * 100.0), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 24.0;
        draw_text_at(&format!("EVA: {:.0}%", derived.evasion * 100.0), x, y, FONT_SIZE_BODY, TEXT_COLOR);
        y += 40.0;

        // World skills
        draw_text_at("-- World Skills --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
        y += 30.0;
        if player.world_skills.is_empty() {
            draw_text_at("(none)", x, y, FONT_SIZE_BODY, TEXT_DIM);
        } else {
            for skill in &player.world_skills {
                draw_text_at(skill, x, y, FONT_SIZE_BODY, TEXT_COLOR);
                y += 24.0;
            }
        }
        y += 20.0;

        // Injuries
        draw_text_at(
            &format!("Injuries: {}/3", player.injuries),
            x, y, FONT_SIZE_BODY,
            if player.injuries > 0 { HP_RED } else { TEXT_COLOR },
        );
    }

    // Back button
    if ui::button("BACK", PADDING * 2.0, 750.0, 120.0, BUTTON_HEIGHT, true) {
        app.go_back();
    }
}
