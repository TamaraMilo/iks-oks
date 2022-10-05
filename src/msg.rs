use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::helpers::Coordinates;

#[cw_serde]
pub struct InitMsg {
    pub player1: Addr,
    pub player2: Addr,
}
#[cw_serde(Serialize)]
pub enum HandleMsg {
    PlayMove { coordinates: Coordinates },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TableStatusResponse)]
    TableStatus {},
    #[returns(PlayerTurnResponse)]
    PlayerTurn {},
    #[returns(GameStatusResponse)]
    GameStatus {},
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
