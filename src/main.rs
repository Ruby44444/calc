use std::io;
use titlecase::Titlecase;
mod maths;
mod parser;



fn main() {
    let mut name_atk = String::new();
    let mut name_def = String::new();

    io::stdin()
        .read_line(&mut name_atk)
        .expect("Erreur");
    
    io::stdin()
        .read_line(&mut name_def)
        .expect("Erreur");

    name_atk.titlecase().replace(" ", "-");
    name_def.titlecase().replace(" ", "-");


}
