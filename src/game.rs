use core::fmt;


pub struct Board {
    pub meta_board: [[BoardState; 3]; 3],
}
impl Board {
    pub fn new() -> Self {
        Board {
            meta_board: [[BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new())],
                    [BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new())],
                    [BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new()), BoardState::Ongoing(SubBoard::new())]
                ]
        }
    }

    pub fn check_wins(&self, pos: (usize, usize)) -> Option<BoardResult> {
        let mut row = 0;
        let mut col = 0;
        let mut diag = 0;
        let mut rdiag = 0;
        for i in 0..3 {
            row += match &self.meta_board[pos.1][i] { BoardState::Ongoing(_) => 0, BoardState::Concluded(x) => *x as i32};
            col += match &self.meta_board[i][pos.0] { BoardState::Ongoing(_) => 0, BoardState::Concluded(x) => *x as i32};
            diag += match &self.meta_board[i][i] { BoardState::Ongoing(_) => 0, BoardState::Concluded(x) => *x as i32};
            rdiag += match &self.meta_board[i][2-i] { BoardState::Ongoing(_) => 0, BoardState::Concluded(x) => *x as i32};
        }
        if row == 3 || col == 3 || diag == 3 || rdiag == 3 {
            return Some(BoardResult::XWin)
        }
        if row == -3 || col == -3 || diag == -3 || rdiag == -3 {
            return Some(BoardResult::OWin)
        }
        return None
}
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for meta_row in 0..2 {
            for row in 0..2 {
                for meta_column in 0..2 {
                    write!(f, "{} | {} | {} \n", self.meta_board[meta_row][meta_column].get_icon((row, 0)), self.meta_board[meta_row][meta_column].get_icon((row, 1)), self.meta_board[meta_row][meta_column].get_icon((row, 2)))?;
                    if meta_column != 2 {
                        write!(f,"║")?;
                    }
                }
                if row != 2 {
                    write!(f,"\n")?;
                    write!(f,"---+---+---╫---+---+---╫---+---+---\n")?;
                }
            }
        if meta_row != 2 {
            write!(f,"\n")?;
            write!(f,"═══════════╬═══════════╬═══════════\n")?;
        }
        write!(f,"\n")?;
        }
        Ok(())
    }
}
pub struct SubBoard {
    pub board: [[Option<Player>; 3]; 3],
    x_count: u32,
    o_count: u32
}


impl SubBoard {
    fn new() -> Self {
        SubBoard {
            board: [[None; 3]; 3],
            x_count: 0,
            o_count: 0
        }
    }

    pub fn place(&mut self, pos: (usize, usize), player: Player) -> Result<Option<BoardResult>, ()> {
        if let None = self.board[pos.1][pos.0] {
            self.board[pos.1][pos.0] = Some(player);
            Ok(self.check_wins(pos))
        } else {
            Err(())
        }
    }

    fn check_wins(&self, pos: (usize, usize)) -> Option<BoardResult> {
        if self.x_count < 3 && self.o_count < 3 {
            return None
        } else {
            let mut row = 0;
            let mut col = 0;
            let mut diag = 0;
            let mut rdiag = 0;
            for i in 0..3 {
                row += match &self.board[pos.1][i] { None => 0, Some(x) => *x as i32};
                col += match &self.board[i][pos.0] { None => 0, Some(x) => *x as i32};
                diag += match &self.board[i][i] { None => 0, Some(x) => *x as i32};
                rdiag += match &self.board[i][2-i] { None => 0, Some(x) => *x as i32};
            }
            if row == 3 || col == 3 || diag == 3 || rdiag == 3 {
                return Some(BoardResult::XWin)
            }
            if row == -3 || col == -3 || diag == -3 || rdiag == -3 {
                return Some(BoardResult::OWin)
            }
            return None

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
pub enum BoardState {
    Ongoing(SubBoard),
    Concluded(BoardResult)
}
impl BoardState {
    fn get_icon(&self, pos: (usize, usize)) -> &'static str {
        if let BoardState::Ongoing(board) = self {
            match board.board[pos.1][pos.0] {
                None => " ",
                Some(Player::X) => "X",
                Some(Player::O) => "O"
            }
        } else if let BoardState::Concluded(result) = self {
            const BIG_X: [[&'static str; 3]; 3] = [
            ["\\", " ", "/"],
            [" ", "X", " "],
            ["/", " ", "\\"],
            ];
            const BIG_O: [[&'static str; 3]; 3] = [
            ["/", "-", "\\"],
            ["|", "O", " "],
            ["\\", "-", "/"],
            ];
            match result {
                BoardResult::XWin => BIG_X[pos.1][pos.0],
                BoardResult::OWin => BIG_O[pos.1][pos.0],
            }
        } else {
            panic!()
        }
        
    }
}

#[derive(Copy, Clone)]
pub enum BoardResult {
    XWin = 1,
    OWin = -1
}