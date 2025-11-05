use crate::parser::{BaseStatKind, Pokemon};


fn round_down(n: f32) -> f32 {
    if n.fract() == 0.5 {return n.floor()}
    else {return n.round()}
}

pub fn damage(level: f32, power: f32, atk: f32, def: f32) -> f32 {

    let num: f32 = (((2.0 * level / 5.0 + 2.0).floor() * power * atk / def).floor() / 50.0).floor() + 2.0;
    return num;
}

pub fn damage_modifier(damage: f32, user: Pokemon, attack: String) -> [f32; 16] {
    let rng: [u32; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut dmg: [f32; 16] = [0.0; 16];
    for i in 0..=15 {
        dmg[i] = (damage * (100 - rng[i]) as f32 / 100.0).floor();
    }

    for i in 0..=15 {
        dmg[i] = round_down(dmg[i] as f32 * 6144.0/4096.0);
    }

    for i in 0..=15 {
        //dmg[i] *= 4.0;
        dmg[i] = round_down(dmg[i]);
    }
    dmg
}

#[allow(dead_code)]
pub fn get_stat_number(
    stat: BaseStatKind,
    base_stat: f32,
    ev: f32,
    iv: f32,
    level: f32,
    nature: f32,
) -> f32 {
    let temp = ((2.0 * base_stat + iv + (ev / 4.0).floor()) * level / 100.0).floor();

    if stat == BaseStatKind::Hp {
        return temp + level + 10.0 as f32;
    } else {
        return (temp + 5.0) * nature;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{maths::{damage, damage_modifier, get_stat_number}, parser::{BaseStatKind, Pokemon}};

    #[test]
    pub fn damage_calc() {
        let def: Pokemon = Pokemon::from_str("abomasnow").unwrap();
        let mut stat_def = def.get_base_stat(BaseStatKind::Spd) as f32;

        let atk: Pokemon = Pokemon::from_str("abomasnow").unwrap();
        let mut stat_atk = atk.get_base_stat(BaseStatKind::Spa) as f32;


        stat_atk = get_stat_number(BaseStatKind::Spa, stat_atk, 0.0, 0.0, 75.0, 1.0);
        stat_def = get_stat_number(BaseStatKind::Spd, stat_def, 0.0, 0.0, 63.0, 1.0);


        let calc = damage_modifier(damage(100.0, 110.0, 220.0, 206.0), Pokemon::from_str("landorus").unwrap(), "a".to_string());

        assert_eq!(calc, [127.0, 129.0, 130.0, 132.0, 133.0, 135.0, 136.0, 138.0, 139.0, 141.0, 142.0, 144.0, 145.0, 147.0, 148.0, 150.0]);
        }
}
