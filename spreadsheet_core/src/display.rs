use crate::hash;
use crate::skeleton::Sheet;
use std::cmp::min;
use std::fmt::Write;

pub fn get_sheet_as_string(sheet: &Sheet)-> String{
	let mut output = String::new();

    let colt = sheet.col_top;
    let rowt = sheet.row_top;
    let numcols = sheet.cols;
    let numrows= sheet.rows;

    write!(output, "   ").unwrap();
    for j in colt..min(colt + 10, numcols) {
        write!(output, "{:>12}", hash::col_mapping(j+1)).unwrap();
    }
    writeln!(output).unwrap();
    for i in rowt..min(rowt+10, numrows){
        write!(output, "{:>3}", i+1).unwrap();
        for j in colt..min(colt+10, numcols){
            let cell = &sheet.matrix[(i*numcols+j)as usize];
            if cell.is_valid {
                write!(output, "{:>12}", cell.val).unwrap();
            }
            else{
                write!(output, "         ERR").unwrap();
            }
        }
        writeln!(output).unwrap();
    }
    output
    
}   

