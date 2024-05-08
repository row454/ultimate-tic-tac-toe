use std::{sync::atomic::{AtomicI32, Ordering}, thread, time::Instant};

use rand::Rng;

use crate::{game::{BoardResult, BoardState, Game, Player}, random_games};

#[cfg(not(debug_assertions))]
#[test]
fn random_games_1000000() {
    random_games(1000000, 1);

}
#[cfg(not(debug_assertions))]
#[test]
fn random_games_1000000_with_10_threads() {
    random_games(1000000, 10);

}

#[test]
fn random_games_10000() {
    random_games(10000, 1);
}

#[test]
fn random_games_10000_with_10_threads() {
    random_games(10000, 10);
}

