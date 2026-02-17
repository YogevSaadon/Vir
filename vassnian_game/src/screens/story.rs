//! Story screen -- paginated text with arrow navigation, choices on last page

use crate::app::{App, GameScreen};
use crate::rendering::*;
use crate::ui;
use vassnian_engine::story::engine::{get_current_node, is_story_end};
use vassnian_engine::story::choice::{ChoiceRequirement, check_stat_requirement, check_world_skill};

/// Max lines of text per page (fits in the text panel area).
const LINES_PER_PAGE: usize = 12;
/// Line height in design coords.
const LINE_HEIGHT: f32 = 24.0;

pub fn update(app: &mut App) {
    // Story text progress (typewriter effect)
    if app.story_text_progress < 1.0 {
        let speed = match app.text_speed {
            crate::app::TextSpeed::Instant => 100.0,
            crate::app::TextSpeed::Medium => 2.0,
            crate::app::TextSpeed::Slow => 0.8,
        };
        app.story_text_progress += speed * macroquad::prelude::get_frame_time();
        app.story_text_progress = app.story_text_progress.min(1.0);
    }
}

pub fn draw(app: &mut App) {
    // Clone story data we need to avoid borrow issues
    let story = match &app.current_story {
        Some(s) => s.clone(),
        None => {
            draw_centered_text("No story loaded", 400.0, FONT_SIZE_BODY, TEXT_DIM);
            if ui::button("MAIN MENU", 100.0, 500.0, 200.0, BUTTON_HEIGHT, true) {
                app.go_to(GameScreen::MainMenu);
            }
            return;
        }
    };
    let state = match &app.story_state {
        Some(st) => st.clone(),
        None => {
            draw_centered_text("No story state", 400.0, FONT_SIZE_BODY, TEXT_DIM);
            return;
        }
    };

    let node = match get_current_node(&story, &state) {
        Some(n) => n.clone(),
        None => {
            draw_centered_text("Story node not found", 400.0, FONT_SIZE_BODY, HP_RED);
            return;
        }
    };

    // Background placeholder
    if let Some(ref bg) = node.background {
        draw_placeholder(0.0, 0.0, DESIGN_WIDTH, 200.0, bg, BG_PANEL);
    }

    // Speaker name
    let mut header_y = 220.0;
    if let Some(ref speaker) = node.speaker {
        draw_text_at(speaker, PADDING * 2.0, header_y, FONT_SIZE_BODY, ACCENT_COLOR);
        header_y += 30.0;
    }

    // Story text (with typewriter)
    let full_text = &node.text;
    let chars_to_show = (full_text.len() as f32 * app.story_text_progress) as usize;
    let visible_text: String = full_text.chars().take(chars_to_show).collect();

    // Word-wrap the text
    let max_width = DESIGN_WIDTH - PADDING * 4.0;
    let lines = word_wrap(&visible_text, max_width, FONT_SIZE_BODY);

    // Split lines into pages
    let total_text_pages = (lines.len() + LINES_PER_PAGE - 1) / LINES_PER_PAGE;
    let total_text_pages = total_text_pages.max(1);

    // Tap to advance if text not fully shown
    if app.story_text_progress < 1.0 {
        // Show first page of text so far
        let page_lines = &lines[..lines.len().min(LINES_PER_PAGE)];
        let panel_h = 20.0 + page_lines.len() as f32 * LINE_HEIGHT;
        draw_panel(PADDING, header_y - 10.0, DESIGN_WIDTH - PADDING * 2.0, panel_h);
        let text_x = PADDING * 2.0;
        for (i, line) in page_lines.iter().enumerate() {
            draw_text_at(line, text_x, header_y + 10.0 + i as f32 * LINE_HEIGHT, FONT_SIZE_BODY, TEXT_COLOR);
        }

        if macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
            app.story_text_progress = 1.0;
        }
        return;
    }

    // --- Text fully revealed: pagination mode ---

    // Apply node tags (once text is shown)
    app.tags.set_many(&node.tags_to_set);

    // Apply node rewards once per visit
    let reward_key = format!("{}:{}", state.story_id, state.current_node);
    if !app.claimed_reward_nodes.contains(&reward_key) {
        if let Some(ref rewards) = node.rewards {
            if rewards.gold > 0 {
                app.gold += rewards.gold;
            }
            if rewards.exp > 0 {
                app.apply_exp(rewards.exp);
            }
        }
        app.claimed_reward_nodes.insert(reward_key);
    }

    // Check for companion join
    if app.tags.has("companion_aldric_joined") && app.companions.is_empty() {
        app.add_companion("sir_aldric");
    }

    // Check for story end
    let is_end = is_story_end(&node);

    // Determine if choices exist (non-end nodes with available choices)
    let has_choices = !is_end && !node.choices.is_empty();

    // Total pages: text pages + 1 choice page (if there are choices)
    let total_pages = if has_choices || is_end {
        total_text_pages + 1
    } else {
        total_text_pages
    };

    // Clamp current page
    if app.story_page >= total_pages {
        app.story_page = total_pages.saturating_sub(1);
    }

    let text_x = PADDING * 2.0;
    let on_choice_page = app.story_page >= total_text_pages;

    if on_choice_page {
        // --- Choice / End page ---
        if is_end {
            draw_story_end(app, &state, &node, header_y, text_x);
        } else {
            draw_choices_page(app, &node, header_y, text_x, max_width);
        }
    } else {
        // --- Text page ---
        let start = app.story_page * LINES_PER_PAGE;
        let end = (start + LINES_PER_PAGE).min(lines.len());
        let page_lines = &lines[start..end];

        let panel_h = 20.0 + page_lines.len() as f32 * LINE_HEIGHT;
        draw_panel(PADDING, header_y - 10.0, DESIGN_WIDTH - PADDING * 2.0, panel_h);

        for (i, line) in page_lines.iter().enumerate() {
            draw_text_at(line, text_x, header_y + 10.0 + i as f32 * LINE_HEIGHT, FONT_SIZE_BODY, TEXT_COLOR);
        }

        // Show rewards on first text page if any
        if app.story_page == 0 {
            if let Some(ref rewards) = node.rewards {
                let reward_y = header_y + 10.0 + page_lines.len() as f32 * LINE_HEIGHT + 10.0;
                if rewards.gold > 0 {
                    draw_text_at(&format!("+{} gold", rewards.gold), text_x, reward_y, FONT_SIZE_SMALL, ACCENT_COLOR);
                }
                if rewards.exp > 0 {
                    let ey = if rewards.gold > 0 { reward_y + 20.0 } else { reward_y };
                    draw_text_at(&format!("+{} EXP", rewards.exp), text_x, ey, FONT_SIZE_SMALL, ACCENT_COLOR);
                }
            }
        }
    }

    // Navigation arrows (above the menu buttons)
    let arrow_y = 700.0;
    let arrow_w = 100.0;
    let arrow_h = BUTTON_HEIGHT;

    // Page indicator
    draw_centered_text(
        &format!("{} / {}", app.story_page + 1, total_pages),
        arrow_y + 15.0, FONT_SIZE_SMALL, TEXT_DIM,
    );

    // Back arrow (can't go before page 0)
    let can_go_back = app.story_page > 0;
    if ui::button("<  BACK", PADDING * 2.0, arrow_y, arrow_w, arrow_h, can_go_back) {
        if can_go_back {
            app.story_page -= 1;
        }
    }

    // Forward arrow (can't go past last page)
    let can_go_forward = app.story_page < total_pages - 1;
    if ui::button("NEXT  >", DESIGN_WIDTH - PADDING * 2.0 - arrow_w, arrow_y, arrow_w, arrow_h, can_go_forward) {
        if can_go_forward {
            app.story_page += 1;
        }
    }

    // Menu buttons (below arrows)
    let menu_y = 760.0;
    let has_player = app.player.is_some();
    let shop_available = node.allows_shop && node.shop_id.is_some();
    let shop_id_clone = node.shop_id.clone();

    if shop_available {
        // 4-button layout: INV, STATS, SHOP, OPT
        let menu_btn_w = 70.0;
        let gap = 6.0;
        let total_w = menu_btn_w * 4.0 + gap * 3.0;
        let menu_x = (DESIGN_WIDTH - total_w) / 2.0;

        if ui::button("INV", menu_x, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, has_player) {
            app.go_to(GameScreen::Inventory);
        }
        if ui::button("STATS", menu_x + (menu_btn_w + gap), menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, has_player) {
            app.go_to(GameScreen::Stats);
        }
        if ui::button("SHOP", menu_x + (menu_btn_w + gap) * 2.0, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, true) {
            if let Some(ref sid) = shop_id_clone {
                app.open_shop(sid);
            }
        }
        if ui::button("OPT", menu_x + (menu_btn_w + gap) * 3.0, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, true) {
            app.go_to(GameScreen::InGameOptions);
        }
    } else {
        // 3-button layout: INV, STATS, OPT
        let menu_btn_w = 80.0;
        let menu_x = (DESIGN_WIDTH - menu_btn_w * 3.0 - 16.0) / 2.0;

        if ui::button("INV", menu_x, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, has_player) {
            app.go_to(GameScreen::Inventory);
        }
        if ui::button("STATS", menu_x + menu_btn_w + 8.0, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, has_player) {
            app.go_to(GameScreen::Stats);
        }
        if ui::button("OPT", menu_x + (menu_btn_w + 8.0) * 2.0, menu_y, menu_btn_w, SMALL_BUTTON_HEIGHT, true) {
            app.go_to(GameScreen::InGameOptions);
        }
    }

    // Show level-up message if any
    if let Some(ref msg) = app.level_up_message {
        draw_panel(30.0, 800.0, 330.0, 30.0);
        draw_centered_text(msg, 820.0, FONT_SIZE_SMALL, ACCENT_COLOR);
    }
}

