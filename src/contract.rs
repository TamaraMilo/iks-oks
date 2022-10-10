use crate::board::Board;
use crate::cell::Coordinates;
use crate::contract_storage::ContractStorage;
use crate::error::ContractError;
use crate::msg::{
    GameStatusResponse, HandleMsg, InitMsg, PlayerTurnResponse, QueryMsg, TableStatusResponse,
};
use crate::room::GameState;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    SubMsg, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn init(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response, ContractError> {
    match msg {
        HandleMsg::PlayMove { coordinates, name } => play_move(deps, info, coordinates, name),
        HandleMsg::RestartGame { name } => restart_game(deps, name),
        HandleMsg::AddRoom {
            name,
            player1,
            player2,
        } => add_room(name, deps, player1, player2),
    }
}

pub fn play_move(
    deps: DepsMut,
    info: MessageInfo,
    coordinates: Coordinates,
    name: String,
) -> Result<Response, ContractError> {
    let mut sent_coins = Uint128::zero();
    for coin in info.funds.iter() {
        if coin.denom != "ioc" {
            return Err(ContractError::CustomError {
                val: "Only ioc is supported. Invlid token sent.".to_string(),
            });
        }
        sent_coins = sent_coins + coin.amount;
    }

    if sent_coins.is_zero() {
        return Err(ContractError::CustomError {
            val: "No coins sent".to_string(),
        });
    }

    let room = ContractStorage::load_room(name.clone(), deps.storage);
    if room == None {
        return Err(ContractError::RoomError {
            val: "Room does not exist.".to_string(),
        });
    }
    let mut room = room.unwrap();
    if room.game_state != GameState::InProgess || room.no_moves == 9 {
        return Err(ContractError::CustomError {
            val: "Game ended.".to_string(),
        });
    }
    if room.turn != info.sender {
        return Err(ContractError::CustomError {
            val: "It's not your turn or you missed a room.".to_string(),
        });
    }
    let mut sign = "".to_string();

    if room.turn == room.player1 {
        sign = "X".to_string();
    } else {
        sign = "O".to_string();
    }

    if !room
        .board
        .occupy_cell(room.turn.clone(), coordinates.clone(), sign)
    {
        return Err(ContractError::CustomError {
            val: "Spot is occupied".to_string(),
        });
    }
    room.no_moves = room.no_moves + 1;

    room.total_coins_raised = room.total_coins_raised + sent_coins;
    let mut response = Response::default();
    if room.board.check_for_win(coordinates.clone()) {
        room.game_state = GameState::GameWon {
            player: room.turn.clone(),
        };
        let bankmsg = BankMsg::Send {
            to_address: room.turn.to_string(),
            amount: vec![Coin {
                amount: room.total_coins_raised,
                denom: "ioc".to_string(),
            }],
        };
        let submsg = SubMsg::new(bankmsg);
        response.messages.push(submsg)
    }
    if room.no_moves == 9 && room.game_state == GameState::InProgess {
        room.game_state = GameState::Tie
    }

    if room.turn == room.player1 {
        room.turn = room.player2.clone();
    } else {
        room.turn = room.player1.clone();
    }

    ContractStorage::save_room(name, deps.storage, room.clone())?;

    response = response.set_data(to_binary(&room.board).unwrap());
    Ok(response)
}
pub fn add_room(
    name: String,
    deps: DepsMut,
    player1: Addr,
    player2: Addr,
) -> Result<Response, ContractError> {
    let room_exist = ContractStorage::load_room(name.clone(), deps.storage);
    if room_exist != None {
        return Err(ContractError::RoomError {
            val: "Room already exist".to_string(),
        });
    }
    ContractStorage::add_room(name, deps.storage, player1, player2)?;

    let response = Response::default();
    Ok(response)
}

pub fn restart_game(deps: DepsMut, name: String) -> Result<Response, ContractError> {
    let room = ContractStorage::load_room(name.clone(), deps.storage);
    if room == None {
        return Err(ContractError::RoomError {
            val: "Room does not exist.".to_string(),
        });
    }
    let mut room = room.unwrap();

    room.board = Board::new();
    room.no_moves = 0;
    room.game_state = GameState::InProgess;
    room.total_coins_raised = Uint128::zero();
    ContractStorage::save_room(name, deps.storage, room.clone())?;

    let mut response = Response::default();
    response = response.set_data(to_binary(&room.board).unwrap());
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TableStatus { name } => to_binary(&query_table_status(deps, name)?),
        QueryMsg::PlayerTurn { name } => to_binary(&query_player_turn(deps, name)?),
        QueryMsg::GameStatus { name } => to_binary(&query_game_status(deps, name)?),
    }
}
pub fn query_table_status(deps: Deps, name: String) -> StdResult<TableStatusResponse> {
    let room = ContractStorage::load_room(name, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    let status = room.board.draw_board();

    Ok(TableStatusResponse { status })
}
pub fn query_player_turn(deps: Deps, name: String) -> StdResult<PlayerTurnResponse> {
    let room = ContractStorage::load_room(name, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    Ok(PlayerTurnResponse { turn: room.turn })
}
pub fn query_game_status(deps: Deps, name: String) -> StdResult<GameStatusResponse> {
    let room = ContractStorage::load_room(name, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    let mut status = String::new();
    if room.game_state == GameState::InProgess {
        status = format!("Game is still in progress. It's player {} turn.", room.turn);
    } else {
        if room.game_state == GameState::Tie {
            status = String::from("Game ended in a tie;")
        } else {
            let player = room.game_state as crate::room::GameState;
            status = format!("Game ended. Player {:?} won!", player.to_owned())
        }
    }

    Ok(GameStatusResponse { status })
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{coins, Addr, MemoryStorage, OwnedDeps};

    fn add_room_init() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let info = mock_info("info", &coins(10, "ioc"));

        let msg = HandleMsg::AddRoom {
            name: "room".to_string(),
            player1: Addr::unchecked("player1"),
            player2: Addr::unchecked("player2"),
        };
        let room = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(room.is_ok());
        deps
    }

    #[test]
    fn add_room_test() {
        add_room_init();
    }

    #[test]
    fn player_turn_query_test() {
        let deps = add_room_init();
        let name = "room".to_string();
        query(deps.as_ref(), mock_env(), QueryMsg::PlayerTurn { name }).unwrap();
    }

    #[test]
    fn table_status_query_test() {
        let deps = add_room_init();
        let name = "room".to_string();
        query(deps.as_ref(), mock_env(), QueryMsg::TableStatus { name }).unwrap();
    }

    #[test]
    fn game_status_query_test() {
        let deps = add_room_init();
        let name = "room".to_string();
        query(deps.as_ref(), mock_env(), QueryMsg::GameStatus { name }).unwrap();
    }

    #[test]
    fn play_move_test() {
        let mut deps = add_room_init();
        let name = "room".to_string();
        let coordinates = Coordinates::new(1, 1);

        let msg = HandleMsg::PlayMove { coordinates, name };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());

        assert!(play_move.is_ok());
    }

    #[test]
    fn play_same_move_test() {
        let mut deps = add_room_init();
        let name = "room".to_string();
        let coordinates = Coordinates::new(1, 1);

        let msg = HandleMsg::PlayMove { coordinates, name };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg.clone());
        assert!(play_move.is_ok());

        let play_move = execute(deps.as_mut(), mock_env(), player2_info, msg);
        assert!(play_move.is_err())
    }

    #[test]
    fn play_same_player_test() {
        let mut deps = add_room_init();
        let name = "room".to_string();
        let coordinates = Coordinates::new(1, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            name: name.clone(),
        };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());

        let coordinates = Coordinates::new(1, 2);
        let msg = HandleMsg::PlayMove {
            coordinates,
            name: name.clone(),
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg);
        assert!(play_move.is_err());
    }
    #[test]
    fn restart_game() {
        let mut deps = add_room_init();
        let name = "room".to_string();

        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));
        let coordinates = Coordinates::new(1, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            name: name.clone(),
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());
        let coordinates = Coordinates::new(2, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            name: name.clone(),
        };
        let play_move = execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
        assert!(play_move.is_ok());

        let restart = HandleMsg::RestartGame { name: name.clone() };
        let restart_game = execute(deps.as_mut(), mock_env(), player1_info.clone(), restart);

        assert!(restart_game.is_ok());
    }
    #[test]
    fn play_game_in_not_existing_room_test() {
        let mut deps = add_room_init();
        let name = "some_room".to_string();
        let player1_info = mock_info("player1", &coins(10, "ioc"));

        let coordinates = Coordinates::new(2, 0);
        let msg = HandleMsg::PlayMove {
            coordinates,
            name: name.clone(),
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_err())
    }

    #[test]
    fn play_winning_game_test() {
        let mut deps = add_room_init();
        let name = "room".to_string();
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));
        let all_coordinates: Vec<Coordinates> = vec![
            Coordinates::new(0, 0),
            Coordinates::new(0, 1),
            Coordinates::new(1, 0),
            Coordinates::new(1, 1),
            Coordinates::new(2, 0),
            Coordinates::new(2, 1),
        ];

        for i in 0..all_coordinates.len() {
            let msg = HandleMsg::PlayMove {
                coordinates: all_coordinates[i].clone(),
                name: name.clone(),
            };
            if i % 2 == 0 {
                let play_move =
                    execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
                assert!(play_move.is_ok());
            } else {
                let play_move =
                    execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
                if i != 5 {
                    assert!(play_move.is_ok())
                } else {
                    assert!(play_move.is_err());
                }
            }
        }
    }

    #[test]
    fn play_tie_game_test() {
        let mut deps = add_room_init();
        let name = "room".to_string();
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));

        let all_coordinates: Vec<Coordinates> = vec![
            Coordinates::new(0, 0),
            Coordinates::new(0, 1),
            Coordinates::new(0, 2),
            Coordinates::new(2, 0),
            Coordinates::new(2, 1),
            Coordinates::new(2, 2),
            Coordinates::new(1, 0),
            Coordinates::new(1, 1),
            Coordinates::new(1, 2),
        ];

        for i in 0..all_coordinates.len() {
            let msg = HandleMsg::PlayMove {
                coordinates: all_coordinates[i].clone(),
                name: name.clone(),
            };
            if i % 2 == 0 {
                let play_move =
                    execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
                assert!(play_move.is_ok());
            } else {
                let play_move =
                    execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
                assert!(play_move.is_ok())
            }
        }
        let msg = HandleMsg::PlayMove {
            coordinates: all_coordinates[0].clone(),
            name,
        };
        let play_move = execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
        assert!(play_move.is_err());
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
