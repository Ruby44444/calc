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
#[error("Unknown Pokémon {0}")]
pub struct ParseMonError(pub String);

#[derive(Debug, thiserror::Error)]
#[error("Unknown type {0}")]
pub struct ParseTypeError(pub String);

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

#[derive(Debug, Serialize, Deserialize)]
pub enum MonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
    Stellar,
}

impl FromStr for MonType {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "fire" => Ok(Self::Fire),
            "water" => Ok(Self::Water),
            "electric" => Ok(Self::Electric),
            "grass" => Ok(Self::Grass),
            "ice" => Ok(Self::Ice),
            "fighting" => Ok(Self::Fighting),
            "poison" => Ok(Self::Poison),
            "ground" => Ok(Self::Ground),
            "flying" => Ok(Self::Flying),
            "psychic" => Ok(Self::Psychic),
            "bug" => Ok(Self::Bug),
            "rock" => Ok(Self::Rock),
            "ghost" => Ok(Self::Ghost),
            "dragon" => Ok(Self::Dragon),
            "dark" => Ok(Self::Dark),
            "steel" => Ok(Self::Steel),
            "fairy" => Ok(Self::Fairy),
            "stellar" => Ok(Self::Stellar),
            _ => Err(ParseTypeError(s.to_string())),
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
    type1: MonType,
    type2: Option<MonType>,
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
    egg_groups: serde_json::Value,
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
        let mut data = serde_json::from_str::<Value>(SPECIES)
            .ok()
            .and_then(|v| match v {
                Value::Object(d) => Some(d),
                _ => None,
            })
            .and_then(|mut v| v.remove("9"))
            .and_then(|v| match v {
                Value::Object(d) => Some(d),
                _ => None,
            })
            .expect("data should an object");

        let mon_data = data.remove(s).ok_or(ParseMonError(s.to_string()))?;

        Ok(serde_json::from_value(mon_data).expect("data should be a valid mon description"))
    }
}
