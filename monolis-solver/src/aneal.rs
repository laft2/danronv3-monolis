use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::time::Instant;

use crate::{
    game::{Coord, Game},
    SEED,
};

pub fn mu(game: &mut Game) {
    let current_t: Vec<Coord> = {
        let mut tmp: Vec<Coord> = vec![];
        for i in 0..11 {
            for j in 0..22 {
                tmp.push(Coord { y: i, x: j });
            }
        }
        tmp
    };
    game.score(&current_t);
}

pub fn anneal(game: &mut Game, end_time: f64, v: Vec<Coord>) -> Vec<Coord> {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let start_time = Instant::now();
    const START_TEMP: f64 = 25.0;
    const END_TEMP: f64 = 1.0;
    let mut current_t: Vec<Coord> = {
        let mut tmp: Vec<Coord> = vec![];
        for i in 0..11 {
            for j in 0..22 {
                tmp.push(Coord { y: i, x: j });
            }
        }
        if v.len() == 11 * 22 {
            tmp = v;
        }
        tmp
    };
    let mut best_t: Vec<Coord> = current_t.clone();
    let mut score = game.score(&current_t);
    let mut best_score = score;

    let mut time = 0.0;
    let mut iter_count = 0;
    while time < end_time {
        // while iter_count < 1{
        let progress_ratio = time / end_time;
        let temp = START_TEMP + (END_TEMP - START_TEMP) * progress_ratio;

        let a = rng.gen_range(0..11 * 22);
        let b = rng.gen_range(0..11 * 22);
        current_t.swap(a, b);
        let score2 = game.score(&current_t);
        let deltascore = score2 - score;
        let prob = (deltascore as f64 / temp).exp();
        if prob > rng.gen() {
            score += deltascore;
            if score > best_score {
                best_score = score;
                best_t = current_t.clone();
                println!("{}", iter_count);
            }
        } else {
            current_t.swap(a, b);
        }
        iter_count += 1;

        let current_time = Instant::now();
        time = (current_time - start_time).as_secs_f64();
    }
    println!("{}", iter_count);
    best_t
}

pub fn ndfs(game: &mut Game, end_time: f64) -> Vec<Coord> {
    let start_time = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut current_t: Vec<Coord> = {
        let mut tmp: Vec<Coord> = vec![];
        for i in 0..11 {
            for j in 0..22 {
                tmp.push(Coord { y: i, x: j });
            }
        }
        tmp
    };
    let mut best_t: Vec<Coord> = current_t.clone();
    let mut score = game.score(&current_t);
    let mut best_score = score;

    let mut time = 0.0;
    let mut iter_count = 0;

    while time < end_time {
        current_t.shuffle(&mut rng);
        score = game.score(&current_t);
        if score > best_score {
            best_score = score;
            best_t = current_t.clone();
            println!("{}", iter_count);
        }
        iter_count += 1;

        let current_time = Instant::now();
        time = (current_time - start_time).as_secs_f64();
    }
    println!("{}", iter_count);
    best_t
}
