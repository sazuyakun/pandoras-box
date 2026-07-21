use std::env;
use std::fs;
use wavers::{Wav, read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (samples, sample_rate) = read::<i16, _>("files/audio_1.wav").unwrap();

    dbg!(&samples[100..150]);
    dbg!(sample_rate);

    // let first_arg = &args[0];
    // let second_arg = &args[1];
    // println!("The first arg is {first_arg} and the second arg is {second_arg}");
}
