use Excel_Rust::{display::display_sheet, parsing, skeleton::Sheet};
const max_length:u32 = 100;
use std::env::args;
use cputime::ProcessTime;
use std::io;
fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() !=2 && args.len()!=3{
        eprintln!("To use sheet enter cargo run --rows collmns. If you want to know more about the product run cargo run --about");
    }
    if args.len() == 2{
        if args[1].to_lowercase() == "about"{
            println!("Hey! This is a rust app that provides excel functionality with some additional peer to peer benefits using libp2p.Hope you enjoy the product!");
            return;
        }
        else{
            eprintln!("To use sheet enter cargo run --rows collmns. If you want to know more about the product run cargo run --about");
        }
    }  
    let rows:u32 = args[1].parse().unwrap_or_else(|_|panic!("Please enter a valid positive integer for rows."));
    let cols:u32 = args[2].parse().unwrap_or_else(|_|panic!("Please enter a valid positive integer for collumns."));

    let clock_st = ProcessTime::now();
    let mut sheet:Sheet = Sheet::create_sheet(rows, cols);
    display_sheet(sheet);
    let el_t = clock_st.elapsed().as_secs_f64();
    println!("[{}] (ok) > ",el_t);
    let mut input = String::new();

    loop{
        let mut suc = true;
        io::stdin().read_line(&mut input).expect("Failed to read the input.");
        input = input.trim();
        let strt = ProcessTime::now();
        if input.to_lowercase() == "q"{
            break;
        }
        else if input.to_lowercase() == "w"{
            sheet.scroll_up();
        }
        else if input.to_lowercase() == "a"{
            sheet.scroll_down();
        }
        else if input.to_lowercase() == "s"{
            sheet.scroll_left();
        }
        else if input.to_lowercase() == "d"{
            sheet.scroll_right();
        }
        else if input.to_lowercase() == "disable_output"{
            sheet.disable_display();
        }
        else if input.to_lowercase() == "enable_output"{
            sheet.enable_display();
        }
        else if input.len()>=10{
            if input.to_lowercase()[..10] == "scroll_to"{
                let cell = String::from(&input[10..]);
                if sheet.isvalidcell(cell){
                    sheet.scroll_to(cell);
                }
                else{
                    suc = false;
                }
            }
        }
        else{
            parse_input(input,sheet).unwrap();
        }
        let timed = strt.elapsed().as_secs_f64();
        display_sheet(sheet);
        if suc{
            println!("[{}] (ok) > ",timed);
        }
        else{
            println!("[{}] (error parsing input) > ",timed);
        }
    }
}
