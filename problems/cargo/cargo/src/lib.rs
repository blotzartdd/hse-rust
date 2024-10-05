#![forbid(unsafe_code)]

type Game = Box<dyn Round>;

fn play_game(x: &mut Game, fair_rounds: usize, unfair_rounds: usize) -> Option<u8> {
    // TODO: your code goes here.
    unimplemented!()
}

fn play_games(games: &Vec<(String, usize, usize)>) -> Vec<Option<u8>> {
    // TODO: your code goes here.
    unimplemented!()
}
