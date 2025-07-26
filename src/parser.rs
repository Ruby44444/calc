use std::error::Error;
use std::{fs::File, io::BufReader};
use serde_json::{Value};
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
pub struct Generation {
    #[serde(rename = "1")]
    a: Vec<Pokemon>,
    #[serde(rename = "2")]
    b: Vec<Pokemon>,
    #[serde(rename = "3")]
    c: Vec<Pokemon>,
    #[serde(rename = "4")]
    d: Vec<Pokemon>,
    #[serde(rename = "5")]
    e: Vec<Pokemon>,
    #[serde(rename = "6")]
    f: Vec<Pokemon>,
    #[serde(rename = "7")]
    g: Vec<Pokemon>,
    #[serde(rename = "8")]
    h: Vec<Pokemon>,
    #[serde(rename = "9")]
    i: Vec<Pokemon>,
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
    #[serde(rename = "requiredAbility")]
    required_ability: serde_json::Value,
    #[serde(rename = "battleOnly")]
    battle_only: Option<serde_json::Value>,
    tags: Option<serde_json::Value>,
    #[serde(rename = "otherFormes")]
    other_formes: serde_json::Value,

}

pub fn get_data_mon(name: String) -> Result<Value, Box<dyn Error>> {

    let file = File::open("data/species.json").expect("Unable to open the file");
    let reader = BufReader::new(file);

    let data: serde_json::Value = serde_json::from_reader(reader)?;
    let pokemon = &data["9"][name];
    Ok(pokemon.clone())
}