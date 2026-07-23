mod plot;

use plot::plot_graph;
use std::env;
use wavers::read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let audio_file_path = &args[1];
    let (samples, sample_rate) = read::<f32, _>(audio_file_path).unwrap();

    plot_graph(&samples, sample_rate);
}
