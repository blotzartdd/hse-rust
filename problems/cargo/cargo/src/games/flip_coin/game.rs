#![forbid(unsafe_code)]

use super::config::FlipCoinConfig;
use crate::r#trait::{FairRound, InitGame, UnfairRound};

use std::cmp::Ordering;

pub struct FlipCoinGame {
    pub config: FlipCoinConfig,
    pub last_winner: Option<u8>,
}

impl FlipCoinGame {
    fn get_winner(&self) -> u8 {
        match self
            .config
            .players_proba
            .0
            .partial_cmp(&self.config.players_proba.1)
        {
            None | Some(Ordering::Equal) => self.config.birthday_player,
            Some(Ordering::Less) => self.config.players.1,
            Some(Ordering::Greater) => self.config.players.0,
        }
    }
    fn update_game(&mut self, winner: u8, delta: f64) {
        if self.config.players.0 == winner {
            self.config.players_proba.1 += delta;
            self.config.players_proba.0 -= delta;
        } else {
            self.config.players_proba.0 += delta;
            self.config.players_proba.1 -= delta;
        };
    }
}

impl FairRound for FlipCoinGame {
    fn play(&mut self) -> u8 {
        let winner = self.get_winner();
        self.update_game(winner, self.config.delta);
        self.last_winner = Some(winner);
        winner
    }
}

impl UnfairRound for FlipCoinGame {
    fn play(&mut self) -> u8 {
        if let Some(winner) = self.last_winner {
            if winner == self.config.birthday_player {
                self.update_game(winner, 0.2);
            }
        }
        <Self as FairRound>::play(self)
    }
}

impl InitGame<FlipCoinConfig> for FlipCoinGame {
    fn init(config: FlipCoinConfig) -> Self {
        Self {
            config,
            last_winner: None,
        }
    }
}
