use std::error::Error;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;
use std::vec;
use serde_json::{from_str};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct GenderRatio {
    #[serde(rename = "M")]
    m: Option<f32>,
    #[serde(rename = "F")]
    f: Option<f32>,
    #[serde(rename = "N")]
    n: Option<char>
}

#[derive(Deserialize, Serialize, Debug)]
struct BaseStats {
    hp: u32,
    atk: u32,
    def: u32,
    spa: u32,
    spd: u32,
    spe: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Abilites {
    #[serde(rename = "0")]
    first: String,
    #[serde(rename = "1")]
    second: Option<String>,
    #[serde(rename = "H")]
    hidden: Option<String>,
    #[serde(rename = "S")]
    secret: Option<String>,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pokemon {
    num: i32,
    name: String,
    types: serde_json::Value,
    #[serde(rename = "baseStats")]
    base_stats: BaseStats,
    abilities: Option<Abilites>,
    weightkg: i32,
    #[serde(rename = "eggGroups")]
    egggroups: serde_json::Value,
    tags: serde_json::Value,
    #[serde(rename = "otherFormes")]
    other_formes: serde_json::Value,

}

pub fn get_stat() -> Result<Pokemon, Box<dyn Error>> {

    let file = File::open("pokedex.json").expect("Unable to open the file");
    let reader = BufReader::new(file);

    let data: Pokemon = serde_json::from_reader(reader)?;

    Ok(data)
}