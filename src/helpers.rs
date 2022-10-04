use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum GameState 
{
    GameWon{player: Addr},
    Tie,
    InProgess
}
#[derive(Serialize,Deserialize,Clone, Debug,PartialEq, Eq,JsonSchema)]
pub struct Coordinates{
    pub x: u8,
    pub y: u8,
}
impl Coordinates
{
    pub fn new(x: u8, y: u8)->Self
    {
        Self { x, y}
    }
}
#[derive(Serialize,Deserialize,Clone, Debug,PartialEq, Eq,JsonSchema)]
pub struct Cell
{
    pub coordinats: Coordinates,
    pub player:Option<Addr>,
    pub sign: String,

}
#[derive(Serialize,Deserialize,Clone, Debug,PartialEq, Eq,JsonSchema)]
pub struct Board {
    pub cells: Vec<Cell>
}
impl Board{
    pub fn new() -> Self
    {
        let mut cells = Vec::new();

        for i in 0..3
        {
            for j in 0..3
            {
                let coordinats  = Coordinates{ x: i, y: j };
                let cell = Cell{
                    coordinats,
                    player: None,
                    sign: String::from(""),
                };
                cells.push(cell);
            }
        }

        Self { cells }
    }
    pub fn occupy_cell(&mut self, player: Addr, coordinats: Coordinates, sign: String) -> bool
    {

        let cell_index: usize = (coordinats.x*3+coordinats.y).into();
        if self.cells[cell_index].player == None
        {
            self.cells[cell_index].player = Some(player);
            self.cells[cell_index].sign = sign;
            return true;
        }
        
        false
        
    }
    pub fn check_for_win(&self, coordinats: Coordinates) -> bool
    {
        let win_lines = vec![
            vec![[1, 2], [4, 8], [3, 6]],
            vec![[0, 2], [4, 7]],
            vec![[0, 1], [4, 6], [5, 8]],
            vec![[4, 5], [0, 6]],
            vec![[3, 5], [0, 8], [2, 6], [1, 7]],
            vec![[3, 4], [2, 8]],
            vec![[7, 8], [2, 4], [0, 3]],
            vec![[6, 8], [1, 4]],
            vec![[6, 7], [0, 4], [2, 5]]
        ];
        let last_move: usize = (coordinats.x*3+coordinats.y).into();
        let player = self.cells[last_move].player.clone();
        for i in 0..win_lines[last_move].len()
        {
            let line = win_lines[last_move][i];
            if player ==  self.cells[line[0]].player && player ==  self.cells[line[1]].player
            {
                return  true;
            }
        }
        false

    }
    pub fn draw_board(&self) -> String
    {
        let mut board_look = String::from("---------\x20"); 
        for i in 0..3
        {
            board_look += &"||".to_string();
            for j in 0..3
            {   
                let cell_index: usize = i*3+j;
                board_look += " ";
                board_look +=&self.cells[cell_index].sign;
                board_look += " |";
                
            }
            board_look += &"|".to_string();
            board_look += "---------\x20"
        }



        board_look
    }
   
}
































// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// use cosmwasm_std::{
//     to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
// };

// use crate::msg::{ExecuteMsg, GetCountResponse, QueryMsg};

// /// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
// /// for working with this.
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct CwTemplateContract(pub Addr);

// impl CwTemplateContract {
//     pub fn addr(&self) -> Addr {
//         self.0.clone()
//     }

//     pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
//         let msg = to_binary(&msg.into())?;
//         Ok(WasmMsg::Execute {
//             contract_addr: self.addr().into(),
//             msg,
//             funds: vec![],
//         }
//         .into())
//     }

//     /// Get Count
//     pub fn count<Q, T, CQ>(&self, querier: &Q) -> StdResult<GetCountResponse>
//     where
//         Q: Querier,
//         T: Into<String>,
//         CQ: CustomQuery,
//     {
//         let msg = QueryMsg::GetCount {};
//         let query = WasmQuery::Smart {
//             contract_addr: self.addr().into(),
//             msg: to_binary(&msg)?,
//         }
//         .into();
//         let res: GetCountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
//         Ok(res)
//     }
// }
