use Excel_Rust::{display::display_sheet, parsing::{parse_input, is_valid_cell}, skeleton::Sheet};
use std::cell::RefCell;
use std::fmt::Write;
thread_local!{
    static SHEET:RefCell<Option<Sheet>> = RefCell::new(None);
}
pub fn initialise_sheet(rows:u32,cols:u32){
    SHEET.with(|s| *s.borrow_mut() = Some(Sheet::create_sheet(rows,cols)) );
}
pub fn handle_command(input:&str)->String{
    let input = input.trim().to_string();
    let mut out = String::new();
    if input.starts_with("init"){
        let inpt:Vec<String> = input.split_whitespace().map(str::to_string).collect();
        if inpt.len() !=3{
            return String::from("Initialise using the command init <rows> <collums>");
        }
        return match (inpt[1].parse::<u32>(),inpt[2].parse::<u32>()){
            (Ok(rows),Ok(cols)) => {
            // let clock_st = Instant::now();
            initialise_sheet(rows, cols);
            SHEET.with(|s| {let s_ref = s.borrow();
                        let sh = s_ref.as_ref().expect("Sheet not initialised");
                        write!(out,"{}",display_sheet(sh)).unwrap();
                        // let el_t = clock_st.elapsed().as_secs_f64();
                        // write!(out,"[{el_t}] (ok) > ").unwrap();
            });
            out},            
            _=>"Rows and Collumns should be positive integers".to_string(),
        };
    }
    SHEET.with(|s|{
        let mut res: Result<(), String> = Ok(());
        // let strt = Instant::now();
        if input.to_lowercase() == "w"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.scroll_up();}
        }
        else if input.to_lowercase() == "a"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.scroll_down();}
        }
        else if input.to_lowercase() == "s"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.scroll_left();}
        }
        else if input.to_lowercase() == "d"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.scroll_right();}
        }
        else if input.to_lowercase() == "disable_output"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.disable_display();}
        }
        else if input.to_lowercase() == "enable_output"{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            sheet.enable_display();}
        }
        
        else if input.to_lowercase().starts_with("scroll_to") {
            let cell: Vec<&str> = input.split_whitespace().collect();

            let is_valid = SHEET.with(|s| {
                let sh_ref = s.borrow();
                let sheet = sh_ref.as_ref().expect("Sheet not initialised");
                is_valid_cell(cell[1], sheet)
            });

            match is_valid {
                Ok(_) => {
                    SHEET.with(|s| {
                        let mut sh_ref = s.borrow_mut();
                        let sheet = sh_ref.as_mut().expect("Sheet not initialised");
                        sheet.scroll_to(cell[1]);
                    });
                },
                Err(e) => {
                    res = Err(e);
                }
            }
        }
        
        else{
            {let mut sh_ref = s.borrow_mut();
            let sheet = sh_ref.as_mut().expect("Sheet not initialised");
            match parse_input(&input,sheet){
                Ok(_)=> (),
                Err(e)=>{res= Err(e);},
            }}
        }
        let sh_ref = s.borrow();
        let sheet = sh_ref.as_ref().expect("Sheet not initialised");
        // let timed = strt.elapsed().as_secs_f64();
        write!(out,"{}",display_sheet(&*sheet)).unwrap();
        
        match res{
            Ok(_)=> {write!(out," (ok) > ").unwrap();},
            Err(e)=> {write!(out," ({e}) > ").unwrap();}
        }
    });
    out
}