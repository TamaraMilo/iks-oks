use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cell::{Cell, Coordinates};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Board {
    pub cells: Vec<Cell>,
}
impl Board {
    pub fn new() -> Self {
        let mut cells = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                let coordinates = Coordinates { x: i, y: j };
                let cell = Cell::new(coordinates);
                cells.push(cell);
            }
        }

        Self { cells }
    }
    pub fn occupy_cell(&mut self, player: Addr, coordinates: Coordinates, sign: String) -> bool {
        let cell_index: usize = coordinates.index().into();
        if self.cells[cell_index].player == None {
            self.cells[cell_index].occupy(player, sign);
            // self.cells[cell_index].player = Some(player);
            // self.cells[cell_index].sign = sign;
            return true;
        }

        false
    }
    pub fn check_for_win(&self, coordinates: Coordinates) -> bool {
        let win_lines: Vec<Vec<[usize; 2]>> = vec![
            vec![[1, 2], [4, 8], [3, 6]],
            vec![[0, 2], [4, 7]],
            vec![[0, 1], [4, 6], [5, 8]],
            vec![[4, 5], [0, 6]],
            vec![[3, 5], [0, 8], [2, 6], [1, 7]],
            vec![[3, 4], [2, 8]],
            vec![[7, 8], [2, 4], [0, 3]],
            vec![[6, 8], [1, 4]],
            vec![[6, 7], [0, 4], [2, 5]],
        ];
        let last_move: usize = coordinates.index().into();
        let player = self.cells[last_move].player.clone();
        for i in 0..win_lines[last_move].len() {
            let line = win_lines[last_move][i];
            if player == self.cells[line[0]].player && player == self.cells[line[1]].player {
                return true;
            }
        }
        false
    }
    pub fn draw_board(&self) -> String {
        let mut board_look = String::from(
            "-------------
        ",
        );
        for i in 0..3 {
            board_look += &"||".to_string();
            for j in 0..3 {
                let cell_index: usize = i * 3 + j;
                board_look += " ";
                board_look += &self.cells[cell_index].sign;
                board_look += " |";
            }
            board_look += &"|".to_string();
            board_look += "--------------
            "
        }

        board_look
    }
    pub fn restart_board(&self) -> Self {
        let mut cells: Vec<Cell> = vec![];
        for index in 0..self.cells.len() {
            cells[index] = self.cells[index].restart();
        }
        Self { cells }
    }
}
