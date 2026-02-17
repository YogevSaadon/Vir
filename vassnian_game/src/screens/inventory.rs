//! Inventory screen -- visual grid with character silhouette and equipment slots

use crate::app::{App, SelectedItem};
use crate::rendering::*;
use crate::ui;
use vassnian_engine::character::equipment::EquipSlot;
use vassnian_engine::inventory::items::EquipmentInstance;

/// Size of each inventory grid square.
const SLOT_SIZE: f32 = 56.0;
/// Gap between slots.
const SLOT_GAP: f32 = 6.0;
/// Silhouette area dimensions.
const SILHOUETTE_W: f32 = 200.0;
const SILHOUETTE_H: f32 = 280.0;
/// Equipment slot square size (around silhouette).
const EQUIP_SLOT_SIZE: f32 = 46.0;
/// Gap between equipment slots in columns.
const EQUIP_GAP: f32 = 4.0;

pub fn update(_app: &mut App) {}

pub fn draw(app: &mut App) {
    draw_centered_text("Inventory", 40.0, FONT_SIZE_HEADER, ACCENT_COLOR);

    // Gold display
    draw_text_at(&format!("Gold: {}", app.gold), PADDING * 2.0, 70.0, FONT_SIZE_BODY, ACCENT_COLOR);

    // --- Character selector (player + companions) ---
    draw_char_selector(app);

    // --- Character silhouette with equipment slots ---
    draw_equipment_area(app);

    // --- Item grid (shared inventory) ---
    draw_item_grid(app);

    // --- Detail popup (if item selected) ---
    draw_detail_popup(app);

    // Back button
    if ui::button("BACK", PADDING * 2.0, 790.0, 120.0, BUTTON_HEIGHT, true) {
        app.inv_selected_item = None;
        app.go_back();
    }
}

/// Draws clickable character portraits to switch between player/companions.
fn draw_char_selector(app: &mut App) {
    let y = 90.0;
    let portrait_size = 48.0;
    let gap = 8.0;
    let start_x = PADDING * 2.0;

    // Player portrait
    let player_name = app.player.as_ref()
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "Player".to_string());

    let is_selected = app.inv_selected_char == 0;
    if draw_portrait_slot(start_x, y, portrait_size, &player_name, "PLR", is_selected) {
        app.inv_selected_char = 0;
        app.inv_selected_item = None;
    }

    // Companion portraits
    let comp_names: Vec<String> = app.companions.iter()
        .map(|c| c.name.clone())
        .collect();

    for (i, name) in comp_names.iter().enumerate() {
        let cx = start_x + (portrait_size + gap) * (i as f32 + 1.0);
        let is_sel = app.inv_selected_char == i + 1;
        let short = if name.len() > 3 { &name[..3] } else { name };
        if draw_portrait_slot(cx, y, portrait_size, name, short, is_sel) {
            app.inv_selected_char = i + 1;
            app.inv_selected_item = None;
        }
    }
}

/// Draws a portrait slot square. Returns true if clicked.
fn draw_portrait_slot(x: f32, y: f32, size: f32, _name: &str, label: &str, selected: bool) -> bool {
    let (sx, sy) = to_screen(x, y);
    let ss = scaled(size);

    let mouse = macroquad::prelude::mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + ss
        && mouse.1 >= sy && mouse.1 <= sy + ss;

    // Background
    let bg = if selected { BUTTON_HOVER } else if hovered { BG_PANEL } else { BUTTON_COLOR };
    macroquad::prelude::draw_rectangle(sx, sy, ss, ss, bg);

    // Border
    let border = if selected { SELECTED_BORDER } else { BORDER_COLOR };
    macroquad::prelude::draw_rectangle_lines(sx, sy, ss, ss, if selected { 2.0 } else { 1.0 }, border);

    // Label
    let fs = scaled(FONT_SIZE_SMALL);
    let dims = macroquad::prelude::measure_text(label, None, fs as u16, 1.0);
    macroquad::prelude::draw_text(
        label,
        sx + (ss - dims.width) / 2.0,
        sy + (ss + dims.height) / 2.0,
        fs, TEXT_COLOR,
    );

    hovered && macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left)
}

