use crate::skeleton::Sheet;
use crate::skeleton::Celltype;
use crate::hash;
use regex::Regex;

pub fn parse_input(input: &str, sheet: &Sheet) -> Result<(), String> {
    let (lhs, rhs) = input
        .split_once('=')
        .ok_or_else(|| "Missing '=' in the input".to_string())?;

    is_valid_cell(lhs, sheet)?;
    validate_rhs(rhs, sheet)?;
    let (typ , opval, cell1, cell2, is_valid) = get_vals(rhs, sheet);
    
    Ok(())
}
fn get_vals(rhs: &str, sheet: &Sheet)->(Celltype, Option<i32>, Option<i32>, Option<i32>, bool){
    let mut t = Celltype::Constant;
    let mut opval:Option<i32> = None;
    let mut cell1:Option<i32> = None;
    let mut cell2:Option<i32> = None;
    let mut is_valid= true;
    if validate_rhs(rhs, sheet).unwrap() == 0{
        let num = rhs.parse::<i32>().unwrap();
        opval = Some(num);
        return (t, opval, cell1, cell2, is_valid);
    }
    else if validate_rhs(rhs, sheet).unwrap() == 1{
        if is_valid_cell(rhs, sheet).is_ok(){
            opval = Some(0);
            t = Celltype::Arithmetic('+');
            cell1 = Some(hash::get_hash(rhs, sheet.cols) as i32);
        }
        if let Some((lhs_part, op, rhs_part)) = split_by_operators(rhs) {
            if is_valid_number(lhs_part).is_ok() && is_valid_number(rhs_part).is_ok(){
                let num1 = lhs_part.parse::<i32>().unwrap();
                let num2 = rhs_part.parse::<i32>().unwrap();
                let op_char = op.chars().next().unwrap();
                t = Celltype::Constant;
                opval = match op_char {
                    '+' => Some(num1 + num2),
                    '-' => Some(num1 - num2),
                    '*' => Some(num1 * num2),
                    '/' => {
                        if num2 == 0 {
                            is_valid = false;
                            None
                        } else {
                            Some(num1 / num2)
                        }
                    }
                    _ => None,
                };
            }
            else if is_valid_number(lhs_part).is_ok(){
                let num1 = lhs_part.parse::<i32>().unwrap();
                let op_char = op.chars().next().unwrap();
                t = Celltype::Arithmetic(op_char);
                cell1 = Some(hash::get_hash(rhs_part, sheet.cols) as i32);
                opval = Some(num1);
            }
            else if is_valid_number(rhs_part).is_ok(){
                let num2 = rhs_part.parse::<i32>().unwrap();
                let op_char = op.chars().next().unwrap();
                t = Celltype::Arithmetic(op_char);
                cell1 = Some(hash::get_hash(lhs_part, sheet.cols) as i32);
                opval = Some(num2);
            }
            else{
                let op_char = op.chars().next().unwrap();
                t = Celltype::Arithmetic(op_char);
                cell1 = Some(hash::get_hash(lhs_part, sheet.cols) as i32);
                cell2 = Some(hash::get_hash(rhs_part, sheet.cols) as i32);
            }
        }
        return (t, opval, cell1, cell2, is_valid);
    }
  
    else{
        let (func, range_name) = split_for_parenthesis(rhs).unwrap();
        t = match func {
            "MIN" => Celltype::Min,
            "MAX" => Celltype::Max,
            "SUM" => Celltype::Sum,
            "AVG" => Celltype::Avg,
            "STDEV" => Celltype::Stdev,
            "SLEEP" => Celltype::Sleep,
            _ => Celltype::Constant,
        };
        match range_name.split_once(':') {
            Some((st, en)) => {
                cell1 = Some(hash::get_hash(st, sheet.cols) as i32 );
                cell2 = Some(hash::get_hash(en, sheet.cols) as i32 );
            },
            None => (),
        }
                    
        return (t, opval, cell1, cell2, is_valid);
    }

}
fn is_valid_cell(input: &str, sheet: &Sheet) -> Result<(), String> {
    let (letters, numbers) = hash::separate_cell(input)
        .map_err(|err| err.to_string())?;
    let rows = numbers.parse::<u32>().unwrap();
    let cols = hash::get_column(&letters);
    if rows > sheet.rows {
        return Err(format!("{}: Row number cannot be greater than the number of rows.", input));
    }
    if cols > sheet.cols {
        return Err(format!("{}: Column number cannot be greater than the number of columns.", input));
    }
    Ok(())
}