/// Draws the story end page (complete message + continue/menu buttons).
fn draw_story_end(app: &mut App, state: &vassnian_engine::story::engine::StoryState, node: &vassnian_engine::story::engine::StoryNode, y: f32, _text_x: f32) {
    // Unlock stories
    for sid in &node.unlock_stories {
        if !app.available_stories.contains(sid) {
            app.available_stories.push(sid.clone());
        }
    }

    // Mark completed + record in journal
    let story_id = state.story_id.clone();
    if !app.completed_stories.contains(&story_id) {
        app.completed_stories.push(story_id.clone());
        if let Some(ref story_def) = app.current_story {
            app.journal.record(story_def);
        }
    }

    draw_panel(PADDING, y - 10.0, DESIGN_WIDTH - PADDING * 2.0, 120.0);
    draw_centered_text("--- Story Complete ---", y + 20.0, FONT_SIZE_BODY, ACCENT_COLOR);

    // Check if this was the god intro
    if app.screen == GameScreen::GodIntro {
        if ui::button("BEGIN YOUR JOURNEY", 70.0, y + 60.0, 250.0, BUTTON_HEIGHT, true) {
            app.go_to(GameScreen::CharacterCreation);
        }
        return;
    }

    // Find next available story
    let next = app.available_stories.iter()
        .find(|s| !app.completed_stories.contains(s))
        .cloned();

    if let Some(next_id) = next {
        if ui::button("CONTINUE", 95.0, y + 60.0, 200.0, BUTTON_HEIGHT, true) {
            app.start_story(&next_id);
            app.story_page = 0;
            app.auto_save();
        }
    } else {
        draw_centered_text("-- To be continued --", y + 60.0, FONT_SIZE_BODY, TEXT_DIM);
        if ui::button("MAIN MENU", 95.0, y + 100.0, 200.0, BUTTON_HEIGHT, true) {
            app.go_to(GameScreen::MainMenu);
        }
    }
}

