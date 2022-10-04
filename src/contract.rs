#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::KeyDeserialize;

use crate::error::ContractError;
use crate::msg::{
    GameStatusRespons, HandleMsg, InitMsg, PlayerTurnResponse, QueryMsg, TableStatusResponse,
};
use crate::state::{config, config_read, State};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn init(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    let state = State {
        player1: msg.player1.clone(),
        player2: msg.player2,
        turn: msg.player1,
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
        game_ended: false,
        winner: Addr::from_slice(b"")?,
    };

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response, ContractError> {
    match msg {
        HandleMsg::PlayMove { place } => playmove(deps, info, place),
    }
}

pub fn playmove(
    deps: DepsMut,
    info: MessageInfo,
    place: Vec<u8>,
) -> Result<Response, ContractError> {
    let mut storage = config(deps.storage).load()?;
    if storage.game_ended == true {
        return Err(ContractError::CustomError {
            val: "Game ended.".to_string(),
        });
    }
    if storage.turn != info.sender {
        return Err(ContractError::CustomError {
            val: "It's not your turn!".to_string(),
        });
    }

    let mut simbol = "".to_string();

    if storage.turn == storage.player1 {
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
    storage.table[k] = simbol;

    if check_for_win(storage.table.clone()) {
        storage.game_ended = true;
        storage.winner = storage.turn.clone();
    }

    if storage.turn == storage.player1 {
        storage.turn = storage.player2.clone();
    } else {
        storage.turn = storage.player1.clone();
    }

    config(deps.storage).save(&storage)?;

    let mut response = Response::default();
    response = response.set_data(to_binary(&storage.table).unwrap());
    Ok(response)
}

pub fn check_for_win(table: Vec<String>) -> bool {
    let main_diagonal = table[0].clone();
    let mut main_win = true;
    let anti_diagonal = table[2].clone();
    let mut anti_win = true;

    for i in 0..3 {
        if main_diagonal != table[4 * i] || main_diagonal == "-".to_string() {
            main_win = false;
        }
        if anti_diagonal != table[i * 2 + 2] || anti_diagonal == "-".to_string() {
            anti_win = false;
        }

        if table[i] == table[i + 3] && table[i] == table[i + 6] && table[i] != "-".to_string() {
            return true;
        }
        if table[3 * i] == table[i * 3 + 1]
            && table[3 * i] == table[3 * i + 2]
            && table[3 * i] != "-".to_string()
        {
            return true;
        }
    }

    if main_win || anti_win {
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

    for i in 0..3 {
        let mut row = vec![];
        for j in 0..3 {
            let k = i * 3 + j;
            row.push(storage.table[k].clone());
        }
        table += &row.join(" | ").to_string();
        table += "\x20";
    }

    Ok(TableStatusResponse { table })
}
pub fn query_player_turn(deps: Deps) -> StdResult<PlayerTurnResponse> {
    let storage = config_read(deps.storage).load()?;
    Ok(PlayerTurnResponse { turn: storage.turn })
}
pub fn query_game_status(deps: Deps) -> StdResult<GameStatusRespons> {
    let storage = config_read(deps.storage).load()?;
    let mut status = String::new();
    if storage.game_ended {
        if storage.winner == "" {
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
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{coins, from_binary, Addr, MemoryStorage, OwnedDeps};

    fn initialization() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InitMsg {
            player1: Addr::unchecked("player1"),
            player2: Addr::unchecked("player2"),
        };
        let info = mock_info("creator", &coins(10000, "ioc"));

        init(deps.as_mut(), mock_env(), info, msg).unwrap();
        deps
    }

    #[test]
    fn proper_initialization() {
        initialization();
    }

    #[test]
    fn player_turn_query_test() {
        let deps = initialization();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::PlayerTurn {}).unwrap();
        let value: PlayerTurnResponse = from_binary(&res).unwrap();
        assert_eq!("player1", value.turn);
    }

    #[test]
    fn table_status_query_test() {
        let deps = initialization();
        query(deps.as_ref(), mock_env(), QueryMsg::TableStatus {}).unwrap();
    }

    #[test]
    fn game_status_query_test() {
        let deps = initialization();
        query(deps.as_ref(), mock_env(), QueryMsg::GameStatus {}).unwrap();
    }

    #[test]
    fn play_move_test() {
        let mut deps = initialization();
        let place: Vec<u8> = vec![1, 1];

        let msg = HandleMsg::PlayMove { place };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());
    }

    #[test]
    fn play_same_move_test() {
        let mut deps = initialization();
        let place: Vec<u8> = vec![1, 1];
        let msg = HandleMsg::PlayMove { place };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg.clone());
        assert!(play_move.is_ok());
        let play_move = execute(deps.as_mut(), mock_env(), player2_info, msg);
        assert!(play_move.is_err())
    }
    #[test]
    fn play_same_player_test() {
        let mut deps = initialization();
        let place: Vec<u8> = vec![1, 1];
        let msg = HandleMsg::PlayMove { place };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());
        let place: Vec<u8> = vec![1, 2];
        let msg = HandleMsg::PlayMove { place };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg);
        assert!(play_move.is_err())
    }
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
