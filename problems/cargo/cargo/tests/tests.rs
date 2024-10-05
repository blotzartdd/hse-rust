use cargo::config::{get_game, GameConfig};
use cargo::play_games;
use cargo::r#trait::{FairRound, Round, UnfairRound};

use std::io::Read;

fn check_test(test_name: &str, test_data: &serde_json::Value) {
    let input_data: &Vec<serde_json::Value> = test_data.get("input").unwrap().as_array().unwrap();
    let output_data: &Vec<serde_json::Value> = test_data.get("output").unwrap().as_array().unwrap();
    let (mut input_data_vec, mut output_data_vec) = (vec![], vec![]);
    for (input, output) in input_data.iter().zip(output_data.iter()) {
        input_data_vec.push((
            format!("{}", input.get("data").unwrap()),
            input.get("fair_rounds").unwrap().as_u64().unwrap() as usize,
            input.get("unfair_rounds").unwrap().as_u64().unwrap() as usize,
        ));
        output_data_vec.push(if output.is_null() {
            None
        } else {
            Some(output.as_u64().unwrap() as u8)
        });
    }
    let game_winners = play_games(&input_data_vec);
    assert_eq!(game_winners.len(), output_data_vec.len());
    for (test_line, (&winner_opt, &real_winner)) in
        game_winners.iter().zip(output_data_vec.iter()).enumerate()
    {
        assert_eq!(
            winner_opt,
            real_winner,
            "The game is broken for test {test_name} on sample #{}",
            test_line + 1
        );
    }
}

#[test]
fn it_works() {
    let mut file = std::fs::File::open("./tests/tests.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for test_name in ["simple_test"] {
        check_test(test_name, &*json.get(test_name).unwrap());
    }
}
