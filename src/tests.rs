use std::{sync::atomic::{AtomicI32, Ordering}, thread, time::{Duration, Instant}};

use rand::{seq::IteratorRandom, Rng};

use crate::{game::{BoardResult, BoardState, Game, Player}, mcts::mcts, minimax_expected_outcome, random_games};

#[cfg(not(debug_assertions))]
#[test]
fn random_games_1000000() {
    random_games(1000000, 1, &Game::new(Player::X));

}
#[cfg(not(debug_assertions))]
#[test]
fn random_games_1000000_with_10_threads() {
    random_games(1000000, 10, &Game::new(Player::X));

}

#[test]
fn random_games_10000() {
    random_games(10000, 1, &Game::new(Player::X));
}

#[test]
fn random_games_10000_with_10_threads() {
    random_games(10000, 10, &Game::new(Player::X));
}

#[test]
fn random_vs_minimax_10() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    for _ in 0..10 {
        match random_vs_minimax() {
            BoardResult::XWin => x_wins += 1,
            BoardResult::OWin => o_wins += 1,
            BoardResult::Tie => ties += 1
        }
        println!("X:{x_wins} O:{o_wins} Tie:{ties}")
    }
    println!("X:{x_wins} O:{o_wins} Tie:{ties}")
}

#[test]
fn random_vs_mcts_10() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    for _ in 0..10 {
        match random_vs_mcts() {
            BoardResult::XWin => x_wins += 1,
            BoardResult::OWin => o_wins += 1,
            BoardResult::Tie => ties += 1
        }
        println!("X:{x_wins} O:{o_wins} Tie:{ties}")
    }
    println!("X:{x_wins} O:{o_wins} Tie:{ties}")
}
#[test]
fn minimax_vs_minimax_5() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    for _ in 0..5 {
        match minimax_vs_minimax() {
            BoardResult::XWin => x_wins += 1,
            BoardResult::OWin => o_wins += 1,
            BoardResult::Tie => ties += 1
        }
        println!("X:{x_wins} O:{o_wins} Tie:{ties}")
    }
    println!("X:{x_wins} O:{o_wins} Tie:{ties}")
}
#[test]
fn minimax1_vs_minimax2_5() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    for _ in 0..5 {
        match minimax1_vs_minimax2() {
            BoardResult::XWin => x_wins += 1,
            BoardResult::OWin => o_wins += 1,
            BoardResult::Tie => ties += 1
        }
        println!("X:{x_wins} O:{o_wins} Tie:{ties}")
    }
    println!("X:{x_wins} O:{o_wins} Tie:{ties}")
}
fn minimax_vs_minimax() -> BoardResult {
    let mut game = Game::new(Player::X);

    loop {
        let (_, meta_move, mini_move) = minimax_expected_outcome(0, 1, &game, game.turn, 1000);
        let result = game.place(meta_move, mini_move).unwrap();
        match result {
            BoardState::Concluded(x) => {
                println!("{game}");
                return x
            },
            BoardState::Ongoing => ()


        }
    }
}
fn minimax1_vs_minimax2() -> BoardResult {
    let mut game = Game::new(Player::X);

    loop {
        let (_, meta_move, mini_move) = minimax_expected_outcome(0, match game.turn { Player::X => 1, Player::O => 2 }, &game, game.turn, match game.turn { Player::X => 1000, Player::O => 100 });
        let result = game.place(meta_move, mini_move).unwrap();
        match result {
            BoardState::Concluded(x) => {
                println!("{game}");
                return x
            },
            BoardState::Ongoing => ()


        }
    }
}
fn random_vs_minimax() -> BoardResult {
    let mut game = Game::new(Player::X);
    let mut rng = rand::thread_rng();

    loop {
        let result;
        if let Player::X = game.turn {
            let (_, meta_move, mini_move) = minimax_expected_outcome(0, 1, &game, game.turn, 1000);
            result = game.place(meta_move, mini_move).unwrap();
        } else {
            let move_ = game.get_possible_moves().into_iter().choose(&mut rng).unwrap();
            result = game.place(move_.0, move_.1).unwrap();
        }
        match result {
            BoardState::Concluded(x) => {
                return x
            },
            BoardState::Ongoing => ()


        }
    }
}
#[test]
fn mcts_vs_minimax_10() {
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;
    for _ in 0..10 {
        match mcts_vs_minimax() {
            BoardResult::XWin => x_wins += 1,
            BoardResult::OWin => o_wins += 1,
            BoardResult::Tie => ties += 1
        }
        println!("X:{x_wins} O:{o_wins} Tie:{ties}")
    }
    println!("X:{x_wins} O:{o_wins} Tie:{ties}")
}
fn mcts_vs_minimax() -> BoardResult {
    let mut game = Game::new(Player::X);

    loop {
        let result;
        if let Player::X = game.turn {
            let (meta_move, mini_move) = mcts(&game, game.turn, 100, Duration::from_millis(500));
            result = game.place(meta_move, mini_move).unwrap();
        } else {
            let (_, meta_move, mini_move) = minimax_expected_outcome(0, 2, &game, game.turn, 1000);
            result = game.place(meta_move, mini_move).unwrap();
        }
        match result {
            BoardState::Concluded(x) => {
                return x
            },
            BoardState::Ongoing => ()


        }
    }
}
fn random_vs_mcts() -> BoardResult {
    let mut game = Game::new(Player::X);
    let mut rng = rand::thread_rng();

    loop {
        let result;
        if let Player::X = game.turn {
            let (meta_move, mini_move) = mcts(&game, game.turn, 100, Duration::from_millis(500));
            result = game.place(meta_move, mini_move).unwrap();
        } else {
            let move_ = game.get_possible_moves().into_iter().choose(&mut rng).unwrap();
            result = game.place(move_.0, move_.1).unwrap();
        }
        match result {
            BoardState::Concluded(x) => {
                return x
            },
            BoardState::Ongoing => ()


        }
    }
}

