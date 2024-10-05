#![forbid(unsafe_code)]

use crate::games::{flip_coin::game::FlipCoinGame, roll_dice::game::RollDiceGame};

// TODO: your code goes here.
unimplemented!()
#[serde(tag = "type", content = "config")]
enum GameConfig {
    FlipCoin(FlipCoinConfig),
    RollDice(RollDiceConfig),
}

fn get_game(config: GameConfig) -> Box<dyn Round> {
    let object: Box<dyn Round> = match config {
        GameConfig::FlipCoin(flip_coin_config) => {
            Box::new(init_game::<FlipCoinGame, _>(flip_coin_config))
        }
    };
    object
}