/// Draws the character silhouette area with equipment slots around it.
/// Layout: Left column (Helmet/Necklace/Armor/Boots), center silhouette,
/// right column (3 Rings / Main Hand / Off Hand).
fn draw_equipment_area(app: &mut App) {
    let area_x = (DESIGN_WIDTH - SILHOUETTE_W) / 2.0;
    let area_y = 150.0;

    // Silhouette background
    draw_panel(area_x, area_y, SILHOUETTE_W, SILHOUETTE_H);

    // Draw character placeholder silhouette
    let silhouette_color = macroquad::prelude::Color::new(0.15, 0.2, 0.35, 0.8);
    let body_x = area_x + 60.0;
    let body_w = 80.0;

    // Head
    let (hsx, hsy) = to_screen(body_x + 20.0, area_y + 20.0);
    macroquad::prelude::draw_circle(hsx + scaled(20.0), hsy + scaled(20.0), scaled(22.0), silhouette_color);
    // Body
    draw_placeholder(body_x + 5.0, area_y + 65.0, body_w - 10.0, 120.0, "", silhouette_color);
    // Legs
    draw_placeholder(body_x + 10.0, area_y + 190.0, 25.0, 70.0, "", silhouette_color);
    draw_placeholder(body_x + 35.0, area_y + 190.0, 25.0, 70.0, "", silhouette_color);

    // Character name in silhouette
    let char_name = if app.inv_selected_char == 0 {
        app.player.as_ref().map(|p| p.name.clone()).unwrap_or_default()
    } else {
        app.companions.get(app.inv_selected_char - 1)
            .map(|c| c.name.clone()).unwrap_or_default()
    };
    draw_centered_text(&char_name, area_y + SILHOUETTE_H - 15.0, FONT_SIZE_SMALL, TEXT_COLOR);

    // --- Left column: Helmet, Necklace, Armor, Boots ---
    let left_x = area_x - EQUIP_SLOT_SIZE - 6.0;
    let left_start_y = area_y + 10.0;
    let step = EQUIP_SLOT_SIZE + EQUIP_GAP;

    draw_equip_slot(app, left_x, left_start_y, "HELM", &EquipSlot::Helmet, None);
    draw_equip_slot(app, left_x, left_start_y + step, "NECK", &EquipSlot::Necklace, None);
    draw_equip_slot(app, left_x, left_start_y + step * 2.0, "ARMR", &EquipSlot::Armor, None);
    draw_equip_slot(app, left_x, left_start_y + step * 3.0, "BOOT", &EquipSlot::Boots, None);

    // --- Right column: 3 Rings (small, stacked), Main Hand, Off Hand ---
    let right_x = area_x + SILHOUETTE_W + 6.0;

    // 3 Ring slots (smaller, stacked)
    let ring_size = 32.0;
    let ring_gap = 3.0;
    let ring_y = area_y + 10.0;
    for i in 0..3 {
        let ry = ring_y + i as f32 * (ring_size + ring_gap);
        let label = format!("R{}", i + 1);
        draw_ring_slot(app, right_x + (EQUIP_SLOT_SIZE - ring_size) / 2.0, ry, ring_size, &label, i);
    }

    // Main Hand
    let hand_y = ring_y + 3.0 * (ring_size + ring_gap) + 8.0;
    draw_equip_slot(app, right_x, hand_y, "MAIN", &EquipSlot::MainHand, None);

    // Off Hand (locked indicator if two-handed)
    let off_y = hand_y + step;
    let off_locked = app.equipment.is_off_hand_locked();
    let off_label = if off_locked { "LOCK" } else { "OFF" };
    draw_equip_slot(app, right_x, off_y, off_label, &EquipSlot::OffHand, Some(off_locked));
}

