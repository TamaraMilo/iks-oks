#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{
    GameStatusRespons, HandleMsg, InitMsg, PlayerTurnResponse, QueryMsg, TableStatusResponse,
};
use crate::state::{config, config_read, State};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:iks-oks";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    let state = State {
        player1: msg.player1,
        player2: msg.player2,
        turn: 1,
        contract_addr: msg.token_contract_addr,
        table: vec![
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
        ],

        exchange_rate: msg.token_exchange_rate,
        contract_hash: msg.token_contract_hash,
        total_raised: Uint128::zero(),
        played_game: false,
        winner: 0,
    };

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response, ContractError> {
    match msg {
        HandleMsg::PlayMove { place } => playmove(deps, env, info, msg, place),
    }
}

pub fn playmove(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
    place: Vec<u8>,
) -> Result<Response, ContractError> {
    let storage = config(deps.storage).load()?;
    if storage.turn == 1 {
        if storage.player1 != info.sender {
            return Err(ContractError::CustomError {
                val: "It's not your turn!".to_string(),
            });
        }
    } else {
        if storage.player2 != info.sender {
            return Err(ContractError::CustomError {
                val: "It's not your turn!".to_string(),
            });
        }
    }

    let total_coins_sent: Uint128 = Uint128::zero();
    for coin in info.funds.iter() {
        //Da li ovde stavljam moj naziv tokena
        if coin.denom != "moj???" {
            return Err(ContractError::CustomError {
                val: "Only my coin is suppoted. Invalid coin sent.".to_owned(),
            });
        }
        total_coins_sent = total_coins_sent + coin.amount;
    }
    if total_coins_sent.is_zero() {
        return Err(ContractError::CustomError {
            val: "No coins sent.".to_owned(),
        });
    }

    storage.total_raised = storage.total_raised + total_coins_sent;

    if storage.played_game {
        return Err(ContractError::CustomError {
            val: "Game played already".to_string(),
        });
    }

    let mut simbol = "".to_string();

    if storage.turn == 1 {
        simbol = "X".to_string();
    } else {
        simbol = "O".to_string();
    }

    let k: usize = (place[0] * 3 + place[1]).into();
    if storage.table[k] != "-" {
        return Err(ContractError::CustomError {
            val: "Spot is not empty".to_string(),
        });
    }
    storage.table[k] == simbol;

    if CheckForWin(storage.table.clone()) {
        storage.played_game = true;
        storage.winner = storage.turn;
    }

    if storage.turn == 1 {
        storage.turn = 2;
    } else {
        storage.turn = 1;
    }

    config(deps.storage).save(&storage)?;

    Ok(())
}

pub fn CheckForWin(table: Vec<String>) -> bool {
    let main_diagonal = table[0];
    let mut main_d = true;
    let anti_diagonal = table[2];
    let mut anti_d = true;

    for i in 0..2 {
        if main_diagonal != table[4 * i] {
            main_d = false;
        }
        if anti_diagonal != table[i * 2 + 2] {
            anti_d = false;
        }

        if table[i] == table[i + 3] && table[i] == table[i + 6] {
            return true;
        }
        if table[3 * i] == table[i * 3 + 1] && table[3 * i] == table[3 * i + 2] {
            return true;
        }
    }

    if main_d || anti_d {
        return true;
    }

    return false;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TableStatus {} => to_binary(&query_table_status(deps)?),
        QueryMsg::PlayerTurn {} => to_binary(&query_player_turn(deps)?),
        QueryMsg::GameStatus {} => to_binary(&query_game_status(deps)?),
    }
}
pub fn query_table_status(deps: Deps) -> StdResult<TableStatusResponse> {
    let storage = config_read(deps.storage).load()?;
    let mut table = String::from(" ");

    for i in 0..2 {
        let mut row = vec![];
        for j in 0..2 {
            let k = i * 3 + j;
            row.push(storage.table[k]);
        }
        table += &row.join(" | ").to_string();
        table += "\n";
    }

    Ok(TableStatusResponse { table })
}
pub fn query_player_turn(deps: Deps) -> StdResult<PlayerTurnResponse> {
    let storage = config_read(deps.storage).load()?;
    Ok(PlayerTurnResponse { turn: storage.turn })
}
pub fn query_game_status(deps: Deps) -> StdResult<GameStatusRespons> {
    let storage = config_read(deps.storage).load()?;
    let mut status: String = String::new();
    if storage.played_game {
        if storage.winner == 0 {
            status = String::from("game ended in a tie.");
        } else {
            status = format!("Game ended. Player {} won.", storage.winner);
        }
    } else {
        status = format!(
            "Game is still in progress. It's player {} turn.",
            storage.turn
        );
    }

    Ok(GameStatusRespons { status })
}

// pub fn instantiate(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> Result<Response, ContractError> {
//     let state = State {
//         count: msg.count,
//         owner: info.sender.clone(),
//     };
//     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
//     STATE.save(deps.storage, &state)?;

//     Ok(Response::new()
//         .add_attribute("method", "instantiate")
//         .add_attribute("owner", info.sender)
//         .add_attribute("count", msg.count.to_string()))
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     match msg {
//         ExecuteMsg::Increment {} => execute::increment(deps),
//         ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
//     }
// }

// pub mod execute {
//     use super::*;

//     pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             state.count += 1;
//             Ok(state)
//         })?;

//         Ok(Response::new().add_attribute("action", "increment"))
//     }

//     pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//         STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//             if info.sender != state.owner {
//                 return Err(ContractError::Unauthorized {});
//             }
//             state.count = count;
//             Ok(state)
//         })?;
//         Ok(Response::new().add_attribute("action", "reset"))
//     }
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
//     }
// }

// pub mod query {
//     use super::*;

//     pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
//         let state = STATE.load(deps.storage)?;
//         Ok(GetCountResponse { count: state.count })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies();

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: GetCountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
