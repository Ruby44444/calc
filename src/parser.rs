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
    weightkg: f32,
    evos: Option<serde_json::Value>,
    #[serde(rename = "eggGroups")]
    egggroups: serde_json::Value,
    #[serde(rename = "requiredAbility")]
    required_ability: Option<serde_json::Value>,
    #[serde(rename = "battleOnly")]
    battle_only: Option<serde_json::Value>,
    tags: Option<serde_json::Value>,
    #[serde(rename = "otherFormes")]
    other_formes: Option<serde_json::Value>,

}

pub fn get_data_mon(name: String) -> Result<Pokemon, Box< dyn Error>> {

    let file: File = File::open("data/species.json").expect("Unable to open the file");
    let reader: BufReader<File> = BufReader::new(file);

    let data: serde_json::Value = serde_json::from_reader(reader)?;
    let filtered_data: &Value = &data["9"][name];

    let pokemon_filtered = Pokemon {
        num: filtered_data["num"].to_string().parse::<i32>()?,
        name: filtered_data["name"].to_string(),
        types: filtered_data["types"].clone(),
        base_stats: BaseStats { 
            hp: filtered_data["baseStats"]["hp"].to_string().parse::<u32>()?, 
            atk: filtered_data["baseStats"]["atk"].to_string().parse::<u32>()?, 
            def: filtered_data["baseStats"]["def"].to_string().parse::<u32>()?, 
            spa: filtered_data["baseStats"]["spa"].to_string().parse::<u32>()?, 
            spd: filtered_data["baseStats"]["spd"].to_string().parse::<u32>()?, 
            spe: filtered_data["baseStats"]["spe"].to_string().parse::<u32>()? 
        },
        abilities: Some(Abilites {
            first: filtered_data["abilities"]["0"].to_string(),
            second: Some(filtered_data["abilities"].get("1").expect("No Second Ability").to_string()),
            hidden: Some(filtered_data["abilities"].get("H").expect("No Hidden Ability").to_string()),
            secret: Some(filtered_data["abilities"].get("S").expect("No Secret Ability").to_string()),
        }),
        weightkg: filtered_data["weightkg"].to_string().parse::<f32>()?,
        evos: filtered_data.get("evos").cloned(),
        egggroups: filtered_data.get("eggGroups").expect("No Egg Group").clone(),
        required_ability: filtered_data.get("requiredAbility").cloned(),
        battle_only: filtered_data.get("battleOnly").cloned(),
        tags: filtered_data.get("tags").cloned(),
        other_formes: filtered_data.get("battleOnly").cloned()
    };
    Ok(pokemon_filtered)
}

