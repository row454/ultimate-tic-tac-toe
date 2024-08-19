use futures::{SinkExt, StreamExt};
use yew_agent::reactor::{reactor, ReactorScope};
use web_time::Duration;
use crate::game::{GameState, Position};

use super::mcts::{mcts, Node};

pub struct MctsInput {
    pub game_state: GameState,
    pub last_move: Position
}
impl std::fmt::Debug for MctsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MctsInput").field("game_state", &"Omitted").field("last_move", &self.last_move).finish()
    }
}

// output position of this must be placed, otherwise the ai gets out of sync with the board.
#[reactor(MctsReactor)]

pub async fn mcts_reactor(mut scope: ReactorScope<MctsInput, Position>) {
    let mut tree = Node::new();
    while let Some(MctsInput {game_state, last_move}) = scope.next().await {
        tree.take_move(last_move);
        let mcts_move = mcts(&game_state, 100, Duration::from_secs(3), &mut tree);
        tree.take_move(mcts_move);
        if scope.send(mcts_move).await.is_err() {
            return;
        };
    }
}