use crate::{cell::Coordinates, room::Room};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InitMsg;
#[cw_serde(Serialize)]
pub enum HandleMsg {
    PlayMove {
        coordinates: Coordinates,
        room_number: u8,
    },
    RestartGame {
        room_number: u8,
    },
    AddRoom {
        player1: Addr,
        player2: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TableStatusResponse)]
    BoardStatus { room_number: u8 },
    #[returns(PlayerTurnResponse)]
    PlayerTurn { room_number: u8 },
    #[returns(GameStatusResponse)]
    GameStatus { room_number: u8 },
    #[returns(RoomExistResponse)]
    RoomExist { room_number: u8 },
    #[returns(ListPageResponse)]
    RoomList { page_number: u8 },
}

// We define a custom struct for each query response

#[cw_serde(Serialize)]
pub struct TableStatusResponse {
    pub status: String,
}

#[cw_serde(Serialize)]
pub struct PlayerTurnResponse {
    pub turn: Addr,
}
#[cw_serde(Serialize)]
pub struct GameStatusResponse {
    pub status: String,
}
#[cw_serde(Serialize)]
pub struct RoomExistResponse {
    pub room_exist: bool,
}
#[cw_serde(Serialize)]
pub struct ListPageResponse {
    pub list_rooms: Vec<Option<Room>>,
}
