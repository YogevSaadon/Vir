//! Reusable UI components — buttons, tooltips, bars
#![allow(dead_code)]

use macroquad::prelude::*;
use crate::rendering::*;

/// Truncates text to fit within a pixel width, adding ".." if needed.
fn fit_text(text: &str, max_width: f32, font_size: f32) -> String {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    if dims.width <= max_width {
        return text.to_string();
    }
    let mut truncated = text.to_string();
    while !truncated.is_empty() {
        truncated.pop();
        let candidate = format!("{}..", truncated.trim_end());
        let dims = measure_text(&candidate, None, font_size as u16, 1.0);
        if dims.width <= max_width {
            return candidate;
        }
    }
    "..".to_string()
}

/// A clickable button. Returns true if clicked this frame.
pub fn button(text: &str, x: f32, y: f32, w: f32, h: f32, enabled: bool) -> bool {
    let (sx, sy) = to_screen(x, y);
    let sw = scaled(w);
    let sh = scaled(h);

    let mouse = mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + sw
        && mouse.1 >= sy && mouse.1 <= sy + sh;

    let color = if !enabled {
        BUTTON_DISABLED
    } else if hovered {
        BUTTON_HOVER
    } else {
        BUTTON_COLOR
    };

    draw_rectangle(sx, sy, sw, sh, color);
    draw_rectangle_lines(sx, sy, sw, sh, 1.0, BORDER_COLOR);

    let fs = scaled(FONT_SIZE_BODY);
    let text_color = if enabled { TEXT_COLOR } else { TEXT_DIM };
    let pad = scaled(PADDING);
    let display_text = fit_text(text, sw - pad * 2.0, fs);
    let dims = measure_text(&display_text, None, fs as u16, 1.0);
    draw_text(
        &display_text,
        sx + (sw - dims.width) / 2.0,
        sy + (sh + dims.height) / 2.0 - 2.0,
        fs,
        text_color,
    );

    enabled && hovered && is_mouse_button_pressed(MouseButton::Left)
}

/// A small square button (for +/- in stat buy). Returns true if clicked.
pub fn small_button(text: &str, x: f32, y: f32, size: f32, enabled: bool) -> bool {
    button(text, x, y, size, size, enabled)
}

/// Draws an HP/ATB bar at design coordinates.
pub fn draw_bar(x: f32, y: f32, w: f32, h: f32, pct: f32, color: Color, bg: Color) {
    let (sx, sy) = to_screen(x, y);
    let sw = scaled(w);
    let sh = scaled(h);
    draw_rectangle(sx, sy, sw, sh, bg);
    draw_rectangle(sx, sy, sw * pct.clamp(0.0, 1.0), sh, color);
    draw_rectangle_lines(sx, sy, sw, sh, 1.0, BORDER_COLOR);
}

/// A draggable slider. Returns the new value (0.0..1.0) if changed, or the old value.
pub fn slider(x: f32, y: f32, w: f32, h: f32, value: f32, color: Color, bg: Color) -> f32 {
    let (sx, sy) = to_screen(x, y);
    let sw = scaled(w);
    let sh = scaled(h);

    // Draw background
    draw_rectangle(sx, sy, sw, sh, bg);
    // Draw fill
    let clamped = value.clamp(0.0, 1.0);
    draw_rectangle(sx, sy, sw * clamped, sh, color);
    draw_rectangle_lines(sx, sy, sw, sh, 1.0, BORDER_COLOR);

    // Draw handle
    let handle_x = sx + sw * clamped - 4.0;
    draw_rectangle(handle_x, sy - 2.0, 8.0, sh + 4.0, TEXT_COLOR);

    // Check for mouse interaction (click or drag)
    let mouse = mouse_position();
    let hit_area_pad = scaled(10.0); // extra padding for touch
    if mouse.0 >= sx && mouse.0 <= sx + sw
        && mouse.1 >= sy - hit_area_pad && mouse.1 <= sy + sh + hit_area_pad
        && is_mouse_button_down(MouseButton::Left)
    {
        let new_pct = ((mouse.0 - sx) / sw).clamp(0.0, 1.0);
        return new_pct;
    }

    value
}

/// Draws a tooltip panel with text at design coordinates.
pub fn draw_tooltip(x: f32, y: f32, w: f32, lines: &[&str]) {
    let line_height = FONT_SIZE_SMALL + 4.0;
    let h = PADDING * 2.0 + line_height * lines.len() as f32;
    draw_panel(x, y, w, h);
    for (i, line) in lines.iter().enumerate() {
        draw_text_at(
            line,
            x + PADDING,
            y + PADDING + line_height * (i as f32 + 1.0),
            FONT_SIZE_SMALL,
            TEXT_COLOR,
        );
    }
}

/// Draws a selectable list item. Returns true if clicked.
pub fn selectable_item(
    text: &str,
    x: f32, y: f32, w: f32, h: f32,
    selected: bool,
) -> bool {
    let (sx, sy) = to_screen(x, y);
    let sw = scaled(w);
    let sh = scaled(h);

    let mouse = mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + sw
        && mouse.1 >= sy && mouse.1 <= sy + sh;

    let bg = if selected { BUTTON_HOVER } else if hovered { BG_PANEL } else { BG_COLOR };
    draw_rectangle(sx, sy, sw, sh, bg);

    let border = if selected { SELECTED_BORDER } else { BORDER_COLOR };
    draw_rectangle_lines(sx, sy, sw, sh, if selected { 2.0 } else { 1.0 }, border);

    let fs = scaled(FONT_SIZE_BODY);
    let dims = measure_text(text, None, fs as u16, 1.0);
    draw_text(
        text,
        sx + PADDING * scale_factor(),
        sy + (sh + dims.height) / 2.0 - 2.0,
        fs,
        TEXT_COLOR,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
