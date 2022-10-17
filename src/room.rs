use core::fmt;
use crate::{board::Board, state::State, type_helpers::may_deserialize, ContractError};
use cosmwasm_std::{to_vec, Addr, Storage, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
    GameWon { player: Addr },
    Tie,
    InProgess,
}
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Room {
    pub player1: Addr,
    pub player2: Addr,
    pub turn: Addr,
    pub board: Board,
    pub game_state: String,
    pub no_moves: u8,
    pub total_coins_raised: Uint128,
}

impl Room {
    pub fn new(player1: Addr, player2: Addr) -> Self {
        Self {
            player1: player1.clone(),
            player2,
            turn: player1,
            board: Board::new(),
            game_state: GameState::InProgess.to_string(),
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
            game_state: GameState::InProgess.to_string(),
            no_moves: 0,
            total_coins_raised: Uint128::zero(),
        }
    }
    pub fn load_room(room_number: u8, storage: &dyn Storage) -> Option<Room> {
        let key = room_number.to_be_bytes();
        let room_exist = storage.get(&key);
        if room_exist == None {
            return None;
        }
        let room_exist: Room = may_deserialize(&room_exist).unwrap().unwrap();

        Some(room_exist)
    }
    pub fn add_room(
        storage: &mut dyn Storage,
        player1: Addr,
        player2: Addr,
    ) -> Result<u8, ContractError> {
        let new_room = Self::new(player1, player2);
        let mut state = State::load_state(storage);

        let key = [state.room_count];

        storage.set(&key, &to_vec(&new_room)?);
        state.room_count = state.room_count + 1;
        state.save_state(storage)?;
        Ok(state.room_count - 1)
    }
    pub fn save_room(
        room_index: u8,
        storage: &mut dyn Storage,
        room: Room,
    ) -> Result<(), ContractError> {
        let key = room_index.to_be_bytes();
        let room_exist = Self::load_room(room_index.clone(), storage);

        if room_exist == None {
            return Err(ContractError::RoomError {
                val: "Room does not exist".to_string(),
            });
        }

        storage.set(&key, &to_vec(&room)?);
        Ok(())
    }
}
