
#[allow(dead_code)]
fn damage (level: f32, power: f32, atk: f32, def: f32, list: [f32; 12]) -> f32
{
    let mut num: f32 = ((2.0 * level / 5.0 + 2.0).floor() * power * atk / def).floor();

    for i in list {
        num *= i
    }

    return num;
}

#[allow(dead_code)]
fn get_stat (name: String, base_stat: f32, ev: f32, iv: f32, level: f32, nature: f32) -> f32
{
    let temp = ((2.0 * base_stat + iv + (ev/4.0).floor()) * level / 100.0).floor();

    if name == "hp" {
        return temp + level + 10.0 as f32;
    } else {
        return (temp + 5.0) * nature;
    }
     
}