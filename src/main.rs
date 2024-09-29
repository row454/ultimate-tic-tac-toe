use std::future::IntoFuture;

use ai::mcts_worker::{mcts_worker, MctsInput};
use futures::executor::LocalPool;
use game::{Board, BoardState, Game, GameState, InvalidMoveError, Player, PlayerType, Position};
use leptos::{component, create_action, create_effect, create_signal, logging::log, mount_to_body, update, view, Callback, CollectView, IntoSignal, IntoView, ReadSignal, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalWith, SignalWithUntracked};
use rand::distributions::Alphanumeric;


mod game;
mod ai;

fn main() {
    if web_sys::window().is_some() {
        console_error_panic_hook::set_once();
        mount_to_body(|| view!{<Menu/>});
    }
}
#[derive(Clone)]
enum Gamemode {
    Ai,
    Online
}
#[component]
fn Menu() -> impl IntoView {
    let (gamemode, set_gamemode) = create_signal(None);
    let menu = move || {
        match gamemode.get() {
            None => view! {
                <div class="menu">
                    <button on:click=move |_| {set_gamemode.set(Some(Gamemode::Ai))}>Play vs AI</button>
                    <button on:click=move |_| {set_gamemode.set(Some(Gamemode::Online))}>Play Online</button>
                </div>
            },
            Some(Gamemode::Ai) => {
                view! {
                    <div class="post-menu">
                        <Game/>
                    </div>
                }
            },
            Some(Gamemode::Online) => {
                view! {
                    
                    <div class="post-menu">
                        <OnlineGame host=true/>
                    </div>
                }
            }
        }
    };
    view!{ {menu} }
}
#[component]
fn OnlineGame(host: bool) -> impl IntoView {
    if host {

    }
}
#[component]
fn Game() -> impl IntoView {

    let (game, set_game) = create_signal(Game::new(Player::X, PlayerType::Local, PlayerType::Mcts));
    let (mcts_sender, mcts_reciever) = mcts_worker().unwrap();
    let mcts_action = create_action(move |pos: &Position| {
        let pos = pos.to_owned();
        let mcts_sender = mcts_sender.clone();
        let mcts_reciever = mcts_reciever.clone();
        async move {
            mcts_sender.send_async(MctsInput::TakeMove { board: game.get_untracked().state, previous_move: pos }).await.unwrap();
            let best_move = mcts_reciever.recv_async().await.unwrap();
            set_game.update(|game| { game.state.place(best_move.0, best_move.1).unwrap(); });
        }
    });
    
    let game_view = {
        let mut out = Vec::with_capacity(9);
        for row in 0..3 {
            for column in 0..3 {
                out.push(view! {<MiniBoard 
                    board = Signal::derive(move || game.with(|game| game.state.mini_boards[row][column].clone()))
                    state = Signal::derive(move || game.with(|game| game.state.meta_board[row][column]))
                    place = Callback::new(move |(mini_row, mini_column)| {
                        set_game.update(|game| { game.state.place((column, row), (mini_column, mini_row)).unwrap(); log!("{:?}", ((column, row), (mini_row, mini_column)))} );
                        game.with_untracked(|game| {
                            match match game.state.turn {
                                Player::X => &game.x,
                                Player::O => &game.o
                            } {
                                PlayerType::Local => (),
                                PlayerType::Mcts => mcts_action.dispatch(((column, row), (mini_column, mini_row))),
                                _ => todo!(),
                            };
                            
                        });
                    })
                    is_active = Signal::derive(move || game.with(|game| game.state.next_meta_move.map(move |pos| pos == (column, row)).unwrap_or(true)
                        && matches!(game.state.meta_board[row][column], BoardState::Ongoing)
                        && matches!(match game.state.turn {
                            Player::X => &game.x,
                            Player::O => &game.o
                            }, PlayerType::Local)
                        )
                    )
                    
                />})
            }
        }
        out
    };
    view! {
        <div class="outer-meta-board">
        {
            move || game.with(|game| match game.state.board_state {
                BoardState::Ongoing => None,
                BoardState::Concluded(result) => {
                    match result {
                        game::BoardResult::XWin => Some(view!{<img class="overlayed-result" src="x.svg"/>}),
                        game::BoardResult::OWin => Some(view!{<img class="overlayed-result" src="o.svg"/>}),
                        game::BoardResult::Tie => None
                    }
                }
            })
        }
            
        <div class="meta-board" class:concluded=move || game.with(|game| matches!(game.state.board_state, BoardState::Concluded(_)))>
            {game_view}
        </div>
    </div>
        
    }

}

#[component]
fn MiniBoard(board: Signal<Board>, state: Signal<BoardState>, #[prop(into)] place: Callback<(usize, usize)>, is_active: Signal<bool>) -> impl IntoView {
    let mut spaces = Vec::with_capacity(9);
    for row in 0..3 {
        for column in 0..3 {
            spaces.push(
                view ! { 
                    <div class="board-space"
                    class:inactive=move || !is_active()
                    class:taken=move || board.with(|board| matches!(board.board[row][column], game::BoardSpace::Taken(_)))
                    on:click=move |_| {
                        place((row, column)); 
                    }>
                        {   
                            move || match board().board[row][column] {
                                game::BoardSpace::Taken(Player::X) => Some(view!{<img src="x.svg"/>}),
                                game::BoardSpace::Taken(Player::O) => Some(view!{<img src="o.svg"/>}),
                                _ => None
                            }
                        }
                        
                    </div>
                }
            );
        }
    }
    view ! {
        <div class="outer-mini-board">
            {
                move || match state.get() {
                    BoardState::Ongoing => None,
                    BoardState::Concluded(result) => {
                        match result {
                            game::BoardResult::XWin => Some(view!{<img class="overlayed-result" src="x.svg"/>}),
                            game::BoardResult::OWin => Some(view!{<img class="overlayed-result" src="o.svg"/>}),
                            game::BoardResult::Tie => None
                        }
                    }
                }
            }
                
            <div class="mini-board"
            class:concluded=move || matches!(state.get(), BoardState::Concluded(_))>
                {spaces}
            </div>
        </div>
    }
}