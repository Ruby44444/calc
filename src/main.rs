use std::io;

use std::error::Error;

use crate::parser::{get_data_mon, Pokemon};

mod maths;
mod parser;



fn main() {
    
    println!("Name of the attacking Pokémon ? : ");
    let mut name_attacking_mon: String = String::new();

    io::stdin().read_line(&mut name_attacking_mon)
        .expect("Failed to read line");
    name_attacking_mon = name_attacking_mon.replace("\"", "").replace("\n", "").replace(" ", "").to_lowercase();
    
    let data_attacking_mon: Result<Pokemon, Box<dyn Error>> = get_data_mon(&name_attacking_mon);
    println!("{data_attacking_mon:#?}");
}
