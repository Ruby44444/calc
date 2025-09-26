
fn round_up(n: f32) -> f32 {
    if n.fract() == 0.5 {return n.floor()}
    else {return n.round()}
}

#[allow(dead_code)]
pub fn damage(level: f32, power: f32, atk: f32, def: f32, list: [f32; 12]) -> [u32; 16] {

    let mut num: f32 = round_up(((2.0 * level / 5.0 + 2.0) * power * atk / def + 2.0) / 50.0);
    let rng: [f32; 16] = [0.85, 0.86, 0.87, 0.88,
                            0.89, 0.90, 0.91, 0.92,
                            0.93, 0.94, 0.95, 0.96,
                            0.97, 0.98, 0.99, 1.0];

    for i in list {
        //list = all of the multipliers
        num = round_up(num * i);
    }
    let rolls: [u32; 16] = rng.map(|v| (v * num) as u32);
    return rolls;
}

#[allow(dead_code)]
pub fn get_stat_number(
    stat: String,
    base_stat: f32,
    ev: f32,
    iv: f32,
    level: f32,
    nature: f32,
) -> f32 {
    let temp = ((2.0 * base_stat + iv + (ev / 4.0).floor()) * level / 100.0).floor();

    if stat == "hp" {
        return temp + level + 10.0 as f32;
    } else {
        return (temp + 5.0) * nature;
    }
}
