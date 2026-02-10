//! Inventory screen — view items, equip gear, manage potion belt

use crate::app::App;
use crate::rendering::*;
use crate::ui;
use vassnian_engine::character::equipment::{EquippedItem, EquipSlot};

pub fn update(_app: &mut App) {
    // Input handled in draw
}

pub fn draw(app: &mut App) {
    draw_centered_text("Inventory", 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_text_at(&format!("Gold: {}", app.gold), PADDING * 2.0, 90.0, FONT_SIZE_BODY, ACCENT_COLOR);

    let x = PADDING * 2.0;
    let mut y = 120.0;

    // Equipment section
    draw_text_at("-- Equipment --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
    y += 30.0;

    let weapon_name = app.equipment.weapon.as_ref()
        .map(|w| w.name.clone()).unwrap_or_else(|| "[empty]".to_string());
    draw_text_at(&format!("Weapon: {}", weapon_name), x, y, FONT_SIZE_BODY, TEXT_COLOR);

    // Equip button for unequipped weapons
    if app.equipment.weapon.is_none() {
        // Extract equippable weapon info first
        let equippable: Vec<(String, String, vassnian_engine::character::stats::StatBlock)> = app.owned_equipment.iter()
            .filter_map(|equip_inst| {
                app.data.as_ref().and_then(|data| {
                    data.equipment.iter().find(|e| e.id == equip_inst.equipment_id).map(|equip_def| {
                        (equip_def.id.clone(), equip_def.name.clone(), equip_def.stat_bonuses.clone())
                    })
                })
            })
            .collect();

        for (equip_id, equip_name, stat_bonuses) in &equippable {
            if ui::button(
                &format!("Equip {}", equip_name),
                x + 200.0, y - 15.0, 140.0, SMALL_BUTTON_HEIGHT, true,
            ) {
                app.equipment.equip(EquippedItem {
                    item_id: equip_id.clone(),
                    name: equip_name.clone(),
                    slot: EquipSlot::Weapon,
                    stat_bonuses: stat_bonuses.clone(),
                });
                let eid = equip_id.clone();
                app.owned_equipment.retain(|e| e.equipment_id != eid);
            }
        }
    } else {
        // Unequip button
        if ui::button("Unequip", x + 250.0, y - 15.0, 90.0, SMALL_BUTTON_HEIGHT, true) {
            if let Some(item) = app.equipment.unequip(&EquipSlot::Weapon) {
                app.owned_equipment.push(vassnian_engine::inventory::items::EquipmentInstance {
                    equipment_id: item.item_id,
                });
            }
        }
    }

    y += 30.0;
    let armor_name = app.equipment.armor.as_ref()
        .map(|a| a.name.as_str()).unwrap_or("[empty]");
    draw_text_at(&format!("Armor:  {}", armor_name), x, y, FONT_SIZE_BODY, TEXT_COLOR);
    y += 30.0;
    let acc_name = app.equipment.accessory.as_ref()
        .map(|a| a.name.as_str()).unwrap_or("[empty]");
    draw_text_at(&format!("Acc:    {}", acc_name), x, y, FONT_SIZE_BODY, TEXT_COLOR);
    y += 40.0;

    // Potions section
    draw_text_at("-- Potions --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
    y += 30.0;

    if app.potions.is_empty() {
        draw_text_at("(none)", x, y, FONT_SIZE_BODY, TEXT_DIM);
        y += 30.0;
    } else {
        // Extract potion display info
        let potion_info: Vec<(String, String, i32)> = app.potions.iter().map(|potion| {
            let name = app.data.as_ref()
                .and_then(|data| data.potions.iter().find(|p| p.id == potion.potion_id))
                .map(|p| p.name.clone())
                .unwrap_or_else(|| potion.potion_id.clone());
            (potion.potion_id.clone(), name, potion.count)
        }).collect();

        let belt_full = app.potion_belt.is_full();

        for (potion_id, name, count) in &potion_info {
            draw_text_at(&format!("{} x{}", name, count), x, y, FONT_SIZE_BODY, TEXT_COLOR);

            // Add to belt button
            if !belt_full && *count > 0 {
                if ui::button("To Belt", x + 250.0, y - 15.0, 90.0, SMALL_BUTTON_HEIGHT, true) {
                    app.potion_belt.add(potion_id);
                    // Reduce count
                    if let Some(p) = app.potions.iter_mut().find(|p| p.potion_id == *potion_id) {
                        p.count -= 1;
                    }
                    app.potions.retain(|p| p.count > 0);
                }
            }

            y += 30.0;
        }
    }

    // Potion belt display
    y += 10.0;
    draw_text_at("-- Potion Belt --", x, y, FONT_SIZE_BODY, ACCENT_COLOR);
    y += 25.0;

    // Extract belt display info
    let belt_info: Vec<(usize, Option<String>, String)> = (0..4).map(|i| {
        let slot = app.potion_belt.slots.get(i).and_then(|s| s.clone());
        let label = match &slot {
            Some(id) => {
                app.data.as_ref()
                    .and_then(|data| data.potions.iter().find(|p| p.id == *id))
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| id.clone())
            }
            None => "[empty]".to_string(),
        };
        (i, slot, label)
    }).collect();

    for (i, slot, label) in &belt_info {
        draw_text_at(&format!("Slot {}: {}", i + 1, label), x, y, FONT_SIZE_SMALL, TEXT_COLOR);

        // Remove from belt
        if slot.is_some() {
            if ui::button("Remove", x + 280.0, y - 12.0, 70.0, SMALL_BUTTON_HEIGHT - 6.0, true) {
                if let Some(potion_id) = app.potion_belt.remove(*i) {
                    // Return to inventory
                    if let Some(p) = app.potions.iter_mut().find(|p| p.potion_id == potion_id) {
                        p.count += 1;
                    } else {
                        app.potions.push(vassnian_engine::inventory::items::PotionStack {
                            potion_id,
                            count: 1,
                        });
                    }
                }
            }
        }

        y += 24.0;
    }

    // Back button
    if ui::button("BACK", PADDING * 2.0, 750.0, 120.0, BUTTON_HEIGHT, true) {
        app.go_back();
    }
}