/// Draws an equipment slot square. Clicking selects it for detail popup.
/// `locked` = Some(true) dims the slot (e.g., off-hand locked by two-handed weapon).
fn draw_equip_slot(app: &mut App, x: f32, y: f32, label: &str, slot: &EquipSlot, locked: Option<bool>) {
    let (sx, sy) = to_screen(x, y);
    let ss = scaled(EQUIP_SLOT_SIZE);

    let equipped = app.equipment.get(slot);
    let is_locked = locked.unwrap_or(false);
    let is_selected = matches!(&app.inv_selected_item, Some(SelectedItem::Equipment { slot: s }) if s == slot);

    let mouse = macroquad::prelude::mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + ss
        && mouse.1 >= sy && mouse.1 <= sy + ss;

    // Background
    let bg = if is_locked {
        macroquad::prelude::Color::new(0.1, 0.1, 0.1, 1.0)
    } else if is_selected {
        BUTTON_HOVER
    } else if equipped.is_some() {
        macroquad::prelude::Color::new(0.2, 0.18, 0.12, 1.0)
    } else {
        BUTTON_COLOR
    };
    macroquad::prelude::draw_rectangle(sx, sy, ss, ss, bg);

    let border = if is_selected { SELECTED_BORDER } else { BORDER_COLOR };
    macroquad::prelude::draw_rectangle_lines(sx, sy, ss, ss, if is_selected { 2.0 } else { 1.0 }, border);

    // Show item short name or slot label
    let display = if is_locked && equipped.is_none() {
        label.to_string()
    } else if let Some(item) = equipped {
        let n = &item.name;
        if n.len() > 5 { n[..5].to_string() } else { n.clone() }
    } else {
        label.to_string()
    };

    let fs = scaled(FONT_SIZE_SMALL);
    let dims = macroquad::prelude::measure_text(&display, None, fs as u16, 1.0);
    let text_color = if is_locked {
        TEXT_DIM
    } else if equipped.is_some() {
        ACCENT_COLOR
    } else {
        TEXT_DIM
    };
    macroquad::prelude::draw_text(
        &display,
        sx + (ss - dims.width) / 2.0,
        sy + (ss + dims.height) / 2.0,
        fs, text_color,
    );

    // Click handler (don't allow clicking locked slots)
    if !is_locked && hovered && macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
        app.inv_selected_item = Some(SelectedItem::Equipment { slot: slot.clone() });
    }
}

/// Draws a ring slot (smaller square). Clicking selects it.
fn draw_ring_slot(app: &mut App, x: f32, y: f32, size: f32, label: &str, ring_index: usize) {
    let (sx, sy) = to_screen(x, y);
    let ss = scaled(size);

    let equipped = app.equipment.get_ring(ring_index);
    let is_selected = matches!(
        &app.inv_selected_item,
        Some(SelectedItem::RingSlot { index }) if *index == ring_index
    );

    let mouse = macroquad::prelude::mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + ss
        && mouse.1 >= sy && mouse.1 <= sy + ss;

    let bg = if is_selected {
        BUTTON_HOVER
    } else if equipped.is_some() {
        macroquad::prelude::Color::new(0.2, 0.18, 0.12, 1.0)
    } else {
        BUTTON_COLOR
    };
    macroquad::prelude::draw_rectangle(sx, sy, ss, ss, bg);

    let border = if is_selected { SELECTED_BORDER } else { BORDER_COLOR };
    macroquad::prelude::draw_rectangle_lines(sx, sy, ss, ss, if is_selected { 2.0 } else { 1.0 }, border);

    let display = if let Some(item) = equipped {
        let n = &item.name;
        if n.len() > 4 { n[..4].to_string() } else { n.clone() }
    } else {
        label.to_string()
    };

    let fs = scaled(9.0);
    let dims = macroquad::prelude::measure_text(&display, None, fs as u16, 1.0);
    macroquad::prelude::draw_text(
        &display,
        sx + (ss - dims.width) / 2.0,
        sy + (ss + dims.height) / 2.0,
        fs,
        if equipped.is_some() { ACCENT_COLOR } else { TEXT_DIM },
    );

    if hovered && macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left) {
        app.inv_selected_item = Some(SelectedItem::RingSlot { index: ring_index });
    }
}

