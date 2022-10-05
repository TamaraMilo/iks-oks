use crate::error::ContractError;
use crate::helpers::{Board, Coordinates, GameState};
use crate::msg::{
    GameStatusResponse, HandleMsg, InitMsg, PlayerTurnResponse, QueryMsg, TableStatusResponse,
};
use crate::state::{config, config_read, State};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn init(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    let board = Board::new();
    let state = State {
        player1: msg.player1.clone(),
        player2: msg.player2,
        turn: msg.player1,
        board,
        game_state: GameState::InProgess,
        no_moves: 0,
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
        HandleMsg::PlayMove { coordinates } => playmove(deps, info, coordinates),
        HandleMsg::RestartGame {  }=> restartgame(deps)
     }
}

pub fn playmove(
    deps: DepsMut,
    info: MessageInfo,
    coordinates: Coordinates,
) -> Result<Response, ContractError> {
    let mut storage = config(deps.storage).load()?;
    if storage.game_state != GameState::InProgess || storage.no_moves == 9 {
        return Err(ContractError::CustomError {
            val: "Game ended.".to_string(),
        });
    }
    if storage.turn != info.sender {
        return Err(ContractError::CustomError {
            val: "It's not your turn!".to_string(),
        });
    }
    let mut sign = "".to_string();

    if storage.turn == storage.player1 {
        sign = "X".to_string();
    } else {
        sign = "O".to_string();
    }

    if !storage
        .board
        .occupy_cell(storage.turn.clone(), coordinates.clone(), sign)
    {
        return Err(ContractError::CustomError {
            val: "Spot is occupied".to_string(),
        });
    }
    storage.no_moves = storage.no_moves + 1;

    if storage.board.check_for_win(coordinates.clone()) {
        storage.game_state = GameState::GameWon {
            player: storage.turn.clone(),
        };
    }
    if storage.no_moves == 9 && storage.game_state == GameState::InProgess {
        storage.game_state = GameState::Tie
    }

    if storage.turn == storage.player1 {
        storage.turn = storage.player2.clone();
    } else {
        storage.turn = storage.player1.clone();
    }

    config(deps.storage).save(&storage)?;

    let mut response = Response::default();
    response = response.set_data(to_binary(&storage.board).unwrap());
    Ok(response)
}

