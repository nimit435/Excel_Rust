enum celltype{
    Constant,
    Arithmetic(char),
    MIN,
    MAX,
    SUM,
    AVG,
    STDEV,
    SLEEP,
}
struct cell{
    kind: celltype,
    val: i32,
    id: u32,
    is_Valid: bool,
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
