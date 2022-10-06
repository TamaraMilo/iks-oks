// #[cfg(test)]
// mod tests {
//     use crate::helpers::CwTemplateContract;
//     use crate::msg::InstantiateMsg;
//     use cosmwasm_std::{Addr, Coin, Empty, Uint128};
//     use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

//     pub fn contract_template() -> Box<dyn Contract<Empty>> {
//         let contract = ContractWrapper::new(
//             crate::contract::execute,
//             crate::contract::instantiate,
//             crate::contract::query,
//         );
//         Box::new(contract)
//     }

//     const USER: &str = "USER";
//     const ADMIN: &str = "ADMIN";
//     const NATIVE_DENOM: &str = "denom";

//     fn mock_app() -> App {
//         AppBuilder::new().build(|router, _, storage| {
//             router
//                 .bank
//                 .init_balance(
//                     storage,
//                     &Addr::unchecked(USER),
//                     vec![Coin {
//                         denom: NATIVE_DENOM.to_string(),
//                         amount: Uint128::new(1),
//                     }],
//                 )
//                 .unwrap();
//         })
//     }

//     fn proper_instantiate() -> (App, CwTemplateContract) {
//         let mut app = mock_app();
//         let cw_template_id = app.store_code(contract_template());

//         let msg = InstantiateMsg { count: 1i32 };
//         let cw_template_contract_addr = app
//             .instantiate_contract(
//                 cw_template_id,
//                 Addr::unchecked(ADMIN),
//                 &msg,
//                 &[],
//                 "test",
//                 None,
//             )
//             .unwrap();

//         let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

//         (app, cw_template_contract)
//     }

//     mod count {
//         use super::*;
//         use crate::msg::ExecuteMsg;

//         #[test]
//         fn count() {
//             let (mut app, cw_template_contract) = proper_instantiate();

//             let msg = ExecuteMsg::Increment {};
//             let cosmos_msg = cw_template_contract.call(msg).unwrap();
//             app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
//         }
//     }
// }
// #[cfg(test)]
// mod tests {

//     use crate::contract::{init, query, execute};
//     use crate::helpers::Coordinates;
//     use crate::msg::{InitMsg, QueryMsg, HandleMsg};

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


