use std::time::Duration;

use crate::game::{BoardResult, BoardState, Game, Player, PlayerType};

use crate::ai::mcts::Node;

pub fn main() {
    let mut buf = String::new(); 

    println!("Enter an option:
    1. Local multiplayer
    2. Vs Expected outcome minimax
    3. Vs Monte Carlo Tree Search (thinking time = 3 seconds)
    4. MCTS vs MINIMAX");
    std::io::stdin().read_line(&mut buf).unwrap();
    if let Ok(x) = buf.trim_end().parse::<i32>() {
        if x == 1 {
            vs(PlayerType::Local, PlayerType::Local);
        }
        if x == 2 {
            vs(PlayerType::Local, PlayerType::Minmax);
        }
        if x == 3 {
            vs(PlayerType::Local, PlayerType::Mcts { root: Node::new(), thinking_time: Duration::from_millis(3000)});
        }
        if x == 4 {
            vs(PlayerType::Mcts { root: Node::new(), thinking_time: Duration::from_millis(3000)}, PlayerType::Minmax);
        }

    }
}
fn vs(x: PlayerType, o: PlayerType) {
    let mut game = Game::new(Player::random(), x, o);
    if let BoardState::Concluded(result) = game.state.board_state {
        match result {
            BoardResult::XWin => {
                println!("X wins!");
                return;
            },
            BoardResult::OWin => {
                println!("O wins!");
                return;
            },
            BoardResult::Tie => {
                println!("Tie!");
                return;
            }
        }
    }
    loop {
        
        let meta_move =
        loop {
            let meta_move = {
                if let Some(meta_move) = game.state.next_meta_move {
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
            match &mut game.state.meta_board[meta_move.1][meta_move.0] {
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