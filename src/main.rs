use std::{borrow::Borrow, future::IntoFuture};

use ai::mcts_worker::{mcts_worker, MctsInput};
use futures::executor::LocalPool;
use game::{Board, BoardState, Game, GameState, InvalidMoveError, Player, PlayerType, Position};
use leptos::{component, create_action, create_effect, create_signal, ev::click, logging::log, mount_to_body, update, view, Callback, CollectView, IntoSignal, IntoView, ReadSignal, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalWith, SignalWithUntracked};
use rand::{distributions::Alphanumeric, Rng};
use wasm_peers::{one_to_one::NetworkManager, ConnectionType, SessionId};
use web_sys::console;


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
    Host,
    Client
}
#[component]
fn Menu() -> impl IntoView {
    
    let (gamemode, set_gamemode) = create_signal(None);
    if let Ok(search) = web_sys::window().unwrap().location().search() {
        if search != ""  {
            set_gamemode.set(Some(Gamemode::Client));
        }
    }
    let menu = move || {
        match gamemode.get() {
            None => view! {
                <div class="menu">
                    <button on:click=move |_| {set_gamemode.set(Some(Gamemode::Ai))}>Play vs AI</button>
                    <button on:click=move |_| {set_gamemode.set(Some(Gamemode::Host))}>Play Online</button>
                </div>
            },
            Some(Gamemode::Ai) => {
                view! {
                    <div class="post-menu">
                        <Game/>
                    </div>
                }
            },
            Some(Gamemode::Host) => {
                view! {
                    
                    <div class="post-menu">
                        <OnlineGame host=true/>
                    </div>
                }
            },
            Some(Gamemode::Client) => {
                view! {
                    
                    <div class="post-menu">
                        <OnlineGame host=false/>
                    </div>
                }
            }
        }
    };
    view!{ {menu} }
}

const SIGNALING_SERVER_URL: &str = "http://server.row666.com:2001/one-to-one";
const STUN_SERVER_URL: &str = "stun:stun.relay.metered.ca:80";

#[component]            
fn OnlineGame(host: bool) -> impl IntoView {
    if host {
        let (game, set_game) = create_signal(Game::new(Player::X, PlayerType::Local, PlayerType::Online));
        let session_code: String = rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
        let opponent_url = {
            let href = web_sys::window().unwrap().location().href().unwrap();
            format!("{href}?code={}", session_code)
        };
        let session_id = SessionId::new(session_code.clone());
        let mut server = NetworkManager::new(SIGNALING_SERVER_URL, session_id, 
            ConnectionType::StunAndTurn { 
                stun_urls: STUN_SERVER_URL.to_string(), 
                turn_urls: "turn:global.relay.metered.ca:80?transport=tcp".to_string(), 
                username: "575aeee1cd28ff689a1d9f52".to_string(), 
                credential: "wHgTOHX2SFMXgGPD".to_string() 
            }
        ).unwrap();
        let server_clone = server.clone();
        let server_on_message = {
            move |message: String| {
                let pos: Position = serde_json::from_str(message.as_str()).unwrap();
                create_effect(move |_| set_game.update(|game| { game.state.place(pos.0, pos.1).unwrap(); }));
            }
        };
        let server_clone = server.clone();
        let send_move = move |pos: Position| {
            server_clone.send_message(serde_json::to_string(&pos).unwrap().as_str()).unwrap()
        };
        server.start(||{}, server_on_message);

        
        let game_view = {
            let mut out = Vec::with_capacity(9);
            for row in 0..3 {
                for column in 0..3 {
                    let send_move_clone = send_move.clone();
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
                                    PlayerType::Online => send_move_clone(Position((column, row), (mini_column, mini_row))),
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
                <p>Send to your opponent: {opponent_url}</p>
            </div>
            
        }
    } else {
        let (game, set_game) = create_signal(Game::new(Player::X, PlayerType::Online, PlayerType::Local));
        let href = web_sys::window().unwrap().location().search().unwrap();
        let session_code = href.trim_start_matches("?code=").to_string();
        let session_id = SessionId::new(session_code.clone());
        let mut client = NetworkManager::new(
            SIGNALING_SERVER_URL,
            session_id,
            ConnectionType::StunAndTurn { stun_urls: STUN_SERVER_URL.to_string(), 
                turn_urls: "turn:global.relay.metered.ca:80?transport=tcp".to_string(), 
                username: "575aeee1cd28ff689a1d9f52".to_string(), 
                credential: "wHgTOHX2SFMXgGPD".to_string() }
         ).unwrap();

        let client_on_open = ||{};
        let client_clone = client.clone();
        let client_on_message = {
            move |message: String| {
                let pos: Position = serde_json::from_str(message.as_str()).unwrap();
                create_effect(move |_| set_game.update(|game| { game.state.place(pos.0, pos.1).unwrap(); }));
            }
        };
        let client_clone = client.clone();
        let send_move = move |pos: Position| {
            client_clone.send_message(serde_json::to_string(&pos).unwrap().as_str()).unwrap()
        };
        client.start(client_on_open, client_on_message).unwrap();
        
        let game_view = {
        let mut out = Vec::with_capacity(9);
        for row in 0..3 {
            for column in 0..3 {
                let send_move_clone = send_move.clone();
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
                                PlayerType::Online => send_move_clone(Position((column, row), (mini_column, mini_row))),
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
                                PlayerType::Mcts => mcts_action.dispatch(Position((column, row), (mini_column, mini_row))),
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