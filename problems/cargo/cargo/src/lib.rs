#![forbid(unsafe_code)]

pub mod config;
mod games;
pub mod r#trait;

use crate::config::{get_game, GameConfig};
use crate::r#trait::{FairRound, Round, UnfairRound};

type Game = Box<dyn Round>;

fn play_game(x: &mut Game, fair_rounds: usize, unfair_rounds: usize) -> Option<u8> {
    if fair_rounds + unfair_rounds == 0 {
        return None;
    }

    let game = x;
    let mut winner = 0;

    for _ in 0..fair_rounds {
        winner = FairRound::play(&mut **game);
    }

    for _ in 0..unfair_rounds {
        winner = UnfairRound::play(&mut **game);
    }

    Some(winner)
}

pub fn play_games(games: &Vec<(String, usize, usize)>) -> Vec<Option<u8>> {
    let mut results: Vec<Option<u8>> = Vec::new();
    for game in games {
        let config: GameConfig = serde_json::from_str::<GameConfig>(&game.0).unwrap();
        let mut x: Game = get_game(config);

        let fair_rounds: usize = game.1;
        let unfair_rounds: usize = game.2;

        results.push(play_game(&mut x, fair_rounds, unfair_rounds));
    }

    results
}
