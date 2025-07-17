enum celltype{
    Constant,
    Arithmetic(char),
    Min,
    Max,
    Sum,
    Avg,
    Stdev,
    Sleep
}
struct cell{
    kind: celltype,
    val: i32,
    id: u32,
    is_valid: bool,
    op_val: Option<i32>,
    cell1: Option<u32>,
    cell2: Option<u32>
}
struct sheet{
    rows: u32,
    cols: u32,
    is_display: bool,
    row_top: u32,
    col_top: u32
}
