use std::u8;

use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};

use crate::{room::GameState, board::Board};


pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub player1: Addr,
    pub player2: Addr,
    pub turn: Addr,
    pub board: Board,
    pub game_state: GameState, 
    pub no_moves: usize,
    pub total_coins_raised: Uint128,
   
}

// pub const STATE: Item<State> = Item::new("state");

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
