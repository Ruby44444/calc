use std::error::Error;
use std::{fs::File, io::BufReader};
use serde_json::{Value};
use serde::{Deserialize, Serialize};

enum TypesStats {
    Number(i32),
    Name(String),
    Types(Types),
    Stats(BaseStats),
    Abilities(Abilities),
    Weight(f32),
    //Need to add Evos, eggGroups, ...

}

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
struct Abilities {
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
pub struct Types {
    type1: String,
    type2: Option<String>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Pokemon {
    num: i32,
    name: String,
    types: Types,
    #[serde(rename = "baseStats")]
    base_stats: BaseStats,
    abilities: Abilities,
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

pub fn get_data_mon(name: &String) -> Result<Pokemon, Box< dyn Error>> {

    let file: File = File::open("data/species.json").expect("Unable to open the file");
    let reader: BufReader<File> = BufReader::new(file);

    let data: serde_json::Value = serde_json::from_reader(reader)?;
    let filtered_data: &Value = &data["9"][name];

    let pokemon_filtered = Pokemon {
        num: filtered_data["num"].to_string().parse::<i32>()?,
        name: filtered_data["name"].to_string().replace("\"", ""),
        types: Types { 
            type1: filtered_data["types"][0].to_string().replace("\"", ""), 
            type2: {
                let second_type = filtered_data["types"].get(1);
                second_type.map(|val| val.to_string().replace("\"", ""))
            }
        },
        base_stats: BaseStats { 
            hp: filtered_data["baseStats"]["hp"].to_string().parse::<u32>()?, 
            atk: filtered_data["baseStats"]["atk"].to_string().parse::<u32>()?, 
            def: filtered_data["baseStats"]["def"].to_string().parse::<u32>()?, 
            spa: filtered_data["baseStats"]["spa"].to_string().parse::<u32>()?, 
            spd: filtered_data["baseStats"]["spd"].to_string().parse::<u32>()?, 
            spe: filtered_data["baseStats"]["spe"].to_string().parse::<u32>()? 
        },
        abilities: Abilities {
            first: filtered_data["abilities"]["0"].to_string().replace("\"", ""),
            second: {
                let second_ability = filtered_data["abilities"].get("1");
                second_ability.map(|val| val.to_string().replace("\"", ""))
            },
            hidden: {
                let hidden_ability = filtered_data["abilities"].get("H");
                hidden_ability.map(|val| val.to_string().replace("\"", ""))
            },
            secret: {
                let secret_ability = filtered_data["abilities"].get("1");
                secret_ability.map(|val| val.to_string().replace("\"", ""))
            },
        },
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

pub fn get_base_stat(filtered: Result<Pokemon, Box<dyn Error>>, mut stat: String) -> u32 {
    //!! Needs to check if the stat returned > 0
    stat = stat.replace(" ", "").to_lowercase();

    //Gets the base stat from the Pokémon according to it's arguments
    let data: Result<u32, Box<dyn Error>> = match stat.as_str() { 
        "hp" => filtered.map(|x: Pokemon| x.base_stats.hp),
        "atk" => filtered.map(|x: Pokemon| x.base_stats.atk),
        "def" => filtered.map(|x: Pokemon| x.base_stats.def),
        "spa" => filtered.map(|x: Pokemon| x.base_stats.spa),
        "spd" => filtered.map(|x: Pokemon| x.base_stats.spd),
        "spe" => filtered.map(|x: Pokemon| x.base_stats.spe),
        _ => filtered.map(|_x: Pokemon| 0)
    };

    let displayed_data:u32  = match &data {
        Ok(i) => *i as u32,
        Err(_) => 0
    };
    
    return displayed_data

}