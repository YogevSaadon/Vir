//! Shop screen — buy potions and equipment

use crate::app::{App, GameScreen};
use crate::rendering::*;
use crate::ui;

pub fn update(_app: &mut App) {
    // Input handled in draw
}

pub fn draw(app: &mut App) {
    let shop_id = match &app.current_shop_id {
        Some(id) => id.clone(),
        None => {
            draw_centered_text("No shop loaded", 400.0, FONT_SIZE_BODY, TEXT_DIM);
            return;
        }
    };

    let shop = match &app.data {
        Some(data) => match data.shops.get(&shop_id) {
            Some(s) => s.clone(),
            None => return,
        },
        None => return,
    };

    // Header
    draw_centered_text(&shop.name, 50.0, FONT_SIZE_HEADER, ACCENT_COLOR);
    draw_text_at(&format!("Gold: {}", app.gold), PADDING * 2.0, 90.0, FONT_SIZE_BODY, ACCENT_COLOR);

    let item_x = PADDING * 2.0;
    let item_w = DESIGN_WIDTH - PADDING * 4.0;
    let item_h = 80.0;
    let gap = 10.0;
    let mut y = 120.0;

    // Extract item display info first to avoid borrow issues
    struct ItemDisplay {
        item_id: String,
        name: String,
        description: String,
        buy_price: i32,
        stock: i32,
        item_type: String,
        bonus_text: String,
    }

    let mut items_to_display: Vec<ItemDisplay> = Vec::new();

    for shop_item in &shop.items {
        let stock = app.shop_stock.get(&shop_item.item_id).copied().unwrap_or(0);

        if shop_item.item_type == "potion" {
            if let Some(data) = &app.data {
                if let Some(potion) = data.potions.iter().find(|p| p.id == shop_item.item_id) {
                    items_to_display.push(ItemDisplay {
                        item_id: shop_item.item_id.clone(),
                        name: potion.name.clone(),
                        description: potion.description.clone(),
                        buy_price: potion.buy_price,
                        stock,
                        item_type: "potion".to_string(),
                        bonus_text: String::new(),
                    });
                }
            }
        } else if shop_item.item_type == "equipment" {
            if let Some(data) = &app.data {
                if let Some(equip) = data.equipment.iter().find(|e| e.id == shop_item.item_id) {
                    items_to_display.push(ItemDisplay {
                        item_id: shop_item.item_id.clone(),
                        name: equip.name.clone(),
                        description: equip.description.clone(),
                        buy_price: equip.buy_price,
                        stock,
                        item_type: "equipment".to_string(),
                        bonus_text: format!("+{} STR", equip.stat_bonuses.strength),
                    });
                }
            }
        }
    }

    for item in &items_to_display {
        draw_panel(item_x, y, item_w, item_h);
        draw_text_at(&item.name, item_x + PADDING, y + 22.0, FONT_SIZE_BODY, TEXT_COLOR);
        draw_text_at(&item.description, item_x + PADDING, y + 42.0, FONT_SIZE_SMALL, TEXT_DIM);

        if item.item_type == "equipment" {
            draw_text_at(
                &format!("{}g  {}  (Stock: {})", item.buy_price, item.bonus_text, item.stock),
                item_x + PADDING, y + 62.0, FONT_SIZE_SMALL, ACCENT_COLOR,
            );
        } else {
            draw_text_at(
                &format!("{}g  (Stock: {})", item.buy_price, item.stock),
                item_x + PADDING, y + 62.0, FONT_SIZE_SMALL, ACCENT_COLOR,
            );
        }

        let can_buy = app.gold >= item.buy_price && item.stock > 0;
        if ui::button("BUY", item_x + item_w - 70.0, y + 20.0, 60.0, SMALL_BUTTON_HEIGHT, can_buy) {
            app.gold -= item.buy_price;
            *app.shop_stock.entry(item.item_id.clone()).or_insert(0) -= 1;

            if item.item_type == "potion" {
                if let Some(existing) = app.potions.iter_mut().find(|p| p.potion_id == item.item_id) {
                    existing.count += 1;
                } else {
                    app.potions.push(vassnian_engine::inventory::items::PotionStack {
                        potion_id: item.item_id.clone(),
                        count: 1,
                    });
                }
            } else {
                app.owned_equipment.push(vassnian_engine::inventory::items::EquipmentInstance {
                    equipment_id: item.item_id.clone(),
                });
            }
            app.auto_save();
        }

        y += item_h + gap;
    }

    // Leave button
    if ui::button("LEAVE", PADDING * 2.0, 750.0, 120.0, BUTTON_HEIGHT, true) {
        app.current_shop_id = None;
        app.go_to(GameScreen::Story);
        app.story_text_progress = 0.0;
    }
}
