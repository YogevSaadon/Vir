//! Main menu screen — New Game, Continue, Options

use crate::app::{App, GameScreen};
use crate::rendering::*;
use crate::ui;

pub fn update(_app: &mut App) {
    // Input handled in draw via button clicks
}

pub fn draw(app: &mut App) {
    // Title
    draw_centered_text("VASSNIAN", 250.0, FONT_SIZE_TITLE, ACCENT_COLOR);
    draw_centered_text("Isekai RPG", 290.0, FONT_SIZE_BODY, TEXT_DIM);

    let btn_w = 220.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;
    let btn_y_start = 400.0;
    let btn_gap = 60.0;

    if ui::button("NEW GAME", btn_x, btn_y_start, btn_w, BUTTON_HEIGHT, true) {
        app.start_new_game();
    }

    let has_save = app.has_save();
    if ui::button("CONTINUE", btn_x, btn_y_start + btn_gap, btn_w, BUTTON_HEIGHT, has_save) {
        if has_save {
            app.load_game();
        }
    }

    if ui::button("OPTIONS", btn_x, btn_y_start + btn_gap * 2.0, btn_w, BUTTON_HEIGHT, true) {
        app.go_to(GameScreen::Options);
    }

    // Version
    draw_centered_text("MVP Build", 800.0, FONT_SIZE_SMALL, TEXT_DIM);
}
