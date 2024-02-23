use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let query: &String = &args[1];
    let filepath: &String = &args[2];

    println!("query is ===> {}", query);
    println!("filepath is ===> {}", filepath);
}
