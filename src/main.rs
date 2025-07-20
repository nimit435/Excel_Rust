use Excel_Rust::{display::display_sheet, parsing, skeleton::Sheet};

fn main() {
    let sheet = Sheet::create_sheet(10, 10);
    display_sheet(&sheet);


    let input = "A1=-1+-1";

    match parsing::parse_input(&input, &sheet) {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

}
