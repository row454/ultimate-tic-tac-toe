use std::{sync::atomic::{AtomicI32, Ordering}, thread, time::{Duration, Instant}};

use game::{BoardResult, BoardSpace, BoardState, Game, Player};
use rand::{seq::IteratorRandom, Rng};

use crate::mcts::Node;

mod game;
mod mcts;
#[cfg(test)]
mod tests;

fn main() {
    let mut buf = String::new(); 

    println!("Enter an option:
    1. Local multiplayer
    2. Vs Expected outcome minimax (max depth 1, using 1000 random games)
    3. Vs Expected outcome minimax (depth 2, 500 games)
    4. Vs Monte Carlo Tree Search (thinking time = 3 seconds)");
    std::io::stdin().read_line(&mut buf).unwrap();
    if let Ok(x) = buf.trim_end().parse::<i32>() {
        if x == 1 {
            solo();
        }
        if x == 2 {
            vs_minimax(1, 1000);
        }
        if x == 3 {
            vs_minimax(2, 500);
        }
        if x == 4 {
            vs_mcts(Duration::from_millis(3000));
        }

    }
}
fn vs_mcts(thinking_time: Duration) {

    println!("You are X!");
    let mut game = Game::new(Player::random());
    let mut root = Node::new();

    loop {
        println!("{game}");
        let result;
        if let Player::O = game.turn {
            let (meta_move, mini_move) = mcts::mcts(&game, 100, thinking_time, &mut root);
            result = game.place(meta_move, mini_move).unwrap();
            root = root.take_move((meta_move, mini_move)).expect("mcts chose this");
        } else {
            let meta_move =
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
            let mut mini_move;
            result = loop {
                mini_move = {
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
                if let Ok(result) = game.place(meta_move, mini_move) {
                    break result
                } else {
                    continue
                }
            };
            if root.has_children() {
                root = root.take_move((meta_move, mini_move)).expect("this should have all possible moves as children")
            } else {
                root = Node::new();
            }
            
        }
        match result {
            BoardState::Concluded(BoardResult::XWin) => {
                println!("X wins!");
                return;
            },
            BoardState::Concluded(BoardResult::OWin) => {
                println!("O wins!");
                return;
            },
            BoardState::Concluded(BoardResult::Tie) => {
                println!("Tie!");
                return;
            },
            BoardState::Ongoing => ()


        }
    }
    
}
fn minimax_expected_outcome(depth: u32, limit: u32, game: &Game, player: Player, random_count: u32) -> (i32, (usize, usize), (usize, usize)) {
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

fn vs_minimax(depth: u32, random_count: u32) {
    println!("You are X!");
    let mut game = Game::new(Player::X);

    loop {
        println!("{game}");
        let result;
        if let Player::O = game.turn {
            let (_, meta_move, mini_move) = minimax_expected_outcome(0, depth, &game, Player::O, random_count);
            result = game.place(meta_move, mini_move).unwrap();
        } else {
            let meta_move =
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

            result = loop {
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
                if let Ok(result) = game.place(meta_move, mini_move) {
                    break result
                } else {
                    continue
                }
            };
        }
        match result {
            BoardState::Concluded(BoardResult::XWin) => {
                println!("X wins!");
                return;
            },
            BoardState::Concluded(BoardResult::OWin) => {
                println!("O wins!");
                return;
            },
            BoardState::Concluded(BoardResult::Tie) => {
                println!("Tie!");
                return;
            },
            BoardState::Ongoing => ()


        }
    }
}

pub fn random_game(starting_board: &Game) -> BoardResult {
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

pub fn random_games(total: u32, thread_count: u32, starting_board: &Game) -> (i32, i32, i32) {
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


fn solo() {
    let mut game = Game::new(Player::X);

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
            if let Ok(result) = game.place(meta_move, mini_move) {
                break result
            } else {
                continue
            }

        };
        match result {
            BoardState::Concluded(BoardResult::XWin) => {
                println!("X wins!");
                return;
            },
            BoardState::Concluded(BoardResult::OWin) => {
                println!("O wins!");
                return;
            },
            BoardState::Concluded(BoardResult::Tie) => {
                println!("Tie!");
                return;
            }, 
            BoardState::Ongoing => ()


        }
    }
}