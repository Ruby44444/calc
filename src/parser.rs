use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

const SPECIES: &'static str = include_str!("data/species.json");

#[derive(Debug, thiserror::Error)]
#[error("Unknown stat")]
pub struct ParseStatError;

#[derive(Debug, thiserror::Error)]
pub enum ParseMonError {
    #[error("Unknown Pokémon")]
    Unknown,
    #[error("Can't find stat {0}")]
    MissingStat(String),
}
pub enum BaseStatKind {
    Hp,
    Atk,
    Def,
    Spa,
    Spd,
    Spe,
}

impl FromStr for BaseStatKind {
    type Err = ParseStatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hp" => Ok(Self::Hp),
            "atk" => Ok(Self::Atk),
            "def" => Ok(Self::Def),
            "spa" => Ok(Self::Spa),
            "spd" => Ok(Self::Spd),
            "spe" => Ok(Self::Spe),
            _ => Err(ParseStatError),
        }
    }
}

#[allow(dead_code)]
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
    n: Option<char>,
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

impl Index<BaseStatKind> for BaseStats {
    type Output = u32;

    fn index(&self, index: BaseStatKind) -> &Self::Output {
        match index {
            BaseStatKind::Hp => &self.hp,
            BaseStatKind::Atk => &self.atk,
            BaseStatKind::Def => &self.def,
            BaseStatKind::Spa => &self.spa,
            BaseStatKind::Spd => &self.spd,
            BaseStatKind::Spe => &self.spe,
        }
    }
}

impl IndexMut<BaseStatKind> for BaseStats {
    fn index_mut(&mut self, index: BaseStatKind) -> &mut Self::Output {
        match index {
            BaseStatKind::Hp => &mut self.hp,
            BaseStatKind::Atk => &mut self.atk,
            BaseStatKind::Def => &mut self.def,
            BaseStatKind::Spa => &mut self.spa,
            BaseStatKind::Spd => &mut self.spd,
            BaseStatKind::Spe => &mut self.spe,
        }
    }
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
    type2: Option<String>,
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

impl Pokemon {
    ///Gets the base stat from the Pokémon according to it's arguments
    pub fn get_base_stat(&self, stat: BaseStatKind) -> u32 {
        self.base_stats[stat]
    }
}

impl FromStr for Pokemon {
    type Err = ParseMonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: serde_json::Value =
            serde_json::from_str(SPECIES).map_err(|_| ParseMonError::Unknown)?;
        let filtered_data: &Value = &data["9"][s];

        let pokemon_filtered = Pokemon {
            num: filtered_data["num"]
                .to_string()
                .parse()
                .map_err(|_| ParseMonError::MissingStat("num".to_string()))?,
            name: filtered_data["name"].to_string().replace("\"", ""),
            types: Types {
                type1: filtered_data["types"][0].to_string().replace("\"", ""),
                type2: {
                    let second_type = filtered_data["types"].get(1);
                    second_type.map(|val| val.to_string().replace("\"", ""))
                },
            },
            base_stats: BaseStats {
                hp: filtered_data["baseStats"]["hp"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("hp".to_string()))?,
                atk: filtered_data["baseStats"]["atk"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("atk".to_string()))?,
                def: filtered_data["baseStats"]["def"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("def".to_string()))?,
                spa: filtered_data["baseStats"]["spa"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("spa".to_string()))?,
                spd: filtered_data["baseStats"]["spd"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("spd".to_string()))?,
                spe: filtered_data["baseStats"]["spe"]
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| ParseMonError::MissingStat("spe".to_string()))?,
            },
            abilities: Abilities {
                first: filtered_data["abilities"]["0"]
                    .to_string()
                    .replace("\"", ""),
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
            weightkg: filtered_data["weightkg"]
                .to_string()
                .parse::<f32>()
                .map_err(|_| ParseMonError::MissingStat("weightkg".to_string()))?,
            evos: filtered_data.get("evos").cloned(),
            egggroups: filtered_data
                .get("eggGroups")
                .expect("No Egg Group")
                .clone(),
            required_ability: filtered_data.get("requiredAbility").cloned(),
            battle_only: filtered_data.get("battleOnly").cloned(),
            tags: filtered_data.get("tags").cloned(),
            other_formes: filtered_data.get("battleOnly").cloned(),
        };
        Ok(pokemon_filtered)
    }
}
