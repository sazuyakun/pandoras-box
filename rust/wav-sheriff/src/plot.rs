use plotters::prelude::*;
use wavers::Samples;

const FILE_PATH: &str = "files/plot_1_test.png";
const CAPTION_NAME: &str = "The WAV Audio Plot";

pub fn plot_graph(samples: &Samples<f32>, sample_rate: i32) {
    // Plotting the samples
    let duration = samples.len() as f32 / sample_rate as f32;

    let root = BitMapBackend::new(FILE_PATH, (640, 640)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(CAPTION_NAME, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..duration, -1f32..1f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // let iter = samples.iter().enumerate();

    chart
        .draw_series(LineSeries::new(
            samples.iter().enumerate().map(|(index, amplitude)| {
                let time = index as f32 / sample_rate as f32;
                (time, *amplitude)
            }),
            &RED,
        ))
        .unwrap();

    root.present().unwrap();
}
