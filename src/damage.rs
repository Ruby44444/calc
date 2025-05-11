fn damage (level: f32, power: f32, atk: f32, def: f32, list: [f32; 12]) -> f32
{
    let mut num: f32 = (2.0 * level / 5.0 + 2.0).floor();

    for i in list {
        num *= i
    }

    return num;
}