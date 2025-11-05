mod maths;
mod parser;

use std::{io, os::linux::raw::stat, str::FromStr};

use crate::{maths::{damage, get_stat_number}, parser::{BaseStatKind, Pokemon}};


fn main() -> anyhow::Result<()> {
    println!("Name of the attacking Pokémon ? : ");
    let mut name_attacking_mon: String = String::new();

    io::stdin().read_line(&mut name_attacking_mon)?;
    name_attacking_mon = name_attacking_mon
        .replace(char::is_whitespace, "")
        .to_lowercase();

    let attacking_mon = Pokemon::from_str(&name_attacking_mon)?;
    println!("{attacking_mon:#?}");
    let stat_atk = attacking_mon.get_base_stat("atk".parse()?) as f32;
    println!("{stat_atk}");

    println!("Name of the defending Pokémon ? : ");
    let mut name_defending_mon: String = String::new();

    io::stdin().read_line(&mut name_defending_mon)?;
    name_defending_mon = name_defending_mon
        .replace(char::is_whitespace, "")
        .to_lowercase();

    let defending_mon: Pokemon = Pokemon::from_str(&name_defending_mon)?;
    println!("{defending_mon:#?}");
    let stat_defense = defending_mon.get_base_stat("def".parse()?) as f32;
    println!("{stat_defense}");
    println!("{}", get_stat_number(BaseStatKind::Atk, stat_atk, 252.0, 31.0, 100.0, 1.0));

    let damage = damage(100.0, 110.0, 
                                    get_stat_number(BaseStatKind::Atk, stat_atk, 252.0, 31.0, 100.0, 1.0),
                                    get_stat_number(BaseStatKind::Def, stat_defense, 0.0, 31.0, 100.0, 1.0));
    println!("{damage:?}");


    Ok(())
}
