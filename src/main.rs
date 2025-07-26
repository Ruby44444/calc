use std::io;
use titlecase::Titlecase;
mod maths;
mod parser;



fn main() {
    let a = parser::get_stat();
    println!("{a:?}")


}
