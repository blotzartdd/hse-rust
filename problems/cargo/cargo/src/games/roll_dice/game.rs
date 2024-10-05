#![forbid(unsafe_code)]

use super::config::RollDiceConfig;
use crate::r#trait::{FairRound, InitGame, UnfairRound};

use std::cmp::Ordering;

pub struct RollDiceGame {
    pub config: RollDiceConfig,
    pub last_winner: Option<u8>,
}

pub fn argmax<T>(u: &[T]) -> (usize, T)
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    assert!(!u.is_empty());

    let mut max_index = 0;
    let mut max = u[max_index];
    for (i, v) in u.iter().enumerate().skip(1) {
        if max <= *v {
            max_index = i;
            max = *v;
        }
    }

    (max_index, max)
}

pub fn argmin<T>(u: &[T]) -> (usize, T)
where
    T: Copy + PartialOrd,
{
    assert!(!u.is_empty());

    let mut min_index = 0;
    let mut min = u[min_index];

    for (i, v) in u.iter().enumerate().skip(1) {
        if min >= *v {
            min_index = i;
            min = *v;
        }
    }

    (min_index, min)
}

impl RollDiceGame {
    fn update_game(&mut self) {
        if let Some(winner) = self.last_winner {
            if winner != self.config.players.1 {
                let argmax_proba = argmax(&[
                    self.config.probas[0],
                    self.config.probas[2],
                    self.config.probas[4],
                ]);
                let argmin_proba = argmin(&[
                    self.config.probas[1],
                    self.config.probas[3],
                    self.config.probas[5],
                ]);
                if argmin_proba.1 < argmax_proba.1 {
                    self.config
                        .probas
                        .swap(2 * argmax_proba.0, 2 * argmin_proba.0 + 1);
                }
            } else {
                let argmax_proba = argmax(&[
                    self.config.probas[1],
                    self.config.probas[3],
                    self.config.probas[5],
                ]);
                let argmin_proba = argmin(&[
                    self.config.probas[0],
                    self.config.probas[2],
                    self.config.probas[4],
                ]);
                if argmin_proba.1 < argmax_proba.1 {
                    self.config
                        .probas
                        .swap(2 * argmax_proba.0 + 1, 2 * argmin_proba.0);
                }
            }
        }
    }
}

impl FairRound for RollDiceGame {
    fn play(&mut self) -> u8 {
        let sum0 =
            self.config.probas[0] * 1.0 + self.config.probas[2] * 3.0 + self.config.probas[4] * 5.0;
        let sum1 =
            self.config.probas[1] * 2.0 + self.config.probas[3] * 4.0 + self.config.probas[5] * 6.0;
        let winner = match sum0.partial_cmp(&sum1) {
            None | Some(Ordering::Equal) | Some(Ordering::Greater) => self.config.players.0,
            Some(Ordering::Less) => self.config.players.1,
        };
        self.last_winner = Some(winner);
        winner
    }
}

impl UnfairRound for RollDiceGame {
    fn play(&mut self) -> u8 {
        self.update_game();
        <Self as FairRound>::play(self)
    }
}

impl InitGame<RollDiceConfig> for RollDiceGame {
    fn init(config: RollDiceConfig) -> Self {
        Self {
            config,
            last_winner: None,
        }
    }
}
