use game::{Board, BoardResult, BoardState, Player};

mod game;

fn main() {
    let mut board = Board::new();
    let mut next_meta_move = None;
    let mut current_player = Player::X;

    loop {
        println!("{board}");
        let (meta_move, mut sub_board) =
        loop {
            let meta_move = {
                if let Some(meta_move) = next_meta_move {
                    meta_move
                } else {
                        (
                    loop {
                        
                        println!("Meta move X:"); 
                        let mut buf = String::new(); 
                        std::io::stdin().read_line(&mut buf).unwrap();
                        if let Ok(x) = buf.parse::<usize>() {
                            if x >= 3 {
                                continue;
                            }
                            break x-1;
                        }
                    },
                    loop {
                        
                        println!("Meta move Y:"); 
                        let mut buf = String::new(); 
                        std::io::stdin().read_line(&mut buf).unwrap();
                        if let Ok(y) = buf.parse::<usize>() {
                            if y >= 3 {
                                continue;
                            }
                            break y-1;
                        }
                    }
                    )
                }
            };
            match &mut board.meta_board[meta_move.1][meta_move.0] {
                BoardState::Concluded(_) => {
                    next_meta_move = None;
                    continue
                },
                BoardState::Ongoing(board) => break (meta_move, board)
                
            }
        };

        let result = loop {
            let mini_move = {
                (
                loop {
                    
                    println!("Mini move X:"); 
                    let mut buf = String::new(); 
                    std::io::stdin().read_line(&mut buf).unwrap();
                    if let Ok(x) = buf.parse::<usize>() {
                        if x >= 3 {
                            continue;
                        }
                        break x-1;
                    }
                },
                loop {
                    
                    println!("Mini move Y:"); 
                    let mut buf = String::new(); 
                    std::io::stdin().read_line(&mut buf).unwrap();
                    if let Ok(y) = buf.parse::<usize>() {
                        if y >= 3 {
                            continue;
                        }
                        break y-1;
                    }
                }
                )
            };
            if let Ok(result) = sub_board.place(mini_move, current_player) {
                break result
            } else {
                continue
            }

        };
        



        if let Some(result) = board.check_wins(meta_move) {
            println!("{board}");
            match result {
                BoardResult::XWin => println!("X wins!"),
                BoardResult::OWin => println!("O wins!")
            }
            break
        }
    }
}