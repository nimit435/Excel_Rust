use crate::hash;
use crate::skeleton::Sheet;
use std::cmp::min;
use std::fmt::Write;

pub fn display_sheet(sheet: &Sheet)->String{
    let mut res = String::new();
	if sheet.is_display {
        print!("   ");
        let colt = sheet.col_top;
        let rowt = sheet.row_top;
        let numcols = sheet.cols;
        let numrows= sheet.rows;
        for j in colt..min(colt + 10, numcols) {
            write!(res, "{:>12}", hash::col_mapping(j+1)).unwrap();
        }
        writeln!(res,"").unwrap();
        for i in rowt..min(rowt+10, numrows){
            write!(res,"{:>3}", i+1).unwrap();
            for j in colt..min(colt+10, numcols){
                let cell = &sheet.matrix[(i*numcols+j)as usize];
                if cell.is_valid {
                    write!(res,"{:>12}", cell.val).unwrap();
                }
                else{
                    write!(res,"         ERR").unwrap();
                }
            }
            writeln!(res,"").unwrap();
        }
    }
    res
}   

