
use std::{panic::panic_any, result, sync::{Mutex, RwLock}};


use game::{Board, BoardState, Game, GameState, InvalidMoveError, Player, PlayerType};
use yew::prelude::*;

mod game;
mod ai;

#[cfg(feature = "cli")]
mod cli;

#[cfg(feature = "cli")]
fn main() {
    cli::main();
}

#[cfg(not(feature = "cli"))]
fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    
    let game = use_state(|| RwLock::from(Game::new(Player::random(), PlayerType::Local, PlayerType::Local)));
    let update = use_force_update();
    let game_html = {
        
        game.read().unwrap()
        .state.mini_boards.iter().flatten()
        .zip(game.read().unwrap()
            .state.meta_board.iter().flatten())
        .enumerate()
        .map(|(i, (b, s))| {
                let game_clone = game.clone();
                html! {
                    <MiniBoard 
                    board={b.clone()} 
                    state={*s} 
                    place={
                       let update = update.clone();
                        Callback::from(move |pos| {
                        
                        let result = game_clone.try_write().unwrap().place((i%3, i/3),  pos);
                        update.force_update();
                        result
                    })
                    } 
                    is_active={
                        game.read().unwrap().state.next_meta_move.map(|pos| pos == (i%3, i/3)).unwrap_or(true) && matches!(s, BoardState::Ongoing)
                    }
                    />
                }
            }
        )
        .collect::<Html>()
    };
    html! {
        <div class="meta-board">
            {game_html}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MiniBoardProps {
    board: Board,
    state: BoardState,
    place: Callback<(usize, usize), Result<BoardState, InvalidMoveError>>,
    is_active: bool
}

#[function_component(MiniBoard)]
fn mini_board(MiniBoardProps {board, state, place, is_active} : &MiniBoardProps) -> Html {
    let tiles = board.board.iter().flatten().enumerate().map(|(i, t)| {
        let place = place.clone();
        let mut class = String::from("board-space");
        if *is_active {
            class.push_str(" active");
        } else {
            class.push_str(" inactive");
        }

        match t {
        game::BoardSpace::Empty => {
            if *is_active {
                html ! { 
                    <div class={class} 
                        onclick={ 
                            Callback::from(move |_| {
                                place.emit((i%3, i/3)); 
                                use_force_update();
                            })
                        }
                    />
                }
                
            }
            else {
                html ! {<div class={class}/>}
            }
        },
        game::BoardSpace::Taken(x) => {
            class.push_str(" taken");
            html ! {
                <div class={class}> {
                    match x {
                        Player::X => html!{<img src="x.svg"/>},
                        Player::O => html!{<img src="o.svg"/>},
                    }
                } </div>
                
            }
        },
        
        }
    }).collect::<Html>();

    let mut class = String::from("mini-board");
    if matches!(state, BoardState::Concluded(_)) {
        class.push_str(" concluded")
    }

    html ! {
        <div class="outer-mini-board">
            {
                match state {
                    BoardState::Ongoing => html!{},
                    BoardState::Concluded(result) => {
                        match result {
                            game::BoardResult::XWin => html!{<img class="overlayed-result" src="x.svg"/>},
                            game::BoardResult::OWin => html!{<img class="overlayed-result" src="o.svg"/>},
                            game::BoardResult::Tie => html!{}
                        }
                    }
                }
            }
                
            <div class={class}>
                {tiles}
            </div>
        </div>
    }
}