/// Draws the choices page with "What do you do?" header.
fn draw_choices_page(app: &mut App, node: &vassnian_engine::story::engine::StoryNode, y: f32, _text_x: f32, _max_width: f32) {
    // Header
    draw_panel(PADDING, y - 10.0, DESIGN_WIDTH - PADDING * 2.0, 40.0);
    draw_centered_text("What do you do?", y + 15.0, FONT_SIZE_HEADER, ACCENT_COLOR);

    // Get player/companion skills for requirement checks
    let player_skills: Vec<String> = app.player.as_ref()
        .map(|p| p.world_skills.clone())
        .unwrap_or_default();
    let companion_skills: Vec<String> = app.companions.iter()
        .flat_map(|c| c.world_skills.clone())
        .collect();
    let player_stats = app.player.as_ref()
        .map(|p| p.stats.clone())
        .unwrap_or_default();

    let choices = node.choices.clone();

    let choice_w = DESIGN_WIDTH - PADDING * 4.0;
    let choice_h = 52.0;
    let choice_gap = 8.0;
    let choice_x = PADDING * 2.0;
    let mut choice_y = y + 50.0;

    for choice in &choices {
        // Check requirement
        let (available, req_text) = match &choice.requirement {
            None => (true, String::new()),
            Some(ChoiceRequirement::Stat { stat, min_value }) => {
                let val = player_stats.get(stat).unwrap_or(0);
                let met = check_stat_requirement(&player_stats, stat, *min_value);
                let label = format!("[{} {}] (You: {})", stat.to_uppercase(), min_value, val);
                (met, label)
            }
            Some(ChoiceRequirement::WorldSkill { skill_id, source }) => {
                let (met, who) = check_world_skill(&player_skills, &companion_skills, skill_id, source);
                let label = if let Some(ref w) = who {
                    format!("[{}: {}]", w, skill_id)
                } else {
                    format!("[{}]", skill_id)
                };
                (met, label)
            }
            Some(ChoiceRequirement::Tag { tag }) => {
                (app.tags.has(tag), String::new())
            }
            Some(ChoiceRequirement::Item { item_id }) => {
                let has = app.potions.iter().any(|p| p.potion_id == *item_id)
                    || app.owned_equipment.iter().any(|e| e.equipment_id == *item_id);
                (has, String::new())
            }
        };

        if !available {
            continue;
        }

        let is_shop = choice.text.contains("[OPEN SHOP]");

        let display = if req_text.is_empty() {
            choice.text.clone()
        } else {
            format!("{} {}", req_text, choice.text)
        };

        if ui::button(&display, choice_x, choice_y, choice_w, choice_h, true) {
            app.tags.set_many(&choice.tags_to_set);

            if let Some(ref combat) = choice.trigger_combat {
                let eg = combat.enemy_group.clone();
                let ow = combat.on_win.clone();
                let ol = combat.on_lose.clone();
                app.start_combat(&eg, &ow, &ol);
                return;
            }

            if is_shop {
                app.open_shop("village_shop");
                if let Some(ref mut ss) = app.story_state {
                    ss.advance(&choice.next_node);
                }
                return;
            }

            if let Some(ref mut ss) = app.story_state {
                ss.advance(&choice.next_node);
            }
            app.story_text_progress = 0.0;
            app.story_page = 0;
            app.auto_save();
        }

        choice_y += choice_h + choice_gap;
    }
}

/// Simple word-wrap: splits text into lines that fit within max_width.
fn word_wrap(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let test = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        let dims = macroquad::prelude::measure_text(&test, None, (font_size * scale_factor()) as u16, 1.0);
        if dims.width > max_width * scale_factor() && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
