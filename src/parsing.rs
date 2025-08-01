use crate::function::add_edge;
use crate::function::check_cycle;
use crate::function::delete_edge;
use crate::function::recalculate_node;
use crate::skeleton::Sheet;
use crate::skeleton::Celltype;
use crate::hash;
use regex::Regex;

pub fn parse_input(input: &str, sheet: &mut Sheet) -> Result<(), String> {
    
    let (lhs, rhs) = input
        .split_once('=')
        .ok_or_else(|| "Missing '=' in the input".to_string())?;
    
    is_valid_cell(lhs, sheet)?;
    validate_rhs(rhs, sheet)?;
    let (typ , opval, cell1, cell2, is_valid) = get_vals(rhs, sheet);
    let mut stack: Vec<u32> = Vec::new();
    let mut flag: bool = false;
    let t: i32 = match typ{
        Celltype::Constant=> 0,
        Celltype::Arithmetic(_) => 1,
        Celltype::Sleep => 3,
        _ => 2
    };
    let id: usize = hash::get_hash(lhs, sheet.cols) as usize;
    check_cycle(id as usize ,  sheet, &cell1, &cell2, &mut flag, t, &mut stack);
    
   
    if !is_valid{
        sheet.matrix[id].is_valid = false;
    }
    if flag{
        return Err(String::from("This input forms a cyclic dependency."));
    }
    delete_edge(sheet, id);
    assign_values(typ, opval, cell1, cell2, id, sheet);
    add_edge(sheet, id);
    recalculate_node(sheet, &mut stack);
    Ok(())
}

fn assign_values(typ: Celltype, opval: Option<i32>, cell1: Option<i32>, cell2: Option<i32>, id: usize, sheet: &mut Sheet){
    
    sheet.matrix[id].kind = typ;
    sheet.matrix[id].op_val = opval;
    sheet.matrix[id].cell1 = cell1;
    sheet.matrix[id].cell2 = cell2;
}

fn get_vals(rhs: &str, sheet: &Sheet)->(Celltype, Option<i32>, Option<i32>, Option<i32>, bool){
    let temp = validate_rhs(rhs, sheet).unwrap();
    let mut t = Celltype::Constant;
    let mut opval:Option<i32> = None;
    let mut cell1:Option<i32> = None;
    let mut cell2:Option<i32> = None;
    let mut is_valid= true;
    if temp == 0{
        let num = rhs.parse::<i32>().unwrap();
        opval = Some(num);
        (t, opval, cell1, cell2, is_valid)
    }
    else if temp == 1{
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
                    '/' => None,
                    _ => None,
                };
                if op_char == '/'{
                    if num2 ==0{
                        is_valid = false;
                    }
                    else{
                        opval = Some(num1/num2);
                    }
                }
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
  
    else if temp==2 {
        let (func, range_name) = split_for_parenthesis(rhs).unwrap();
        t = match func {
            "MIN" => Celltype::Min,
            "MAX" => Celltype::Max,
            "SUM" => Celltype::Sum,
            "AVG" => Celltype::Avg,
            "STDEV" => Celltype::Stdev,
            _ => Celltype::Constant,
        };
        if let Some((st,en))= range_name.split_once(':') {
            cell1 = Some(hash::get_hash(st, sheet.cols) as i32 );
            cell2 = Some(hash::get_hash(en, sheet.cols) as i32 );
        }
                    
        return (t, opval, cell1, cell2, is_valid);
    }
    else{
        let (_func, arg) = split_for_parenthesis(rhs).unwrap();
        t = Celltype::Sleep;
        if(is_valid_cell(arg, sheet)).is_ok(){
            cell1 = Some(hash::get_hash(arg, sheet.cols) as i32);
        }
        else{
            opval = Some(arg.parse::<i32>().unwrap()); 
        }
        return (t, opval, cell1, cell2, is_valid);

    }
}

pub fn is_valid_cell(input: &str, sheet: &Sheet) -> Result<(), String> {
    let (letters, numbers) = hash::separate_cell(input)
        .map_err(|err| err.to_string())?;
    let rows = numbers.parse::<u32>().unwrap();
    let cols = hash::get_column(&letters);
    if rows > sheet.rows {
        return Err(format!("{input}: Row number cannot be greater than the number of rows." ));
    }
    if cols > sheet.cols {
        return Err(format!("{input}: Column number cannot be greater than the number of columns." ));
    }
    Ok(())
}

fn validate_rhs(rhs: &str, sheet: &Sheet) -> Result<u32, String> {
    let mut st_cnt = 0;
    let mut en_cnt = 0;

    for ch in rhs.chars() {
        if ch == '(' {
            if en_cnt > st_cnt {
                return Err(format!("{rhs}: Invalid RHS." ));
            }
            st_cnt += 1;
        } else if ch == ')' {
            en_cnt += 1;
        }
    }
    if st_cnt != en_cnt || st_cnt > 1{
        Err(format!("{rhs}: Invalid RHS."))
    } 
    else if st_cnt == 0 {
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
            return Err(format!("{rhs}: Invalid RHS." ));
        }
    }
    
    else if let Some((func_name, range_str)) = split_for_parenthesis(rhs) {
        
        is_valid_func(func_name)?;
        if func_name == "SLEEP"{
            if(is_valid_cell(range_str, sheet)).is_ok() || is_valid_number(range_str).is_ok(){
                return Ok(3);
            }

            else{
                return Err(format!("{rhs}: Invalid RHS." ));
            }
        }
        is_valid_range(range_str, sheet)?;
        Ok(2)
    } 
    else {
        Err(format!("{rhs}: Invalid RHS." ))
    }
    
}

fn is_valid_func(input: &str)->Result<(), String>{

    let valid_funcs = ["MIN", "MAX", "AVG", "STDEV", "SLEEP", "SUM"];
    if valid_funcs.contains(&input.trim().to_uppercase().as_str()) {
        Ok(())
    } else {
        Err(format!("{input}: Not a valid function name."))
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
        return Err(format!("{input}: Range is invalid"));
    }

    Ok(())
}

fn is_valid_number(input: &str) -> Result<(), String> {

    if input.trim().is_empty() {
        return Err("Input number is empty.".to_string());
    }

    match input.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{input}: Not a valid integer.")),
    }
}

fn split_by_operators(input: &str) -> Option<(&str, &str, &str)> {
    // Match: cell or number, operator, cell or number
    let re = Regex::new(r"^([A-Za-z]+\d+|-?\d+)\s*([+\-*/])\s*([A-Za-z]+\d+|-?\d+)$").unwrap();
    if let Some(caps) = re.captures(input.trim()) {
        let lhs = caps.get(1)?.as_str();
        let op = caps.get(2)?.as_str();
        let rhs = caps.get(3)?.as_str();
        Some((lhs.trim(), op, rhs.trim()))
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