mod maths;
mod parser;



fn main() {
    let a = parser::get_data_mon("dianciemega".to_owned());
    println!("{a:#?}")
}
