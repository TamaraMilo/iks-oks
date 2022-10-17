
use crate::board::Board;

use crate::cell::Coordinates;
use crate::error::ContractError;
use crate::msg::{
    GameStatusResponse, HandleMsg, InitMsg, PlayerTurnResponse, QueryMsg, RoomExistResponse,
    TableStatusResponse, ListPageResponse,
};
use crate::room::{ Room, GameState};
use crate::state::State;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    SubMsg, Uint128,
};
use paginate::Pages;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    State::init_state(deps.storage)?;
    Ok(Response::default())
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response, ContractError> {
    match msg {
        HandleMsg::PlayMove { coordinates, room_number } => play_move(deps, info, coordinates, room_number),
        HandleMsg::RestartGame { room_number } => restart_game(deps, room_number),
        HandleMsg::AddRoom {
            player1,
            player2,
        } => add_room(deps, player1, player2),
    }
}

pub fn play_move(
    deps: DepsMut,
    info: MessageInfo,
    coordinates: Coordinates,
    room_number: u8,
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

    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Err(ContractError::RoomError {
            val: "Room does not exist.".to_string(),
        });
    }
    let mut room = room.unwrap();

    let mut state = State::load_state(deps.storage);
    
    if room.game_state != GameState::InProgess.to_string()|| room.no_moves == 9 {
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
        }.to_string();
        let raised_coins = room.total_coins_raised.u128() / 2 ;
        let coins = Uint128::from(raised_coins);
        state.balance = state.balance + coins;
        let bankmsg = BankMsg::Send {
            to_address: room.turn.to_string(),
            amount: vec![Coin {
                amount: coins,
                denom: "ioc".to_string(),
            }],
        };
        state.save_state(deps.storage)?;
        let submsg = SubMsg::new(bankmsg);
        response.messages.push(submsg)
    }
    
    if room.no_moves == 9 && room.game_state == GameState::InProgess.to_string() {
        room.game_state = GameState::Tie.to_string()
    }

    if room.turn == room.player1 {
        room.turn = room.player2.clone();
    } else {
        room.turn = room.player1.clone();
    }

    Room::save_room(room_number, deps.storage, room.clone())?;

    response = response.set_data(to_binary(&room.board).unwrap());
    Ok(response)
}
pub fn add_room(

    deps: DepsMut,
    player1: Addr,
    player2: Addr,
) -> Result<Response, ContractError> {
    let room_number = Room::add_room(deps.storage, player1, player2)?;
    let response = Response::default();
    let response = response.set_data(to_binary(&room_number)?);
    Ok(response)
}

pub fn restart_game(deps: DepsMut, room_number: u8) -> Result<Response, ContractError> {
    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Err(ContractError::RoomError {
            val: "Room does not exist.".to_string(),
        });
    }
    let mut room = room.unwrap();

    room = room.restart_game();
    Room::save_room(room_number, deps.storage, room.clone())?;

    let mut response = Response::default();
    response = response.set_data(to_binary(&room.board).unwrap());
    Ok(response)
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BoardStatus { room_number } => to_binary(&query_table_status(deps, room_number)?),
        QueryMsg::PlayerTurn { room_number } => to_binary(&query_player_turn(deps, room_number)?),
        QueryMsg::GameStatus { room_number } => to_binary(&query_game_status(deps, room_number)?),
        QueryMsg::RoomExist { room_number } => to_binary(&query_room_exist(deps, room_number)?),
        QueryMsg::RoomList { page_number } => to_binary(&query_list_rooms(deps,page_number)?)
    }
}
pub fn query_table_status(deps: Deps, room_number: u8) -> StdResult<TableStatusResponse> {
    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    let status = room.board.draw_board();

    Ok(TableStatusResponse { status })
}
pub fn query_player_turn(deps: Deps, room_number: u8) -> StdResult<PlayerTurnResponse> {
    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    Ok(PlayerTurnResponse { turn: room.turn })
}
pub fn query_game_status(deps: Deps, room_number: u8) -> StdResult<GameStatusResponse> {
    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Err(cosmwasm_std::StdError::NotFound {
            kind: "Room not found!".to_string(),
        });
    }
    let room = room.unwrap();
    let status = String::new();

    Ok(GameStatusResponse { status })
}
pub fn query_list_rooms(deps: Deps, page_number: u8) -> StdResult<ListPageResponse> 
{
    let  mut list_rooms: Vec<Option<Room>> = Vec::new();
    let state = State::load_state(deps.storage);
    let pages = Pages::new(state.room_count.into(), 10);
    let page = pages.with_offset(page_number.into());
    for i in page.start..page.end
    {
        let room = Room::load_room(i.try_into().unwrap(), deps.storage);
        list_rooms.push(room);
    }
    Ok(ListPageResponse{
        list_rooms
    })
}

