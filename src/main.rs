
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
    yew::Renderer::<Game>::new().render();
}
pub enum GameAction {
    Place(game::Position)
}
impl Component for Game {
    type Message = GameAction;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Game::new(Player::random(), PlayerType::Local, PlayerType::Local)
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let game_html = {
            self.state.mini_boards.iter().flatten()
            .zip(self.state.meta_board.iter().flatten())
            .enumerate()
            .map(|(i, (b, s))| {
                    html! {
                        <MiniBoard
                        board={b.clone()} 
                        state={*s} 
                        place={
                            ctx.link().callback(move |pos| GameAction::Place(((i%3, i/3),  pos)))
                        } 
                        is_active={
                            self.state.next_meta_move.map(move |pos| pos == (i%3, i/3)).unwrap_or(true) && matches!(s, BoardState::Ongoing)
                        }
                        />
                    }
                }
            )
            .collect::<Html>()
        };
        let mut class = String::from("meta-board");
        if matches!(self.state.board_state, BoardState::Concluded(_)) {
            class.push_str(" concluded");
        }
        html! {
            <div class="outer-meta-board">
            {
                match self.state.board_state {
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
                {game_html}
            </div>
        </div>
            
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameAction::Place(pos) => {
                self.place(pos.0, pos.1).expect("it should not be possible to send an incorrect message!");
                true
            }
        }
    }
}

#[derive(Properties, PartialEq)]
struct MiniBoardProps {
    board: Board,
    state: BoardState,
    place: Callback<(usize, usize)>,
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
