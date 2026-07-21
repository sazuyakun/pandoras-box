use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = &args[0];
    let second_arg = &args[1];
    println!("The first arg is {first_arg} and the second arg is {second_arg}");
}
