extern crate rand;
use game::State;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::aneal::{anneal, mu, ndfs};
use crate::game::Game;

mod aneal;
mod game;

#[allow(dead_code)]
fn get_random_state() -> State {
    let mut rng: _ = ChaCha8Rng::seed_from_u64(SEED);
    let mut state: State = [[1; 22]; 11];
    for i in 0..11 {
        for j in 0..22 {
            state[i][j] = rng.gen_range(1..=4);
        }
    }
    state
}

fn get_score(state: State) -> i32 {
    let mut score = 0;
    for ele in state {
        for ele in ele {
            if ele == 0 {
                score += 1
            }
        }
    }
    score
}

fn output_state(state: &State) {
    for (_i, v) in state.iter().enumerate() {
        for (j, v2) in v.iter().enumerate() {
            let symbol = match v2 {
                1 => "Ｏ",
                2 => "Ｘ",
                3 => "｜",
                4 => "＋",
                _ => "　",
            };
            print!("{}", symbol);
            if j < v.len() - 1 {
                print!("");
            }
        }
        println!();
    }
}

fn read_from_csv(path: &str) -> State {
    let mut ret: State = [[0; 22]; 11];
    let mut v = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();
    for (i, record) in v.byte_records().enumerate() {
        let record = record.unwrap();
        for (j, value) in record.iter().enumerate() {
            let val = value[0] - '0' as u8;
            ret[i][j] = val as i32;
        }
    }
    ret
}

const SEED: u64 = 131;

fn main() {
    let PATH = "../monolis-fetch/colors.csv";
    let test_path = "./test.csv";
    let state: State = read_from_csv(test_path);
    output_state(&state);
    // println!("{:#?}", state);

    let mut game = Game::new(&state);

    mu(&mut game);
    println!("{}", get_score(game.state));

    // let best_t = aneal(&mut game, 180., vec![]);
    // let best_t = ndfs(&mut game, 120.);
    let tmp_t = ndfs(&mut game, 2.);
    let best_t = anneal(&mut game, 20., tmp_t);
    println!("{}", game.score(&best_t));
    output_state(&game.state);

    for ele in game.method(&best_t) {
        println!("({}, {})", ele.y, ele.x);
    }
    println!();
}
