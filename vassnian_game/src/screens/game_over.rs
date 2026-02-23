//! Game over screen — shown on party wipe

use crate::app::{App, GameScreen};
use crate::rendering::*;
use crate::ui;

pub fn update(_app: &mut App) {
    // Input handled in draw
}

pub fn draw(app: &mut App) {
    let _ = app; // suppress unused warning

    draw_centered_text("YOU HAVE DIED", 280.0, FONT_SIZE_TITLE, HP_RED);

    draw_centered_text(
        "Your party has been defeated.",
        340.0, FONT_SIZE_HEADER, TEXT_DIM,
    );

    draw_centered_text(
        "Your journey ends here... for now.",
        400.0, FONT_SIZE_BODY, TEXT_COLOR,
    );

    let btn_w = 220.0;
    let btn_x = (DESIGN_WIDTH - btn_w) / 2.0;

    if ui::button("NEW GAME", btn_x, 500.0, btn_w, BUTTON_HEIGHT, true) {
        app.start_new_game();
    }

    if ui::button("MAIN MENU", btn_x, 560.0, btn_w, BUTTON_HEIGHT, true) {
        app.go_to(GameScreen::MainMenu);
    }
}
