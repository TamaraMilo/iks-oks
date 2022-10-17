use crate::{type_helpers::may_deserialize, ContractError};
use cosmwasm_std::{to_vec, StdError, Storage, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub room_count: u8,
    pub balance: Uint128,
}
impl State {
    pub fn init_state(storage: &mut dyn Storage) -> Result<(), StdError> {
        let state_exist = storage.get(CONFIG_KEY);
        if state_exist != None {
            return Ok(());
        }
        let state = Self {
            room_count: 0,
            balance: Uint128::zero(),
        };
        storage.set(CONFIG_KEY, &to_vec(&state)?);
        Ok(())
    }

    pub fn load_state(storage: &dyn Storage) -> Self {
        let state = storage.get(CONFIG_KEY);
        let state: Self = may_deserialize(&state).unwrap().unwrap();
        state
    }

    pub fn save_state(&self, storage: &mut dyn Storage) -> Result<(), ContractError> {
        let state = storage.get(CONFIG_KEY);
        if state == None {
            return Err(ContractError::StateError {
                val: "State does not exist.".to_string(),
            });
        }
        storage.set(CONFIG_KEY, &to_vec(self)?);
        Ok(())
    }
}
