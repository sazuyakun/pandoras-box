use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string("files/test_1.txt").expect("Couldn't read files");
    println!("{contents}");

    // let first_arg = &args[0];
    // let second_arg = &args[1];
    // println!("The first arg is {first_arg} and the second arg is {second_arg}");
}
