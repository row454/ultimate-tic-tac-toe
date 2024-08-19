use leptos::logging::log;
use leptos_workers::worker;
use serde::{Deserialize, Serialize};

use crate::game::Position;
use crate::GameState;

use super::mcts::{mcts, Node};

#[derive(Clone, Serialize, Deserialize)]
pub enum MctsInput {
    TakeMove {
        board: GameState,
        previous_move: Position 
    }
}
#[worker(MctsWorker)]
pub async fn mcts_worker(
    rx: leptos_workers::Receiver<MctsInput>,
    tx: leptos_workers::Sender<Position>
) {
    

    let mut root = Node::new();
    while let Ok(input) = rx.recv_async().await {
        match input {
            MctsInput::TakeMove { board, previous_move } => {
                root.take_move(previous_move);
                let best_move = mcts(&board, 100, web_time::Duration::from_secs(3), &mut root);
                root.take_move(best_move);
                log!("nodes in tree:{:?}, number of simulations:{:?}, chance of winning:{:?}", root.count_descendants(), root.simulations, root.score as f32/root.simulations as f32);
                tx.send_async(best_move).await.unwrap();
            }
        }
    }
}