/// Draws the shared item grid at the bottom (2 rows of squares).
fn draw_item_grid(app: &mut App) {
    let grid_y = 460.0;
    let grid_x = PADDING * 2.0;
    let cols = 5;

    draw_text_at("-- Items --", grid_x, grid_y - 10.0, FONT_SIZE_SMALL, ACCENT_COLOR);

    // Build item list: owned equipment + potions
    let mut slot_idx = 0;

    // Owned equipment
    let owned_names: Vec<(String, String)> = app.owned_equipment.iter().map(|e| {
        let name = app.data.as_ref()
            .and_then(|data| data.equipment.iter().find(|eq| eq.id == e.equipment_id))
            .map(|eq| eq.name.clone())
            .unwrap_or_else(|| e.equipment_id.clone());
        let short = if name.len() > 5 { name[..5].to_string() } else { name.clone() };
        (name, short)
    }).collect();

    for (i, (_name, short)) in owned_names.iter().enumerate() {
        let col = slot_idx % cols;
        let row = slot_idx / cols;
        let sx = grid_x + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let sy = grid_y + row as f32 * (SLOT_SIZE + SLOT_GAP);

        let is_sel = matches!(&app.inv_selected_item, Some(SelectedItem::OwnedEquipment { index }) if *index == i);
        if draw_item_slot(sx, sy, short, ACCENT_COLOR, is_sel) {
            app.inv_selected_item = Some(SelectedItem::OwnedEquipment { index: i });
        }
        slot_idx += 1;
    }

    // Potions
    let potion_display: Vec<(String, i32)> = app.potions.iter().map(|p| {
        let name = app.data.as_ref()
            .and_then(|data| data.potions.iter().find(|pd| pd.id == p.potion_id))
            .map(|pd| pd.name.clone())
            .unwrap_or_else(|| p.potion_id.clone());
        (name, p.count)
    }).collect();

    for (i, (name, count)) in potion_display.iter().enumerate() {
        let col = slot_idx % cols;
        let row = slot_idx / cols;
        let sx = grid_x + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let sy = grid_y + row as f32 * (SLOT_SIZE + SLOT_GAP);

        let short = if name.len() > 4 {
            format!("{}x{}", &name[..4], count)
        } else {
            format!("{}x{}", name, count)
        };

        let is_sel = matches!(&app.inv_selected_item, Some(SelectedItem::Potion { index }) if *index == i);
        if draw_item_slot(sx, sy, &short, HP_GREEN, is_sel) {
            app.inv_selected_item = Some(SelectedItem::Potion { index: i });
        }
        slot_idx += 1;
    }

    // Fill remaining slots as empty
    let total_slots = cols * 2; // 2 rows
    while slot_idx < total_slots {
        let col = slot_idx % cols;
        let row = slot_idx / cols;
        let sx = grid_x + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let sy = grid_y + row as f32 * (SLOT_SIZE + SLOT_GAP);
        draw_item_slot(sx, sy, "", TEXT_DIM, false);
        slot_idx += 1;
    }

    // Potion belt row
    let belt_y = grid_y + 2.0 * (SLOT_SIZE + SLOT_GAP) + 15.0;
    draw_text_at("-- Belt --", grid_x, belt_y - 10.0, FONT_SIZE_SMALL, ACCENT_COLOR);

    let belt_info: Vec<(usize, Option<String>, String)> = (0..4).map(|i| {
        let slot = app.potion_belt.slots.get(i).and_then(|s| s.clone());
        let label = match &slot {
            Some(id) => {
                app.data.as_ref()
                    .and_then(|data| data.potions.iter().find(|p| p.id == *id))
                    .map(|p| {
                        let n = &p.name;
                        if n.len() > 5 { n[..5].to_string() } else { n.clone() }
                    })
                    .unwrap_or_else(|| id.clone())
            }
            None => String::new(),
        };
        (i, slot, label)
    }).collect();

    for (i, _slot, label) in &belt_info {
        let sx = grid_x + *i as f32 * (SLOT_SIZE + SLOT_GAP);
        let is_sel = matches!(&app.inv_selected_item, Some(SelectedItem::BeltSlot { index }) if *index == *i);
        let color = if label.is_empty() { TEXT_DIM } else { HP_GREEN };
        if draw_item_slot(sx, belt_y, label, color, is_sel) {
            app.inv_selected_item = Some(SelectedItem::BeltSlot { index: *i });
        }
    }
}

