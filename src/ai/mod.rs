use std::{sync::atomic::{AtomicI32, Ordering}, thread, time::Instant};
use rand::seq::IteratorRandom;

use crate::game::{BoardResult, BoardState, Player, GameState};


pub mod mcts;

pub fn minimax_expected_outcome(depth: u32, limit: u32, game: &GameState, player: Player, random_count: u32) -> (i32, (usize, usize), (usize, usize)) {
    let before = Instant::now();
    // println!("called with: limit = {}, depth = {}", limit, depth);

    let moves = game.get_possible_moves();


    let mut best_move = (-i32::MAX * player as i32, (0,0), (0,0));
    for move_ in moves {
        let mut game = game.clone();
        match game.place(move_.0, move_.1).unwrap() {
            BoardState::Ongoing => (),
            BoardState::Concluded(x) => return (x as i32*i32::MAX, move_.0, move_.1)
        }
        if depth + 1 < limit {
            let (score, _, _) = minimax_expected_outcome(depth + 1, limit, &game, player.switch(), random_count);
            if (score > best_move.0) != (matches!(player, Player::O)) {
                best_move = (score, move_.0, move_.1);
            }
        } else {
            let result = random_games(random_count, 10, &game);
            let score = result.0 - result.1;
            // println!("{score}");
            if (score > best_move.0) != (matches!(player, Player::O)) {
                
                best_move = (score, move_.0, move_.1);
            }
        }
    }
    // println!("{}'s best move is: {:?}. Depth = {}", player as i32, best_move, depth);
    if depth == 0 {
        // println!("{:.2?}, at limit {limit}", before.elapsed())
    }
    best_move
    

}

pub fn random_game(starting_board: &GameState) -> BoardResult {
    let mut rng = rand::thread_rng();
    let mut game = starting_board.clone();
    


    loop {
        let move_ = game.get_possible_moves().into_iter().choose(&mut rng).unwrap();
        let result = game.place(move_.0, move_.1).unwrap();
        match result {
            BoardState::Concluded(x) => return x,
            BoardState::Ongoing => ()
        }

        
    }
    

}
pub fn random_games(total: u32, thread_count: u32, starting_board: &GameState) -> (i32, i32, i32) {
    //#[cfg(test)]
    //#[cfg(debug_assertions)]
    //let before = Instant::now();
    if thread_count < 2 {
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut ties = 0;
        for _ in 0..total {
            match random_game(starting_board) {
                BoardResult::XWin => x_wins += 1,
                BoardResult::OWin => o_wins += 1,
                BoardResult::Tie => ties += 1
            }
        }
        //#[cfg(test)]
        //#[cfg(debug_assertions)]
        //println!("X: {}, O: {}, Ties: {}", x_wins, o_wins, ties);
        //#[cfg(test)]
        //#[cfg(debug_assertions)]
        //println!("Time: {:.2?}", before.elapsed());
        (x_wins, o_wins, ties)
        
    } else {
        let x_wins = AtomicI32::new(0);
        let o_wins = AtomicI32::new(0);
        let ties = AtomicI32::new(0);
        thread::scope(|s|

            for _ in 0..thread_count {
                s.spawn(|| {
                    for _ in 0..total/thread_count {
                        match random_game(starting_board) {
                            BoardResult::XWin => x_wins.fetch_add(1, Ordering::Relaxed),
                            BoardResult::OWin => o_wins.fetch_add(1, Ordering::Relaxed),
                            BoardResult::Tie => ties.fetch_add(1, Ordering::Relaxed),
    
                        };
                    }
                }
                );
            }
        );
        //#[cfg(test)]
        //#[cfg(debug_assertions)]
        //println!("X: {}, O: {}, Ties: {}", x_wins.load(Ordering::Relaxed), o_wins.load(Ordering::Relaxed), ties.load(Ordering::Relaxed));
        //#[cfg(test)]
        //#[cfg(debug_assertions)]
        //println!("Time: {:.2?}", before.elapsed());

        (x_wins.into_inner(), o_wins.into_inner(), ties.into_inner())
    }
    
}