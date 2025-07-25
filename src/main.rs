use Excel_Rust::{display::display_sheet, parsing::{parse_input,is_valid_cell}, skeleton::Sheet};
use std::io;
use std::time::Instant;
use std::io::Write;
use yew::prelude::*;
use yew::Renderer;
pub mod app;
pub mod handle_command;
use app::App;
fn main() {
    // let args:Vec<String> = std::env::args().collect();
    // if args.len() !=2 && args.len()!=3{
    //     println!("To use sheet enter cargo run --rows columns. If you want to know more about the product run cargo run --about");
    //     return;
    // }
    // if args.len() == 2{
    //     if args[1].to_lowercase() == "about"{
    //         println!("Hey! This is a rust app that provides excel functionality with some additional peer to peer benefits using libp2p.Hope you enjoy the product!");
    //         return;
    //     }
    //     else{
    //         println!("To use sheet enter cargo run --rows columns. If you want to know more about the product run cargo run --about");
    //         return;
    //     }
    // }  
    // let rows:u32 = args[1].parse().unwrap_or_else(|_|panic!("Please enter a valid positive integer for rows."));
    // let cols:u32 = args[2].parse().unwrap_or_else(|_|panic!("Please enter a valid positive integer for collumns."));

    // let clock_st = Instant::now();
    // let mut sheet:Sheet = Sheet::create_sheet(rows, cols);
    // display_sheet(&sheet);
    // let el_t = clock_st.elapsed().as_secs_f64();
    // print!("[{el_t}] (ok) > ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();

    // loop{
    //     input.clear();
    //     let mut res: Result<(), String> = Ok(());
    //     io::stdin().read_line(&mut input).expect("Failed to read the input.");
    //     input = input.trim().to_string();
        
    //     let strt = Instant::now();
    //     if input.to_lowercase() == "q"{
    //         break;
    //     }
    //     else if input.to_lowercase() == "w"{
    //         sheet.scroll_up();
    //     }
    //     else if input.to_lowercase() == "a"{
    //         sheet.scroll_down();
    //     }
    //     else if input.to_lowercase() == "s"{
    //         sheet.scroll_left();
    //     }
    //     else if input.to_lowercase() == "d"{
    //         sheet.scroll_right();
    //     }
    //     else if input.to_lowercase() == "disable_output"{
    //         sheet.disable_display();
    //     }
    //     else if input.to_lowercase() == "enable_output"{
    //         sheet.enable_display();
    //     }
        
    //     else if input.to_lowercase().starts_with("scroll_to"){
    //         let cell: Vec<&str> = input.split_whitespace().collect();
    //         match is_valid_cell(cell[1],&sheet){
    //             Ok(_)=>{sheet.scroll_to(cell[1]);},
    //             Err(e)=>{res = Err(e);},
    //         }
    //     }
        
    //     else{
    //         match parse_input(&input,&mut sheet){
    //             Ok(_)=> (),
    //             Err(e)=>{res= Err(e);},
    //         }
    //     }
    //     let timed = strt.elapsed().as_secs_f64();
    //     display_sheet(&sheet);
        
    //     match res{
    //         Ok(_)=> {print!("[{timed}] (ok) > ");},
    //         Err(e)=> {print!("[{timed}] ({e}) > ");}
    //     }
    //     io::stdout().flush().unwrap();
    // }
    console_error_panic_hook::set_once();
    Renderer::<App>::new().render();
}
