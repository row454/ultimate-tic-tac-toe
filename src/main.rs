use std::{sync::atomic::{AtomicI32, Ordering}, thread, time::Instant};

use game::{Game, BoardResult, BoardState, Player};
use rand::Rng;

mod game;
#[cfg(test)]
mod tests;

pub fn random_games(total: u32, thread_count: u32) -> (i32, i32, i32) {
    let before = Instant::now();
    if thread_count < 2 {
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut ties = 0;
        for _ in 0..total {
            match random_game() {
                BoardResult::XWin => x_wins += 1,
                BoardResult::OWin => o_wins += 1,
                BoardResult::Tie => ties += 1
            }
        }

        println!("X: {}, O: {}, Ties: {}", x_wins, o_wins, ties);
        println!("Time: {:.2?}", before.elapsed());
        (x_wins, o_wins, ties)
        
    } else {
        let mut x_wins = AtomicI32::new(0);
        let mut o_wins = AtomicI32::new(0);
        let mut ties = AtomicI32::new(0);
        thread::scope(|s|

            for _ in 0..thread_count {
                s.spawn(||
                    for _ in 0..total/thread_count {
                        match random_game() {
                            BoardResult::XWin => x_wins.fetch_add(1, Ordering::Relaxed),
                            BoardResult::OWin => o_wins.fetch_add(1, Ordering::Relaxed),
                            BoardResult::Tie => ties.fetch_add(1, Ordering::Relaxed),
    
                        };
                    }
                );
            }
        );

        println!("X: {}, O: {}, Ties: {}", x_wins.load(Ordering::Relaxed), o_wins.load(Ordering::Relaxed), ties.load(Ordering::Relaxed));
        println!("Time: {:.2?}", before.elapsed());
        (x_wins.into_inner(), o_wins.into_inner(), ties.into_inner())
    }
    
}

pub fn random_game() -> BoardResult {
    let mut rng = rand::thread_rng();
    let mut game = Game::new();
    let mut current_player = Player::X;
    loop {
        let result = loop {
            let meta_move = {
                if let Some(meta_move) = game.next_meta_move {
                    meta_move
                } else {
                    (rng.gen_range(0..=2),rng.gen_range(0..=2),)
                }
            };
            let mini_move = (rng.gen_range(0..=2),rng.gen_range(0..=2),);
            if let Ok(result) = game.place(meta_move, mini_move, current_player) {
                break result
            } else {
                continue
            }
        };
        match result {
            BoardState::Concluded(x) => return x,
            BoardState::Ongoing => current_player = current_player.switch()
        }

        
    }

}

fn main() {
    let mut buf = String::new(); 

    println!("Enter an option:\n1. Local multiplayer");
    std::io::stdin().read_line(&mut buf).unwrap();
    if let Ok(x) = buf.trim_end().parse::<i32>() {
        if x == 1 {
            solo();
        }

    }
}

fn solo() {
    let mut game = Game::new();
    let mut current_player = Player::X;

    loop {
        println!("{game}");
        let meta_move=
        loop {
            let meta_move = {
                if let Some(meta_move) = game.next_meta_move {
                    meta_move
                } else {
                        (
                    
                    
                    loop {
                        
                        println!("Meta move X:"); 
                        let mut buf = String::new(); 
                        std::io::stdin().read_line(&mut buf).unwrap();
                        if let Ok(x) = buf.trim_end().parse::<usize>() {
                            if 0 == x || x > 3 {
                                continue;
                            }
                            break x-1;
                        }
                    },
                    loop {
                        
                        println!("Meta move Y:"); 
                        let mut buf = String::new(); 
                        std::io::stdin().read_line(&mut buf).unwrap();
                        if let Ok(y) = buf.trim_end().parse::<usize>() {
                            if 0 == y || y > 3 {
                                continue;
                            }
                            break y-1;
                        }
                    },
                    )
                }
            };
            match &mut game.meta_board[meta_move.1][meta_move.0] {
                BoardState::Concluded(_) => {
                    continue
                },
                BoardState::Ongoing => break meta_move
                
            }
        };

        let result = loop {
            let mini_move = {
                (
                loop {
                    
                    println!("Mini move X:"); 
                    let mut buf = String::new(); 
                    std::io::stdin().read_line(&mut buf).unwrap();
                    if let Ok(x) = buf.trim_end().parse::<usize>() {
                        if 0 == x || x > 3 {
                            continue;
                        }
                        break x-1;
                    }
                },
                loop {
                    
                    println!("Mini move Y:"); 
                    let mut buf = String::new(); 
                    std::io::stdin().read_line(&mut buf).unwrap();
                    if let Ok(y) = buf.trim_end().parse::<usize>() {
                        if 0 == y || y > 3 {
                            continue;
                        }
                        break y-1;
                    }
                },
                
                )
            };
            if let Ok(result) = game.place(meta_move, mini_move, current_player) {
                break result
            } else {
                continue
            }

        };
        match result {
            BoardState::Concluded(BoardResult::XWin) => {
                println!("X wins!");
            },
            BoardState::Concluded(BoardResult::OWin) => {
                println!("O wins!");
            },
            BoardState::Concluded(BoardResult::Tie) => {
                println!("Tie!");
            },
            BoardState::Ongoing => {
                current_player = current_player.switch();
            }


        }
    }
}