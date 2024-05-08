use core::fmt;


pub struct Game {
    pub mini_boards: [[Board; 3]; 3],
    pub meta_board: [[BoardState; 3]; 3],
    pub next_meta_move: Option<(usize, usize)>
}
impl Game {
    pub fn new() -> Self {
        Game {
            mini_boards: [[Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()]],
            meta_board: [[BoardState::Ongoing; 3]; 3],
            next_meta_move: None
        }
    }

    

    pub fn get_icon(&self, meta_pos: (usize, usize), mini_pos: (usize, usize)) -> &'static str {
        const BIG_X: [[&str; 3]; 3] = [
            ["\\", " ", "/"],
            [" ", "X", " "],
            ["/", " ", "\\"],
            ];
        const BIG_O: [[&str; 3]; 3] = [
            ["/", "-", "\\"],
            ["|", "O", "|"],
            ["\\", "-", "/"],
            ];
        match self.meta_board[meta_pos.1][meta_pos.0] {
            BoardState::Ongoing | BoardState::Concluded(BoardResult::Tie) => {
                match self.mini_boards[meta_pos.1][meta_pos.0].get_space(mini_pos) {
                    BoardSpace::Empty => " ",
                    BoardSpace::Taken(Player::X) => "X",
                    BoardSpace::Taken(Player::O) => "O"
                }
            },
            BoardState::Concluded(BoardResult::XWin) => BIG_X[mini_pos.1][mini_pos.0],
            BoardState::Concluded(BoardResult::OWin) => BIG_O[mini_pos.1][mini_pos.0]




        }
    }
    pub fn check_wins(&self, pos: (usize, usize)) -> BoardState {
            let mut row = 0;
            let mut col = 0;
            let mut diag = 0;
            let mut rdiag = 0;
            for i in 0..3 {
                row += match &self.meta_board[pos.1][i] { BoardState::Ongoing => 0, BoardState::Concluded(x) => *x as i32};
                col += match &self.meta_board[i][pos.0] { BoardState::Ongoing => 0, BoardState::Concluded(x) => *x as i32};
                diag += match &self.meta_board[i][i] { BoardState::Ongoing => 0, BoardState::Concluded(x) => *x as i32};
                rdiag += match &self.meta_board[i][2-i] { BoardState::Ongoing => 0, BoardState::Concluded(x) => *x as i32};
            }
            if row == 3 || col == 3 || diag == 3 || rdiag == 3 {
                return BoardState::Concluded(BoardResult::XWin)
            }
            if row == -3 || col == -3 || diag == -3 || rdiag == -3 {
                return BoardState::Concluded(BoardResult::OWin)
            }
            if self.meta_board.iter().flatten().all(|x| matches!(x, BoardState::Concluded(_))) {
                return BoardState::Concluded(BoardResult::Tie)
            }

            BoardState::Ongoing
    }
    pub fn place(&mut self, meta_pos: (usize, usize), mini_pos: (usize, usize), player: Player) -> Result<BoardState, InvalidMoveError> {
        if let Some(next_meta_move) = self.next_meta_move {
            if next_meta_move != meta_pos {
                return Err(InvalidMoveError)
            }
        }

        let mini_result = self.mini_boards[meta_pos.1][meta_pos.0].place(mini_pos, player)?;
        self.meta_board[meta_pos.1][meta_pos.0] = mini_result;

        if let BoardState::Ongoing = self.meta_board[mini_pos.1][mini_pos.0] {
            self.next_meta_move = Some(mini_pos);
        } else {
            self.next_meta_move = None;
        }
        
        if let BoardState::Concluded(result) = mini_result {
            return Ok(self.check_wins(meta_pos))
        }

        Ok(BoardState::Ongoing)
    }
}
pub struct InvalidMoveError;
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for meta_y in 0..=2 {
            for mini_y in 0..=2 {
                for meta_x in 0..=2 {
                    write!(f, " {} | {} | {} ", self.get_icon((meta_x, meta_y), (0, mini_y)), self.get_icon((meta_x, meta_y), (1, mini_y)), self.get_icon((meta_x, meta_y), (2, mini_y)))?;
                    if meta_x != 2 {
                        write!(f,"║")?;
                    }
                }
                if mini_y != 2 {
                    writeln!(f)?;
                    writeln!(f,"---+---+---╫---+---+---╫---+---+---")?;
                }
            }
        if meta_y != 2 {
            writeln!(f)?;
            writeln!(f,"═══════════╬═══════════╬═══════════")?;
        }
        // write!(f,"\n")?;
        }
        Ok(())
    }
}
pub struct Board {
    pub board: [[BoardSpace; 3]; 3],
    x_count: u32,
    o_count: u32,
}


impl Board {
    fn new() -> Self {
        Board {
            board: [[BoardSpace::Empty; 3]; 3],
            x_count: 0,
            o_count: 0
        }
    }
    pub fn get_space(&self, pos: (usize, usize)) -> BoardSpace {
        self.board[pos.1][pos.0]
    }
    fn place(&mut self, pos: (usize, usize), player: Player) -> Result<BoardState, InvalidMoveError> {
        if let BoardSpace::Empty = self.board[pos.1][pos.0] {
            self.board[pos.1][pos.0] = BoardSpace::Taken(player);
            match player {
                Player::X => self.x_count += 1,
                Player::O => self.o_count += 1
            }
            Ok(self.check_wins(pos))
        } else {
            Err(InvalidMoveError)
        }
    }

    pub fn check_wins(&self, pos: (usize, usize)) -> BoardState {
        if self.x_count < 3 && self.o_count < 3 {
            BoardState::Ongoing
        } else {
            let mut row = 0;
            let mut col = 0;
            let mut diag = 0;
            let mut rdiag = 0;
            for i in 0..3 {
                row += match &self.board[pos.1][i] { BoardSpace::Empty => 0, BoardSpace::Taken(x) => *x as i32};
                col += match &self.board[i][pos.0] { BoardSpace::Empty => 0, BoardSpace::Taken(x) => *x as i32};
                diag += match &self.board[i][i] { BoardSpace::Empty => 0, BoardSpace::Taken(x) => *x as i32};
                rdiag += match &self.board[i][2-i] { BoardSpace::Empty => 0, BoardSpace::Taken(x) => *x as i32};
            }
            if row == 3 || col == 3 || diag == 3 || rdiag == 3 {
                return BoardState::Concluded(BoardResult::XWin)
            }
            if row == -3 || col == -3 || diag == -3 || rdiag == -3 {
                return BoardState::Concluded(BoardResult::OWin)
            }
            if self.x_count + self.o_count == 9 {
                return BoardState::Concluded(BoardResult::Tie)
            }

            BoardState::Ongoing

        }
    }
}

#[derive(Copy, Clone)]
pub enum Player {
    X = 1,
    O = -1
}

impl Player {
    pub fn switch(self) -> Player {
        match self {
            Self::X => Self::O,
            Self::O => Self::X
        }
    }
}

#[derive(Copy, Clone)]
pub enum BoardSpace {
    Empty,
    Taken(Player)
}

#[derive(Copy, Clone, Debug)]
pub enum BoardResult {
    XWin = 1,
    Tie = 0,
    OWin = -1
}

#[derive(Copy, Clone, Debug)]
pub enum BoardState {
    Ongoing,
    Concluded(BoardResult)
}