pub fn restartgame(deps: DepsMut) -> Result<Response, ContractError>
{
    let mut storage = config(deps.storage).load()?; 
    
    storage.board = Board::new();
    storage.no_moves = 0;
    storage.game_state = GameState::InProgess;
    config(deps.storage).save(&storage)?;

    let mut response = Response::default();
    response = response.set_data(to_binary(&storage.board).unwrap());
    Ok(response)
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
    let status = storage.board.draw_board();

    Ok(TableStatusResponse { status })
}
pub fn query_player_turn(deps: Deps) -> StdResult<PlayerTurnResponse> {
    let storage = config_read(deps.storage).load()?;
    Ok(PlayerTurnResponse { turn: storage.turn })
}
pub fn query_game_status(deps: Deps) -> StdResult<GameStatusResponse> {
    let storage = config_read(deps.storage).load()?;
    let mut status = String::new();
    if storage.game_state == GameState::InProgess {
        status = format!(
            "Game is still in progress. It's player {} turn.",
            storage.turn
        );
    } else {
        if storage.game_state == GameState::Tie {
            status = String::from("Game ended in a tie;")
        } else {
            let player = storage.game_state as crate::helpers::GameState;
            status = format!("Game ended. Player {:?} won!", player.to_owned())
        }
    }

    Ok(GameStatusResponse { status })
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
//     use cosmwasm_std::{coins, from_binary, Addr, MemoryStorage, OwnedDeps};

//     fn initialization() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
//         let mut deps = mock_dependencies();
//         let msg = InitMsg {
//             player1: Addr::unchecked("player1"),
//             player2: Addr::unchecked("player2"),
//         };
//         let info = mock_info("creator", &coins(10000, "ioc"));

//         init(deps.as_mut(), mock_env(), info, msg).unwrap();
//         deps
//     }

//     #[test]
//     fn proper_initialization() {
//         initialization();
//     }

//     #[test]
//     fn player_turn_query_test() {
//         let deps = initialization();
//         query(deps.as_ref(), mock_env(), QueryMsg::PlayerTurn {}).unwrap();
        
//     }

//     #[test]
//     fn table_status_query_test() {
//         let deps = initialization();
//         query(deps.as_ref(), mock_env(), QueryMsg::TableStatus {}).unwrap();
//     }

//     #[test]
//     fn game_status_query_test() {
//         let deps = initialization();
//         query(deps.as_ref(), mock_env(), QueryMsg::GameStatus {}).unwrap();
//     }

//     #[test]
//     fn play_move_test() {
//         let mut deps = initialization();
//         let coordinates = Coordinates::new(1, 1);

//         let msg = HandleMsg::PlayMove { coordinates };
//         let player1_info = mock_info("player1", &coins(10, "ioc"));
//         let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
//         assert!(play_move.is_ok());
//     }

//     #[test]
//     fn play_same_move_test() {
//         let mut deps = initialization();
//         let coordinates = Coordinates::new(1, 1);

//         let msg = HandleMsg::PlayMove { coordinates };
//         let player1_info = mock_info("player1", &coins(10, "ioc"));
//         let player2_info = mock_info("player2", &coins(10, "ioc"));
//         let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg.clone());
//         assert!(play_move.is_ok());

//         let play_move = execute(deps.as_mut(), mock_env(), player2_info, msg);
//         assert!(play_move.is_err())
//     }

    
//     #[test]
//     fn play_same_player_test() {
//         let mut deps = initialization();
//         let coordinates = Coordinates::new(1, 1);
//         let msg = HandleMsg::PlayMove { coordinates };
//         let player1_info = mock_info("player1", &coins(10, "ioc"));
//         let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
//         assert!(play_move.is_ok());

//         let coordinates = Coordinates::new(1, 2);
//         let msg = HandleMsg::PlayMove { coordinates };
//         let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg);
//         assert!(play_move.is_err());
//     }
//     #[test]
//     fn restart_game()
//     {
//         let mut deps = initialization();

//         let player1_info =mock_info("player1", &coins(10, "ioc"));
//         let player2_info = mock_info("player2", &coins(10, "ioc"));
//         let coordinates = Coordinates::new(1, 1);
//         let msg = HandleMsg::PlayMove { coordinates };
//         let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
//         assert!(play_move.is_ok());
//         let coordinates = Coordinates::new(2, 1);
//         let msg = HandleMsg::PlayMove { coordinates };
//         let play_move = execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
//         assert!(play_move.is_ok());
       

//         let restart = HandleMsg::RestartGame {  };
//         let restart_game = execute(deps.as_mut(),mock_env(), player1_info.clone(), restart);
        
//         assert!(restart_game.is_ok());

        

//     }



//     #[test]
//     fn play_game_test() {
//         let mut deps = initialization();
//         let player1_info = mock_info("player1", &coins(10, "ioc"));
//         let player2_info = mock_info("player2", &coins(10, "ioc"));
//         for i in 0..6 {
//             if i % 2 == 0 {
//                 let coordinates = Coordinates::new(i / 2, 0);
//                 let msg = HandleMsg::PlayMove { coordinates };
//                 let play_move =
//                     execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
//                 assert!(play_move.is_ok());
//             } else {
//                 let coordinates = Coordinates::new(i / 2, 1);
//                 let msg = HandleMsg::PlayMove { coordinates };
//                 let play_move =
//                     execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
//                 if i != 5 {
//                     assert!(play_move.is_ok());
//                 } else {
//                     assert!(play_move.is_err());
//                 }
//             }
           
//         }
     
//     }

//     #[test]
//     fn play_tie_game_test() {
//         let mut deps = initialization();
//         let player1_info = mock_info("player1", &coins(10, "ioc"));
//         let player2_info = mock_info("player2", &coins(10, "ioc"));
//         for i in 0..3 {
//             for j in 0..3 {
//                 let coordinates;
//                 if i == 1 {
//                     coordinates = Coordinates::new(i + 1, j);
//                 } else {
//                     if i == 2 {
//                         coordinates = Coordinates::new(i - 1, j);
//                     } else {
//                         coordinates = Coordinates::new(i.clone(), j.clone());
//                     }
//                 }
//                 let msg = HandleMsg::PlayMove { coordinates };
//                 let play_move;
//                 if (i + j) % 2 == 0 {
//                     play_move =
//                         execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
//                 } else {
//                     play_move =
//                         execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
//                 }

//                 assert!(play_move.is_ok());
//             }
//         }
//         let coordinates = Coordinates::new(0, 0);
//         let msg = HandleMsg::PlayMove { coordinates };
//         let play_move = execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
//         assert!(play_move.is_err());
        
//     }
// }

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
