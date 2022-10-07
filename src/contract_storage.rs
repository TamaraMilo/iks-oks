
use crate::type_helpers::{may_deserialize};

use cosmwasm_std::{Storage, Addr, to_vec};
use crate::{ContractError, room::Room};

pub struct ContractStorage {}

impl ContractStorage
{

    pub fn add_room(name: String, storage: &mut dyn Storage, player1: Addr, player2: Addr)-> Result<(),ContractError>
    {
        let key = name.as_bytes();
        let room_exist =Self::load_room(name.clone(),storage)?;
        if room_exist != None {
            return Err(ContractError::RoomError { val: "Room already exists.".to_string() });
        }

        let new_room = Room::new(player1,player2);
        storage.set(key, &to_vec(&new_room)?);

        Ok(())
    }
    pub fn load_room(name: String, storage: &mut dyn Storage) -> Result<Option<Room>, ContractError>
    {
        let key = name.as_bytes();
        let room_exist = storage.get(key);
        if room_exist != None {
            return Err(ContractError::RoomError { val: "Room already exists.".to_string() });
        }
        let room_exist = may_deserialize(&room_exist).map_err(|_|return ContractError::RoomError { val: "Room deserialization error".to_string() })?;
        Ok(room_exist)
    }
    pub fn save_room(name: String, storage: &mut dyn Storage, room: Room) -> Result<(), ContractError>
    {
        let room_exist = Self::load_room(name.clone(), storage)?;
        if room_exist != None 
        {
            return Err(ContractError::RoomError { val: "Room does not exist".to_string() });
        }
        let key = name.as_bytes();
        storage.set(key, &to_vec(&room)?);
        Ok(())
    }
}   