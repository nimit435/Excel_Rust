use Excel_Rust::{display::display_sheet, parsing, skeleton::Sheet};

fn main() {
    let mut sheet = Sheet::create_sheet(10, 10);
    display_sheet(&sheet);
    sheet.matrix[0].is_valid = false;
    display_sheet(&sheet);

    let input = "A1=SUM(C1:B2)";

    match parsing::parse_input(&input, &sheet) {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

}
