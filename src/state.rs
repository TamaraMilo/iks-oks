use std::u8;

use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub exchange_rate: u128,
    pub player1: Addr,
    pub player2: Addr,
    pub turn: u8,
    pub contract_addr: Addr,
    pub table: Vec<String>,
    pub contract_hash: String,
    pub total_raised: Uint128,
    pub played_game: bool, 
    pub winner: u8,
   
}

// pub const STATE: Item<State> = Item::new("state");

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
