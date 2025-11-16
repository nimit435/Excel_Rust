pub fn separate_cell(input: &str)->Result<(String, String), String>{
    let mut numbers = String::new();
    let mut letters = String::new();
    let mut seen_digit = false;

    for ch in input.chars(){
        if ch.is_ascii_uppercase(){
            if seen_digit{
                return Err(format!("{input}: Digits cannot come before alphabets"));
            }
            letters.push(ch);
        }
        else if ch.is_ascii_digit(){
            seen_digit = true;
            numbers.push(ch);
        }
        else {
            return Err(format!("{input}: Only uppercase alphabets and numbers allowed"));
        }
    }
    if letters.is_empty() || numbers.is_empty() {
        return Err(format!("{input}: Must contain both uppercase letters and digits"));
    }

    Ok((letters, numbers))
}

pub fn get_column(letters: &str)->u32{
    let mut col = 0;
    for ch in letters.chars() {
        col = col * 26 + (ch as u8 - b'A' + 1) as u32;
    }
    col  
}

pub fn get_hash(input: &str, cols: u32)->u32{
    let (letters, numbers) = separate_cell(input).unwrap();
    let col = get_column(&letters)-1;
    let row = numbers.parse::<u32>().unwrap()-1;
    (row*cols)+col
}

// pub fn hash_to_string(id: u32, cols: u32) -> String {
//     let row = (id / cols) + 1;
//     let mut col = id % cols;
//     col += 1;
//     let col_letters = col_mapping(col);

//     format!("{col_letters}{row}")
// }

pub fn col_mapping(mut col: u32)->String{
    let mut col_letters = String::new();

    while col > 0 {
        col -= 1;
        let ch = ((col % 26) as u8 + b'A') as char;
        col_letters.insert(0, ch);
        col /= 26;
    }
    col_letters
}