use crate::hash;
use crate::skeleton::Sheet;
use std::cmp::min;

pub fn display_sheet(sheet: &Sheet){
	if sheet.is_display {
        print!("   ");
        let colt = sheet.col_top;
        let rowt = sheet.row_top;
        let numcols = sheet.cols;
        let numrows= sheet.rows;
        for j in colt..min(colt + 10, numcols) {
            print!("{:>12}", hash::col_mapping(j+1));
        }
        println!();
        for i in rowt..min(rowt+10, numrows){
            print!("{:>3}", i+1);
            for j in colt..min(colt+10, numcols){
                let cell = &sheet.matrix[(i*numcols+j)as usize];
                if cell.is_valid {
                    print!("{:>12}", cell.val);
                }
                else{
                    print!("         ERR");
                }
            }
            println!();
        }

    }
}   

