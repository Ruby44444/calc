use std::fs::File;
use std::io::prelude::*;
use std::vec;
use serde_json::from_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct GenderRatio {
    #[serde(rename = "M")]
    m: Option<f32>,
    #[serde(rename = "f")]
    f: Option<f32>,
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
    gender_ratio: GenderRatio,
    base_stats: BaseStats,
    abilities: Abilites,
    heightm: u32,
    weightkg: u32,
    color: String,
    prevo: String,
    
}

fn get_stat(){
    let mut file = File::open("../pokedex.json").expect("Erreur json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Erreur json 2");
    let res = from_str::<Pokemon>(&contents);
}