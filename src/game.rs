use core::fmt;
use std::{borrow::BorrowMut, collections::HashSet, default, fmt::Debug, mem, rc::Rc, sync::Arc, time::Duration};

use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::ai::{mcts::{mcts, Node}, minimax_expected_outcome};

pub type Position = ((usize, usize), (usize, usize));

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub mini_boards: [[Board; 3]; 3],
    pub meta_board: [[BoardState; 3]; 3],
    pub next_meta_move: Option<(usize, usize)>,
    pub board_state: BoardState,
    pub turn: Player,
    empty_spaces : HashSet<Position>,
}
#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub x: PlayerType,
    pub o: PlayerType,
}
impl Game {
    pub fn new(starting_player: Player, x: PlayerType, o: PlayerType) -> Self {
        let mut game = Game {
            state: GameState {
            mini_boards: [[Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()]],
            meta_board: [[BoardState::Ongoing; 3]; 3],
            next_meta_move: None,
            turn: starting_player,
            board_state: BoardState::Ongoing,
            empty_spaces: HashSet::from(ALL_SPACES),
            },
            x,
            o,
        };
        game
    }
}

const ALL_SPACES: [Position; 81] = {
    let mut pairs = [(0, 0); 9];
    let mut i = 0;
    let mut j = 0;
    let mut result = [((0, 0), (0, 0)); 81];
    while i < 3 {
        j = 0;
        while j < 3 {
            pairs[i*3+j] = (i, j);
            j += 1
        }
        i += 1
    }
    i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            result[i*9+j] = (pairs[i], pairs[j]);
            j += 1
        }
        i += 1
    }
    result
};


impl GameState {
    pub fn new(starting_player: Player) -> Self {
        GameState {
            mini_boards: [[Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()], [Board::new(), Board::new(), Board::new()]],
            meta_board: [[BoardState::Ongoing; 3]; 3],
            next_meta_move: None,
            turn: starting_player,
            board_state: BoardState::Ongoing,
            empty_spaces: HashSet::from(ALL_SPACES),
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
    pub fn place(&mut self, meta_pos: (usize, usize), mini_pos: (usize, usize)) -> Result<BoardState, InvalidMoveError> {
        if let Some(next_meta_move) = self.next_meta_move {
            if next_meta_move != meta_pos {
                return Err(InvalidMoveError)
            }
        }
        if let BoardState::Concluded(_) = self.meta_board[meta_pos.1][meta_pos.0] {
            return Err(InvalidMoveError)
        }


        let mini_result = self.mini_boards[meta_pos.1][meta_pos.0].place(mini_pos, self.turn)?;
        self.turn = self.turn.switch();
        self.meta_board[meta_pos.1][meta_pos.0] = mini_result;
        assert!(self.empty_spaces.remove(&(meta_pos, mini_pos)));

        if let BoardState::Ongoing = self.meta_board[mini_pos.1][mini_pos.0] {
            self.next_meta_move = Some(mini_pos);
        } else {
            self.next_meta_move = None;
        }
        
        if let BoardState::Concluded(result) = mini_result {
            self.empty_spaces.retain(|(meta, _mini)| meta != &meta_pos);
            self.board_state = self.check_wins(meta_pos);
            return Ok(self.board_state)
        }

        Ok(BoardState::Ongoing)
    }

    pub fn get_possible_moves(&self)  -> HashSet<&Position> {
        if let Some(meta_move) = self.next_meta_move {
            let moves = self.empty_spaces
            .iter()
            .filter(|(meta, _mini)| meta == &meta_move).collect();
            return moves
        }
        return self.empty_spaces.iter().collect();
    }
}
#[derive(Debug)]
pub struct InvalidMoveError;

impl fmt::Display for GameState {
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
#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn random() -> Player {
        let mut rng = thread_rng();
        *[Self::X, Self::O].iter().choose(&mut rng).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoardSpace {
    Empty,
    Taken(Player)
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BoardResult {
    XWin = 1,
    Tie = 0,
    OWin = -1
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BoardState {
    Ongoing,
    Concluded(BoardResult)
}

#[derive(Clone)]
pub enum PlayerType {
    Local,
    Mcts,
    Online,

}
impl std::fmt::Debug for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "Local"),
            Self::Mcts => write!(f, "Mcts"),
            Self::Online => write!(f, "Online"),
        }
    }
}