/// Draws a single item slot square. Returns true if clicked.
fn draw_item_slot(x: f32, y: f32, label: &str, color: macroquad::prelude::Color, selected: bool) -> bool {
    let (sx, sy) = to_screen(x, y);
    let ss = scaled(SLOT_SIZE);

    let mouse = macroquad::prelude::mouse_position();
    let hovered = mouse.0 >= sx && mouse.0 <= sx + ss
        && mouse.1 >= sy && mouse.1 <= sy + ss;

    let bg = if selected {
        BUTTON_HOVER
    } else if hovered && !label.is_empty() {
        macroquad::prelude::Color::new(0.15, 0.12, 0.2, 1.0)
    } else {
        BUTTON_COLOR
    };

    macroquad::prelude::draw_rectangle(sx, sy, ss, ss, bg);
    let border = if selected { SELECTED_BORDER } else { BORDER_COLOR };
    macroquad::prelude::draw_rectangle_lines(sx, sy, ss, ss, if selected { 2.0 } else { 1.0 }, border);

    if !label.is_empty() {
        // Item icon placeholder: small colored square
        let icon_pad = scaled(8.0);
        let icon_size = ss - icon_pad * 2.0;
        macroquad::prelude::draw_rectangle(sx + icon_pad, sy + icon_pad, icon_size, icon_size * 0.6, color);

        // Text label
        let fs = scaled(10.0);
        let dims = macroquad::prelude::measure_text(label, None, fs as u16, 1.0);
        macroquad::prelude::draw_text(
            label,
            sx + (ss - dims.width) / 2.0,
            sy + ss - scaled(4.0),
            fs, TEXT_COLOR,
        );
    }

    hovered && !label.is_empty() && macroquad::prelude::is_mouse_button_pressed(macroquad::prelude::MouseButton::Left)
}

