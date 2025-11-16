use std::cmp::min;
use crate::hash::{self, get_column};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
pub enum Celltype{
    Constant,
    Arithmetic(char),
    Min,
    Max,
    Sum,
    Avg,
    Stdev,
    Sleep,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Cell{
    pub kind: Celltype,
    pub val: i32,
    pub id: u32,
    pub is_valid: bool,
    pub out_neighbors: Vec<u32>,
    pub op_val: Option<i32>,
    pub cell1: Option<i32>,
    pub cell2: Option<i32>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Sheet{
    pub rows: u32,
    pub cols: u32,
    pub matrix: Vec<Cell>,
    pub is_display: bool,
    pub row_top: u32,
    pub col_top: u32
}

impl Cell{
    fn build_cell(id: u32) -> Cell {
        let out:Vec<u32>= Vec::new(); 
        Cell { kind: Celltype::Constant, val: 0, id, is_valid: true, out_neighbors: out, op_val: None, cell1: None, cell2: None }
    }
    
}
impl Sheet{
    pub fn create_sheet(rows: u32, cols: u32)->Sheet{

        let total = rows * cols;
        let mut cells = Vec::with_capacity(total as usize);
        
        for id in 0..total {
            cells.push(Cell::build_cell(id));
        }

        Sheet { rows, cols, matrix: cells, is_display: true, row_top: 0, col_top: 0 }
         
    }
    pub fn scroll_up(&mut self){
        self.row_top = self.row_top.saturating_sub(10);
    }
    pub fn scroll_down(&mut self){
        self.row_top = min(self.row_top+10, self.rows.saturating_sub(10));
    }
    pub fn scroll_left(&mut self){
        self.col_top = self.col_top.saturating_sub(10);
    }
    pub fn scroll_right(&mut self){
        self.col_top = min(self.col_top+10, self.cols.saturating_sub(10));
    }
    pub fn enable_display(&mut self){
        self.is_display= true;
    }
    pub fn disable_display(&mut self){
        self.is_display= false;
    }
    pub fn scroll_to(&mut self, input: &str){
        let (letters, numbers) = hash::separate_cell(input).unwrap();
        let row = numbers.parse::<u32>().unwrap()-1;
        let col = get_column(&letters)-1;

        self.row_top = row;
        self.col_top = col;

    }
}

