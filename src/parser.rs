use std::fs::File;
use std::io::prelude::*;
use std::vec;
use serde_json::from_str;
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
struct Pokemon {
    num: u32,
    name: String,
    types: Vec<String>,
    gender: Option<char>,
    gender_ratio: GenderRatio,
    base_stats: BaseStats,
    abilities: Abilites,
    heightm: u32,
    weightkg: u32,
    color: String,
    prevo: Option<String>,
    #[serde(rename = "evoLevel")]
    evo_level: u32,
    evos: Option<Vec<String>>,
    egg_group: Vec<String>,
    tags: Option<Vec<String>>,
    #[serde(rename = "otherFormes")]
    other_formes: Option<Vec<String>>,
    #[serde(rename = "formeOrder")]
    forme_order: Option<Vec<String>>,
    tier: String,
}

fn get_stat(){
    let mut json = std::fs::read_to_string("../pokedex.json").unwrap();
    let pokemons = from_str::<Pokemon>(&json);

}