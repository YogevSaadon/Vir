//! Draw helpers — colors, layout constants, and common rendering functions.
#![allow(dead_code)]

use macroquad::prelude::*;

// -- Color Palette (two-color dark fantasy) --
pub const BG_COLOR: Color = Color::new(0.06, 0.04, 0.08, 1.0);
pub const BG_PANEL: Color = Color::new(0.12, 0.08, 0.15, 1.0);
pub const TEXT_COLOR: Color = Color::new(0.86, 0.78, 0.67, 1.0);
pub const TEXT_DIM: Color = Color::new(0.5, 0.47, 0.42, 1.0);
pub const ACCENT_COLOR: Color = Color::new(0.75, 0.6, 0.35, 1.0);
pub const BUTTON_COLOR: Color = Color::new(0.18, 0.14, 0.22, 1.0);
pub const BUTTON_HOVER: Color = Color::new(0.25, 0.2, 0.3, 1.0);
pub const BUTTON_DISABLED: Color = Color::new(0.1, 0.08, 0.12, 1.0);
pub const HP_GREEN: Color = Color::new(0.2, 0.7, 0.2, 1.0);
pub const HP_YELLOW: Color = Color::new(0.8, 0.7, 0.2, 1.0);
pub const HP_RED: Color = Color::new(0.8, 0.2, 0.2, 1.0);
pub const ATB_COLOR: Color = Color::new(0.6, 0.6, 0.8, 1.0);
pub const SKILL_AVAILABLE: Color = Color::new(0.3, 0.6, 0.3, 1.0);
pub const SKILL_LOCKED: Color = Color::new(0.5, 0.2, 0.2, 1.0);
pub const BORDER_COLOR: Color = Color::new(0.4, 0.35, 0.3, 1.0);
pub const SELECTED_BORDER: Color = Color::new(0.9, 0.75, 0.4, 1.0);

// -- Layout Constants --
pub const DESIGN_WIDTH: f32 = 390.0;
pub const DESIGN_HEIGHT: f32 = 844.0;
pub const PADDING: f32 = 12.0;
pub const BUTTON_HEIGHT: f32 = 48.0;
pub const SMALL_BUTTON_HEIGHT: f32 = 36.0;
pub const FONT_SIZE_TITLE: f32 = 42.0;
pub const FONT_SIZE_HEADER: f32 = 24.0;
pub const FONT_SIZE_BODY: f32 = 18.0;
pub const FONT_SIZE_SMALL: f32 = 14.0;
pub const MIN_TOUCH_SIZE: f32 = 44.0;

/// Returns the scale factor to map design coords to screen coords.
pub fn scale_factor() -> f32 {
    let sx = screen_width() / DESIGN_WIDTH;
    let sy = screen_height() / DESIGN_HEIGHT;
    sx.min(sy)
}

/// Returns the offset to center the design area on screen.
pub fn offset() -> (f32, f32) {
    let s = scale_factor();
    let ox = (screen_width() - DESIGN_WIDTH * s) / 2.0;
    let oy = (screen_height() - DESIGN_HEIGHT * s) / 2.0;
    (ox, oy)
}

/// Converts design coordinates to screen coordinates.
pub fn to_screen(x: f32, y: f32) -> (f32, f32) {
    let s = scale_factor();
    let (ox, oy) = offset();
    (ox + x * s, oy + y * s)
}

/// Converts screen coordinates to design coordinates.
pub fn to_design(sx: f32, sy: f32) -> (f32, f32) {
    let s = scale_factor();
    let (ox, oy) = offset();
    ((sx - ox) / s, (sy - oy) / s)
}

/// Scaled size.
pub fn scaled(v: f32) -> f32 {
    v * scale_factor()
}

/// Draws a colored rectangle as a placeholder for a missing asset.
pub fn draw_placeholder(x: f32, y: f32, w: f32, h: f32, label: &str, color: Color) {
    let (sx, sy) = to_screen(x, y);
    let sw = scaled(w);
    let sh = scaled(h);
    draw_rectangle(sx, sy, sw, sh, color);
    draw_rectangle_lines(sx, sy, sw, sh, 2.0, BORDER_COLOR);
    let fs = scaled(FONT_SIZE_SMALL);
    let dims = measure_text(label, None, fs as u16, 1.0);
    draw_text(
        label,
        sx + (sw - dims.width) / 2.0,
        sy + (sh + dims.height) / 2.0,
        fs,
        TEXT_DIM,
    );
}

/// Draws centered text at design coordinates.
pub fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let fs = scaled(font_size);
    let dims = measure_text(text, None, fs as u16, 1.0);
    let (sx, sy) = to_screen((DESIGN_WIDTH - dims.width / scale_factor()) / 2.0, y);
    draw_text(text, sx, sy, fs, color);
}

/// Draws text at design coordinates.
pub fn draw_text_at(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let (sx, sy) = to_screen(x, y);
    draw_text(text, sx, sy, scaled(font_size), color);
}

/// Draws a panel background at design coordinates.
pub fn draw_panel(x: f32, y: f32, w: f32, h: f32) {
    let (sx, sy) = to_screen(x, y);
    draw_rectangle(sx, sy, scaled(w), scaled(h), BG_PANEL);
    draw_rectangle_lines(sx, sy, scaled(w), scaled(h), 1.0, BORDER_COLOR);
}

/// Returns HP bar color based on percentage.
pub fn hp_color(pct: f32) -> Color {
    if pct > 0.5 { HP_GREEN }
    else if pct > 0.25 { HP_YELLOW }
    else { HP_RED }
}
