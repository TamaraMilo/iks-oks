use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::board::Board;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum GameState {
    GameWon { player: Addr },
    Tie,
    InProgess,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Room {
    pub player1: Addr,
    pub player2: Addr,
    pub turn: Addr,
    pub board: Board,
    pub game_state: GameState,
    pub no_moves: usize,
    pub total_coins_raised: Uint128,
}

impl Room {
    pub fn new(player1: Addr, player2: Addr) -> Self {
        Self {
            player1: player1.clone(),
            player2,
            turn: player1,
            board: Board::new(),
            game_state: GameState::InProgess,
            no_moves: 0,
            total_coins_raised: Uint128::zero(),
        }
    }
    pub fn restart_game(&self) -> Self {
        Self {
            player1: self.player1.clone(),
            player2: self.player2.clone(),
            turn: self.player1.clone(),
            board: self.board.restart_board(),
            game_state: GameState::InProgess,
            no_moves: 0,
            total_coins_raised: Uint128::zero(),
        }
    }
}
