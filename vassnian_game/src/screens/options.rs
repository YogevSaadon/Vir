//! Options screen -- text speed, volume settings

use crate::app::{App, GameScreen, TextSpeed};
use crate::rendering::*;
use crate::ui;

pub fn update(_app: &mut App) {
    // Input handled in draw
}

/// Draws shared options controls (text speed, volume). Returns the y after controls.
fn draw_options_controls(app: &mut App) -> f32 {
    // Text Speed
    draw_text_at("Text Speed:", PADDING * 2.0, 130.0, FONT_SIZE_BODY, TEXT_COLOR);

    let btn_w = 100.0;
    let btn_y = 150.0;
    if ui::button(
        "Instant",
        PADDING * 2.0, btn_y, btn_w, SMALL_BUTTON_HEIGHT,
        app.text_speed != TextSpeed::Instant,
    ) {
        app.text_speed = TextSpeed::Instant;
    }

    if ui::button(
        "Medium",
        PADDING * 2.0 + btn_w + 10.0, btn_y, btn_w, SMALL_BUTTON_HEIGHT,
        app.text_speed != TextSpeed::Medium,
    ) {
        app.text_speed = TextSpeed::Medium;
    }

    if ui::button(
        "Slow",
        PADDING * 2.0 + (btn_w + 10.0) * 2.0, btn_y, btn_w, SMALL_BUTTON_HEIGHT,
        app.text_speed != TextSpeed::Slow,
    ) {
        app.text_speed = TextSpeed::Slow;
    }

    // Current selection indicator
    let current = match app.text_speed {
        TextSpeed::Instant => "Instant",
        TextSpeed::Medium => "Medium",
        TextSpeed::Slow => "Slow",
    };
    draw_text_at(
        &format!("Current: {}", current),
        PADDING * 2.0, 210.0, FONT_SIZE_SMALL, TEXT_DIM,
    );

    // Music Volume
    draw_text_at("Music Volume:", PADDING * 2.0, 270.0, FONT_SIZE_BODY, TEXT_COLOR);
    let vol_pct = (app.music_volume * 100.0) as i32;
    draw_text_at(&format!("{}%", vol_pct), 300.0, 270.0, FONT_SIZE_BODY, ACCENT_COLOR);

    app.music_volume = ui::slider(PADDING * 2.0, 290.0, 320.0, 20.0, app.music_volume, ACCENT_COLOR, BG_PANEL);

    // SFX Volume
    draw_text_at("SFX Volume:", PADDING * 2.0, 350.0, FONT_SIZE_BODY, TEXT_COLOR);
    let sfx_pct = (app.sfx_volume * 100.0) as i32;
    draw_text_at(&format!("{}%", sfx_pct), 300.0, 350.0, FONT_SIZE_BODY, ACCENT_COLOR);

    app.sfx_volume = ui::slider(PADDING * 2.0, 370.0, 320.0, 20.0, app.sfx_volume, ACCENT_COLOR, BG_PANEL);

    420.0
}

/// Main menu options screen (just Back button).
pub fn draw(app: &mut App) {
    draw_centered_text("Options", 60.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_options_controls(app);

    // Back button
    if ui::button("BACK", PADDING * 2.0, 750.0, 120.0, BUTTON_HEIGHT, true) {
        app.go_back();
    }
}

/// In-game options screen (Back + Return to Main Menu).
pub fn draw_in_game(app: &mut App) {
    draw_centered_text("Options", 60.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_options_controls(app);

    let btn_w = 220.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;

    // Back to game
    if ui::button("BACK TO GAME", btn_x, 700.0, btn_w, BUTTON_HEIGHT, true) {
        app.go_back();
    }

    // Return to main menu
    if ui::button("RETURN TO MENU", btn_x, 760.0, btn_w, BUTTON_HEIGHT, true) {
        app.go_to(GameScreen::MainMenu);
    }
}
