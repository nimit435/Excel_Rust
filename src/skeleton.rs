pub enum Celltype{
    Constant,
    Arithmetic(char),
    Min,
    Max,
    Sum,
    Avg,
    Stdev,
    Sleep
}
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
        let mut out:Vec<u32>= Vec::new(); 
        Cell { kind: Celltype::Constant, val: 0, id: id, is_valid: true, out_neighbors: out, op_val: None, cell1: None, cell2: None }
    }
    
}
impl Sheet{
    pub fn create_sheet(rows: u32, cols: u32)->Sheet{

        let total = rows * cols;
        let mut cells = Vec::with_capacity(total as usize);
        
        for id in 0..total {
            cells.push(Cell::build_cell(id));
        }

        Sheet { rows: rows, cols: cols, matrix: cells, is_display: true, row_top: 0, col_top: 0 }
         
    }
}