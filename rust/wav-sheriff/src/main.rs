use std::env;
use wavers::read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let audio_file_path = &args[1];
    let (samples, sample_rate) = read::<f32, _>(audio_file_path).unwrap();

    dbg!(&samples[100..110]);
    dbg!(sample_rate);
}