/// Draws the detail popup when an item is selected.
fn draw_detail_popup(app: &mut App) {
    let selected = match &app.inv_selected_item {
        Some(s) => s.clone(),
        None => return,
    };

    let popup_x = PADDING * 3.0;
    let popup_y = 200.0;
    let popup_w = DESIGN_WIDTH - PADDING * 6.0;
    let popup_h = 240.0;

    // Darken background
    let (sx, sy) = to_screen(0.0, 0.0);
    macroquad::prelude::draw_rectangle(sx, sy, scaled(DESIGN_WIDTH), scaled(DESIGN_HEIGHT),
        macroquad::prelude::Color::new(0.0, 0.0, 0.0, 0.5));

    draw_panel(popup_x, popup_y, popup_w, popup_h);

    let text_x = popup_x + PADDING;
    let mut ty = popup_y + 30.0;
    let btn_y = popup_y + popup_h - 60.0;

    match selected {
        SelectedItem::Equipment { ref slot } => {
            let equipped = app.equipment.get(slot).clone();

            if let Some(item) = equipped {
                draw_text_at(&item.name, text_x, ty, FONT_SIZE_BODY, ACCENT_COLOR);
                ty += 25.0;

                // Get description from data
                let desc = app.data.as_ref()
                    .and_then(|data| data.equipment.iter().find(|e| e.id == item.item_id))
                    .map(|e| e.description.clone())
                    .unwrap_or_default();
                draw_text_at(&desc, text_x, ty, FONT_SIZE_SMALL, TEXT_COLOR);
                ty += 25.0;

                // Show stat bonuses
                let bonuses = &item.stat_bonuses;
                let bonus_str = format_stat_bonuses(bonuses);
                if !bonus_str.is_empty() {
                    draw_text_at(&bonus_str, text_x, ty, FONT_SIZE_SMALL, HP_GREEN);
                }

                // Unequip button
                if ui::button("UNEQUIP", popup_x + PADDING, btn_y, 120.0, BUTTON_HEIGHT, true) {
                    let slot_ref = slot.clone();
                    if let Some(unequipped) = app.equipment.unequip(&slot_ref) {
                        app.owned_equipment.push(EquipmentInstance {
                            equipment_id: unequipped.item_id,
                        });
                    }
                    app.inv_selected_item = None;
                }
            } else {
                let slot_name = slot.display_name();
                draw_text_at(&format!("{} slot (empty)", slot_name), text_x, ty, FONT_SIZE_BODY, TEXT_DIM);
            }
        }

        SelectedItem::RingSlot { index } => {
            let equipped = app.equipment.get_ring(index).clone();

            if let Some(item) = equipped {
                draw_text_at(&format!("Ring {} — {}", index + 1, item.name), text_x, ty, FONT_SIZE_BODY, ACCENT_COLOR);
                ty += 25.0;

                let desc = app.data.as_ref()
                    .and_then(|data| data.equipment.iter().find(|e| e.id == item.item_id))
                    .map(|e| e.description.clone())
                    .unwrap_or_default();
                draw_text_at(&desc, text_x, ty, FONT_SIZE_SMALL, TEXT_COLOR);
                ty += 25.0;

                let bonus_str = format_stat_bonuses(&item.stat_bonuses);
                if !bonus_str.is_empty() {
                    draw_text_at(&bonus_str, text_x, ty, FONT_SIZE_SMALL, HP_GREEN);
                }

                if ui::button("UNEQUIP", popup_x + PADDING, btn_y, 120.0, BUTTON_HEIGHT, true) {
                    if let Some(unequipped) = app.equipment.unequip_ring(index) {
                        app.owned_equipment.push(EquipmentInstance {
                            equipment_id: unequipped.item_id,
                        });
                    }
                    app.inv_selected_item = None;
                }
            } else {
                draw_text_at(&format!("Ring slot {} (empty)", index + 1), text_x, ty, FONT_SIZE_BODY, TEXT_DIM);
            }
        }

        SelectedItem::OwnedEquipment { index } => {
            let equip_info = app.owned_equipment.get(index).and_then(|e| {
                app.data.as_ref().and_then(|data| {
                    data.equipment.iter().find(|eq| eq.id == e.equipment_id).cloned()
                })
            });

            if let Some(equip_def) = equip_info {
                draw_text_at(&equip_def.name, text_x, ty, FONT_SIZE_BODY, ACCENT_COLOR);
                ty += 25.0;
                draw_text_at(&equip_def.description, text_x, ty, FONT_SIZE_SMALL, TEXT_COLOR);
                ty += 25.0;

                let bonus_str = format_stat_bonuses(&equip_def.stat_bonuses);
                if !bonus_str.is_empty() {
                    draw_text_at(&bonus_str, text_x, ty, FONT_SIZE_SMALL, HP_GREEN);
                }

                let slot_name = equip_def.slot.display_name();
                draw_text_at(&format!("Slot: {}", slot_name), text_x, ty + 25.0, FONT_SIZE_SMALL, TEXT_DIM);

                // Equip button
                if ui::button("EQUIP", popup_x + PADDING, btn_y, 120.0, BUTTON_HEIGHT, true) {
                    let equip_slot = equip_def.slot.clone();
                    let weapon_hand = equip_def.weapon_hand.clone();
                    let equipped_item = vassnian_engine::character::equipment::EquippedItem {
                        item_id: equip_def.id.clone(),
                        name: equip_def.name.clone(),
                        slot: equip_slot,
                        stat_bonuses: equip_def.stat_bonuses.clone(),
                        weapon_hand,
                    };
                    // Unequip old item first if any
                    if let Some(old) = app.equipment.equip(equipped_item) {
                        app.owned_equipment.push(EquipmentInstance {
                            equipment_id: old.item_id,
                        });
                    }
                    // Remove from owned
                    if index < app.owned_equipment.len() {
                        app.owned_equipment.remove(index);
                    }
                    app.inv_selected_item = None;
                }
            }
        }

        SelectedItem::Potion { index } => {
            let potion_info = app.potions.get(index).and_then(|p| {
                app.data.as_ref().and_then(|data| {
                    data.potions.iter().find(|pd| pd.id == p.potion_id).cloned()
                }).map(|pd| (pd, p.count))
            });

            if let Some((potion_def, count)) = potion_info {
                draw_text_at(&format!("{} x{}", potion_def.name, count), text_x, ty, FONT_SIZE_BODY, HP_GREEN);
                ty += 25.0;
                draw_text_at(&potion_def.description, text_x, ty, FONT_SIZE_SMALL, TEXT_COLOR);

                // Add to belt button
                let belt_full = app.potion_belt.is_full();
                if ui::button("TO BELT", popup_x + PADDING, btn_y, 120.0, BUTTON_HEIGHT, !belt_full && count > 0) {
                    let pid = app.potions[index].potion_id.clone();
                    app.potion_belt.add(&pid);
                    if let Some(p) = app.potions.iter_mut().find(|p| p.potion_id == pid) {
                        p.count -= 1;
                    }
                    app.potions.retain(|p| p.count > 0);
                    app.inv_selected_item = None;
                }
            }
        }

        SelectedItem::BeltSlot { index } => {
            let slot_content = app.potion_belt.slots.get(index).and_then(|s| s.clone());

            if let Some(potion_id) = slot_content {
                let name = app.data.as_ref()
                    .and_then(|data| data.potions.iter().find(|p| p.id == potion_id))
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| potion_id.clone());

                draw_text_at(&format!("Belt {}: {}", index + 1, name), text_x, ty, FONT_SIZE_BODY, HP_GREEN);

                if ui::button("REMOVE", popup_x + PADDING, btn_y, 120.0, BUTTON_HEIGHT, true) {
                    if let Some(pid) = app.potion_belt.remove(index) {
                        if let Some(p) = app.potions.iter_mut().find(|p| p.potion_id == pid) {
                            p.count += 1;
                        } else {
                            app.potions.push(vassnian_engine::inventory::items::PotionStack {
                                potion_id: pid,
                                count: 1,
                            });
                        }
                    }
                    app.inv_selected_item = None;
                }
            } else {
                draw_text_at(&format!("Belt slot {} (empty)", index + 1), text_x, ty, FONT_SIZE_BODY, TEXT_DIM);
            }
        }
    }

    // Close button
    if ui::button("CLOSE", popup_x + popup_w - PADDING - 100.0, btn_y, 100.0, BUTTON_HEIGHT, true) {
        app.inv_selected_item = None;
    }
}

/// Formats stat bonuses into a readable string.
fn format_stat_bonuses(stats: &vassnian_engine::character::stats::StatBlock) -> String {
    let mut parts = Vec::new();
    let str_val = stats.get("strength").unwrap_or(0);
    let vit_val = stats.get("vitality").unwrap_or(0);
    let int_val = stats.get("intelligence").unwrap_or(0);
    let spd_val = stats.get("speed").unwrap_or(0);
    let dex_val = stats.get("dexterity").unwrap_or(0);

    if str_val != 0 { parts.push(format!("STR +{}", str_val)); }
    if vit_val != 0 { parts.push(format!("VIT +{}", vit_val)); }
    if int_val != 0 { parts.push(format!("INT +{}", int_val)); }
    if spd_val != 0 { parts.push(format!("SPD +{}", spd_val)); }
    if dex_val != 0 { parts.push(format!("DEX +{}", dex_val)); }

    parts.join("  ")
}
