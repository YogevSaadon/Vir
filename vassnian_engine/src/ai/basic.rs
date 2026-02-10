//! Basic AI — attack nearest enemy, heal most wounded

use crate::character::entity::{EntityId, EntityKind};
use crate::combat::battle::{BattleState, CombatAction, CombatUnit};

/// Basic AI: attacks the first living enemy.
pub fn basic_ai_decide(unit_id: EntityId, battle: &BattleState) -> CombatAction {
    let unit = match battle.get_unit(unit_id) {
        Some(u) => u,
        None => return CombatAction::Wait,
    };

    // Determine which side this unit is on
    let targets: Vec<&CombatUnit> = if unit.entity.kind == EntityKind::Enemy {
        battle.living_allies()
    } else {
        battle.living_enemies()
    };

    // Attack the first living target
    if let Some(target) = targets.first() {
        CombatAction::Attack { target_id: target.entity.id }
    } else {
        CombatAction::Wait
    }
}