pub fn query_room_exist(deps: Deps, room_number: u8) -> StdResult<RoomExistResponse> {
    let room = Room::load_room(room_number, deps.storage);
    if room == None {
        return Ok(RoomExistResponse { room_exist: false });
    }
    Ok(RoomExistResponse { room_exist: true })
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{coins, Addr, MemoryStorage, OwnedDeps};
    fn init() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier>
    {
        let mut deps = mock_dependencies();
        let res = State::init_state(&mut deps.storage);
        assert!(res.is_ok());
        deps
    }

    fn add_room_init() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps= init();
        let info = mock_info("info", &coins(10, "ioc"));

        let msg = HandleMsg::AddRoom {
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
        query(deps.as_ref(), mock_env(), QueryMsg::PlayerTurn {  room_number: 0 }).unwrap();
    }
    #[test]
    fn room_exist_query_test() {
        let deps = add_room_init();
        query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::RoomExist {
             room_number:0,
            },
        )
        .unwrap();
    }

    #[test]
    fn table_status_query_test() {
        let deps = add_room_init();

        query(deps.as_ref(), mock_env(), QueryMsg::BoardStatus { room_number: 0 }).unwrap();
    }

    #[test]
    fn game_status_query_test() {
        let deps = add_room_init();

        query(deps.as_ref(), mock_env(), QueryMsg::GameStatus { room_number: 0 }).unwrap();
    }

    #[test]
    fn play_move_test() {
        let mut deps = add_room_init();

        let coordinates = Coordinates::new(1, 1);

        let msg = HandleMsg::PlayMove { coordinates, room_number: 0 };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());

        assert!(play_move.is_ok());
    }

    #[test]
    fn play_same_move_test() {
        let mut deps = add_room_init();

        let coordinates = Coordinates::new(1, 1);

        let msg = HandleMsg::PlayMove { coordinates, room_number: 0 };
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

        let coordinates = Coordinates::new(1, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            room_number: 0
        };
        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());

        let coordinates = Coordinates::new(1, 2);
        let msg = HandleMsg::PlayMove {
            coordinates,
            room_number: 0
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info, msg);
        assert!(play_move.is_err());
    }
    #[test]
    fn restart_game() {
        let mut deps = add_room_init();


        let player1_info = mock_info("player1", &coins(10, "ioc"));
        let player2_info = mock_info("player2", &coins(10, "ioc"));
        let coordinates = Coordinates::new(1, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            room_number: 0
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_ok());
        let coordinates = Coordinates::new(2, 1);
        let msg = HandleMsg::PlayMove {
            coordinates,
            room_number: 0
        };
        let play_move = execute(deps.as_mut(), mock_env(), player2_info.clone(), msg.clone());
        assert!(play_move.is_ok());

        let restart = HandleMsg::RestartGame { room_number: 0};
        let restart_game = execute(deps.as_mut(), mock_env(), player1_info.clone(), restart);

        assert!(restart_game.is_ok());
    }
    #[test]
    fn play_game_in_not_existing_room_test() {
        let mut deps = add_room_init();

        let player1_info = mock_info("player1", &coins(10, "ioc"));

        let coordinates = Coordinates::new(2, 0);
        let msg = HandleMsg::PlayMove {
            coordinates,
            room_number: 1,
        };
        let play_move = execute(deps.as_mut(), mock_env(), player1_info.clone(), msg.clone());
        assert!(play_move.is_err())
    }

    #[test]
    fn play_winning_game_test() {
        let mut deps = add_room_init();

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
                room_number: 0
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
                room_number: 0
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
            room_number: 0
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
