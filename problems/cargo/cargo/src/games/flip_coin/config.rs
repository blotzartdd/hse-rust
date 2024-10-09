#![forbid(unsafe_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct FlipCoinConfig {
    pub players_proba: (f64, f64),
    pub players: (u8, u8),
    pub birthday_player: u8,
    pub delta: f64,
}
