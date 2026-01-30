use config::{BalanceConfig, SettlementsConfig};
use world::{PlayerState, World};
use protocol::NotificationData;

pub struct DeathSystem;

impl DeathSystem {
    pub fn check_deaths(
        players: &mut [&mut PlayerState],
        first_settlement: Option<(f64, f64, uuid::Uuid)>,
        balance: &BalanceConfig,
        _settlements: &SettlementsConfig,
    ) -> Vec<(uuid::Uuid, NotificationData)> {
        let mut notifications = Vec::new();

        for player in players.iter_mut().filter(|p| p.spawned).map(|p| &mut **p) {
            if player.vitals.hp <= 0.0 {
                Self::handle_death(player, first_settlement, balance);
                notifications.push((
                    player.id,
                    NotificationData {
                        text: "You have died. Respawning at settlement...".to_string(),
                    },
                ));
            }
        }

        notifications
    }

    fn handle_death(
        player: &mut PlayerState,
        first_settlement: Option<(f64, f64, uuid::Uuid)>,
        balance: &BalanceConfig,
    ) {
        player.stats.deaths += 1;
        player.unspawn();

        // Reset vitals
        let mut vitals = player.vitals;
        vitals.hp = balance.player.max_health;
        vitals.hunger = vitals.max_hunger;
        vitals.temperature = 50.0;
        player.vitals = vitals;

        // Set respawn location
        if let Some((x, y, id)) = first_settlement {
            player.x = x;
            player.y = y;
            player.settlement_id = Some(id);
        }

        // Apply respawn cooldown
        player.respawn_cooldown = 3.0;
    }

    pub fn check_respawns(players: &mut [&mut PlayerState], dt: f64) -> Vec<uuid::Uuid> {
        let mut ready = Vec::new();

        for player in players.iter_mut().filter(|p| !p.spawned && p.respawn_cooldown > 0.0).map(|p| &mut **p) {
            player.respawn_cooldown -= dt;
            if player.respawn_cooldown <= 0.0 {
                ready.push(player.id);
            }
        }

        ready
    }
}
