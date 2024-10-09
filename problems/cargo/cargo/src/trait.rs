#![forbid(unsafe_code)]

pub trait FairRound {
    fn play(&mut self) -> u8;
}

pub trait UnfairRound {
    fn play(&mut self) -> u8;
}

pub trait Round: FairRound + UnfairRound {}
impl<T> Round for T where T: FairRound + UnfairRound {}

pub trait InitGame<GameConfig> {
    fn init(config: GameConfig) -> Self;
}

pub fn init_game<Game, GameConfig>(config: GameConfig) -> Game
where
    Game: InitGame<GameConfig>,
{
    Game::init(config)
}
