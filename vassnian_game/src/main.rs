//! Vassnian — Dark Fantasy Isekai RPG
//! Entry point: launches Macroquad window in portrait orientation.

use macroquad::prelude::*;

mod app;
mod screens;
mod ui;
mod rendering;
mod platform;

/// Macroquad window configuration — portrait aspect ratio.
fn window_conf() -> Conf {
    Conf {
        window_title: "Vassnian".to_string(),
        window_width: 585,
        window_height: 1266,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = app::App::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
