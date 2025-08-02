mod maths;
mod parser;

use std::{io, str::FromStr};

use crate::parser::Pokemon;

fn main() -> anyhow::Result<()> {
    println!("Name of the attacking Pokémon ? : ");
    let mut name_attacking_mon: String = String::new();

    io::stdin().read_line(&mut name_attacking_mon)?;
    name_attacking_mon = name_attacking_mon
        .replace(char::is_whitespace, "")
        .to_lowercase();

    let attacking_mon = Pokemon::from_str(&name_attacking_mon)?;
    println!("{attacking_mon:#?}");
    let stat = attacking_mon.get_base_stat("hp".parse()?);
    println!("{stat}");

    Ok(())
}
