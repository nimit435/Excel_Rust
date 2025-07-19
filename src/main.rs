use Excel_Rust::{display::display_sheet, skeleton::Sheet};

fn main() {
    let mut sheet = Sheet::create_sheet(999, 18278);
    display_sheet(&sheet);
    sheet.scroll_to("SEX420");
    display_sheet(&sheet);

}
