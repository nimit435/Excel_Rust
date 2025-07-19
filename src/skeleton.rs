pub enum celltype{
    Constant,
    Arithmetic(char),
    Min,
    Max,
    Sum,
    Avg,
    Stdev,
    Sleep
}
pub struct cell{
    pub kind: celltype,
    pub val: i32,
    pub id: u32,
    pub is_valid: bool,
    pub op_val: Option<i32>,
    pub cell1: Option<i32>,
    pub cell2: Option<i32>
}
struct sheet{
    rows: u32,
    cols: u32,
    is_display: bool,
    row_top: u32,
    col_top: u32
}
