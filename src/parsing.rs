use crate::skeleton::Sheet;
use crate::hash;

pub fn parse_input(input: &str, sheet: &Sheet)->Result<(), &'static str>{
    let (lhs, rhs) = input
                                .split_once('=')
                                .ok_or("Missing '=' in the input")?;

    is_valid_cell(lhs, sheet)?;
    
    Ok(())
}
pub fn is_valid_cell(input: &str, sheet: &Sheet)->Result<(), &'static str>{
    let (letters, numbers) = hash::separate_cell(input)
                                                .map_err(|err| err)?;
    let rows = numbers.parse::<u32>().unwrap();
    let cols = hash::get_column(&letters);
    if rows > sheet.rows {
        return Err(Box::leak(format!("{}: Row number cannot be greater than the number of rows.", input).into_boxed_str()));
    }
    if cols > sheet.cols {
        return Err(Box::leak(format!("{}: Column number cannot be greater than the number of columns.", input).into_boxed_str()));
    }
    Ok(())
}
pub fn validate_rhs(input: &str, sheet: &Sheet)->Result<(&str, &str), &'static str>{
    
}