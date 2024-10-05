#![forbid(unsafe_code)]

trait FairRound {
    fn play(&mut self) -> u8;
}

trait UnfairRound {
    fn play(&mut self) -> u8;
}

trait Round: FairRound + UnfairRound {}
impl<T> Round for T where T: FairRound + UnfairRound {}

trait InitGame<GameConfig> {
    fn init(config: GameConfig) -> Self;
}

fn init_game<Game, GameConfig>(config: GameConfig) -> Game
where Game: InitGame<GameConfig> {
    // TODO: your code goes here.
    unimplemented!()
}
