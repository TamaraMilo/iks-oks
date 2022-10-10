use crate::cell::Coordinates;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InitMsg;
#[cw_serde(Serialize)]
pub enum HandleMsg {
    PlayMove {
        coordinates: Coordinates,
        name: String,
    },
    RestartGame {
        name: String,
    },
    AddRoom {
        name: String,
        player1: Addr,
        player2: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TableStatusResponse)]
    TableStatus { name: String },
    #[returns(PlayerTurnResponse)]
    PlayerTurn { name: String },
    #[returns(GameStatusResponse)]
    GameStatus { name: String },
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
