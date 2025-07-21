use Excel_Rust::{display::display_sheet, parsing, skeleton::Sheet};

fn main() {
    let mut sheet = Sheet::create_sheet(10, 10);
    // display_sheet(&sheet);
    

    // let input = "A1=-4+-4";

    match parsing::parse_input("A1=MAX(B1:J10)", &mut sheet) {
        Ok(_) => display_sheet(&sheet),
        Err(e) => println!("{}", e),
    }
    match parsing::parse_input("B2=4", &mut sheet) {
        Ok(_) => display_sheet(&sheet),
        Err(e) => println!("{}", e),
    }

    
}
