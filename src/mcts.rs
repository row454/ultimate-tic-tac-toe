use std::{collections::HashMap, f64::consts::SQRT_2, time::{Duration, Instant}};


use crate::{game::{BoardState, Game, Player, Position}, random_games};

#[derive(Debug)]
pub struct Node {
    children: HashMap<Position, Node>,
    score: i64,
    simulations: u64
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: HashMap::new(),
            score: 0,
            simulations: 0
        }
    }
    fn new_child(&mut self, action: Position) {
        let child = Node {
            children: HashMap::new(),
            score: 0,
            simulations: 0
        };
        self.children.insert(action, child);

    }
    fn average_score(&self) -> f64 {
        self.score as f64 / self.simulations as f64
    }
    pub fn take_move(mut self, action: Position) -> Option<Node> {
        self.children.remove(&action)
    }
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}
impl Default for Node {
    fn default() -> Self {
        Node::new()
    }
}

pub fn mcts(starting_board: &Game, random_count: u32, thinking_time: Duration, root: &mut Node) -> Position {

    let start = Instant::now();
    if !root.has_children() {
        let moves = starting_board.get_possible_moves().into_iter();
        for move_ in moves {
            root.new_child(*move_)
        }
    }


    loop {

        mcts_iteration(starting_board.clone(), root, random_count);

        if start.elapsed() > thinking_time {
            break;
        }
    }

    let result = root.children.iter()
    .reduce(|max, next| {
        // println!("{:?} {} {}", next.0, next.1.average_score(), next.1.simulations);
        if max.1.average_score() < next.1.average_score() {
            next
        } else { 
            max 
        }
    }).unwrap();
    // println!("got an average score of {} with {} simulated games", result.1.average_score(), result.1.simulations);
    *result.0

}

const EXPLORATION_PARAMETER: f64 = 2f64; //SQRT_2;

fn mcts_iteration(mut game: Game, node: &mut Node, random_count: u32) -> (i64, u64) {

    if node.has_children() {
        let mut max = (f64::NEG_INFINITY, ((0, 0), (0, 0)));
        for (action, child) in node.children.iter_mut() {
            let confidence = if node.simulations == 0 || child.simulations == 0 {
                f64::INFINITY
            } else {
                child.average_score() + EXPLORATION_PARAMETER * ((node.simulations as f64).ln() / child.simulations as f64).sqrt()
            };

            if confidence > max.0 {
                max = (confidence, *action)
            }
        }
        game.place(max.1.0, max.1.1).expect("tried to place from a result of game.get_possible_moves(), and failed");
        let (score, simulations) = mcts_iteration(game, node.children.get_mut(&max.1).unwrap(), random_count);
        
        node.score += score;
        node.simulations += simulations;
        (-score, simulations)
    } else if let BoardState::Concluded(result) = game.board_state {
        node.score += random_count as i64 * result as i64 * game.turn.switch()  as i64;
        node.simulations += random_count as u64;
        (-node.score, node.simulations)
    } else if node.simulations == 0 {
        
        let result = random_games(random_count, 10, &game);
        let score = (result.0 - result.1) * game.turn.switch() as i32;

        node.score += score as i64;
        node.simulations += random_count as u64;
        (-score as i64, random_count as u64)
    } else {
        let mut moves = game.get_possible_moves().into_iter();
        let first = *moves.next().unwrap();
        for move_ in moves {
            node.new_child(*move_)
        }
        node.new_child(first);

        game.place(first.0, first.1).expect("tried to place from a result of game.get_possible_moves(), and failed");
        let (score, simulations) = mcts_iteration(game, node.children.get_mut(&first).unwrap(), random_count);
        node.score += score;
        node.simulations += simulations;
        (-score, simulations)
    }


}