fn validate_rhs(rhs: &str, sheet: &Sheet) -> Result<u32, String> {
    let mut st_cnt = 0;
    let mut en_cnt = 0;

    for ch in rhs.chars() {
        if ch == '(' {
            if en_cnt > st_cnt {
                return Err(format!("{}: Invalid RHS.", rhs));
            }
            st_cnt += 1;
        } else if ch == ')' {
            en_cnt += 1;
        }
    }
    if st_cnt != en_cnt {
        return Err(format!("{}: Invalid RHS.", rhs));
    } else if st_cnt > 1 {
        return Err(format!("{}: Invalid RHS.", rhs));
    } else if st_cnt == 0 {
        if is_valid_number(rhs).is_ok() {
            return Ok(0);
        } 
        else if is_valid_cell(rhs, sheet).is_ok(){
            return Ok(1);
        }
        else if let Some((lhs_part, _op, rhs_part)) = split_by_operators(rhs) {
            if is_valid_number(lhs_part).is_err() {
                is_valid_cell(lhs_part, sheet)?;
            }
            if is_valid_number(rhs_part).is_err() {
                is_valid_cell(rhs_part, sheet)?;
            }
            return Ok(1);
        } else {
            return Err(format!("{}: Invalid RHS.", rhs));
        }
    }
    else {
        if let Some((func_name, range_str)) = split_for_parenthesis(rhs) {
            is_valid_range(range_str, sheet)?;
            is_valid_func(func_name)?;
            return Ok(2);
        } else {
            return Err(format!("{}: Invalid RHS.", rhs));
        }
    }
}

fn is_valid_func(input: &str)->Result<(), String>{

    let valid_funcs = ["MIN", "MAX", "AVG", "STDEV", "SLEEP", "SUM"];
    if valid_funcs.contains(&input.trim().to_uppercase().as_str()) {
        Ok(())
    } else {
        Err(format!("{}: Not a valid function name.", input))
    }
}

fn is_valid_range(input: &str, sheet: &Sheet) -> Result<(), String> {

    let (st, en) = input
        .split_once(':')
        .ok_or_else(|| "Missing ':' in the input".to_string())?;

    is_valid_cell(st, sheet)?;
    is_valid_cell(en, sheet)?;
    let (letter1, number1) = hash::separate_cell(st).unwrap();
    let (letter2, number2) = hash::separate_cell(en).unwrap();

    let row1 = number1.parse::<u32>().unwrap();
    let row2 = number2.parse::<u32>().unwrap();
    let col1 = hash::get_column(&letter1);
    let col2 = hash::get_column(&letter2);

    if row1 > row2 || col1 > col2 {
        return Err(format!("{}: Range is invalid", input));
    }

    Ok(())
}

fn is_valid_number(input: &str) -> Result<(), String> {

    if input.trim().is_empty() {
        return Err("Input number is empty.".to_string());
    }

    match input.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{}: Not a valid integer.", input)),
    }
}

fn split_by_operators(input: &str)->Option<(&str, &str, &str)>{

    let re = Regex::new(r"([+\-*/])").unwrap();
    if let Some(m) = re.find(input) {
        let op = m.as_str();
        let (lhs, rhs) = input.split_at(m.start());
        Some((lhs.trim(), op, rhs[1..].trim()))
    } else {
        None
    }
}
fn split_for_parenthesis(input: &str) -> Option<(&str, &str)> {
    
    let input = input.trim();
    let open_paren = input.find('(')?;
    let close_paren = input.rfind(')')?;
    if open_paren > close_paren {
        return None;
    }
    let func_name = &input[..open_paren].trim();
    let arg = &input[open_paren + 1..close_paren].trim();
    if func_name.is_empty() || arg.is_empty() {
        return None;
    }
    Some((func_name, arg))
}