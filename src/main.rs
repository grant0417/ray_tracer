use ray_tracer::{render_scene};
use std::{error::Error};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    const IMAGE_WIDTH: usize = 1200 / 2;
    const IMAGE_HEIGHT: usize = 800 / 2;
    const SAMPLES_PER_PIXEL: usize = 50;

    let width_default = IMAGE_WIDTH.to_string();
    let height_default = IMAGE_HEIGHT.to_string();
    let samples_default = SAMPLES_PER_PIXEL.to_string();

    let matches = App::new("Ray Tracer")
        .version("0.1")
        .author("Grant Gurvis")
        .about("A small ray tracer based on Ray Tracing in a weekend")
        .arg(Arg::with_name("OUTPUT")
            .help("The file to output to with a file extension")
            .required(true)
            .index(1))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Sets the render width")
            .default_value(&width_default)
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Sets the render height")
            .default_value(&height_default)
            .takes_value(true))
        .arg(Arg::with_name("samples")
            .short("s")
            .long("samples")
            .help("Sets the number of samples per pixel")
            .default_value(&samples_default)
            .takes_value(true))
        .get_matches();

    let file = matches.value_of("OUTPUT").unwrap();
    let width = matches.value_of("width").unwrap().parse().unwrap_or(IMAGE_WIDTH);
    let height = matches.value_of("height").unwrap().parse().unwrap_or(IMAGE_HEIGHT);
    let samples = matches.value_of("samples").unwrap().parse().unwrap_or(SAMPLES_PER_PIXEL);

    eprintln!("Starting render.");
    eprintln!("Dimensions: {}x{}", width, height);
    eprintln!("Samples per Pixel: {}\n", samples);

    image::ImageFormat::from_path(file)?;

    let time = Instant::now();

    eprintln!("Rendering scene...");

    let total = height * width;
    let render_bar = ProgressBar::new(total as u64);
    render_bar.set_style(ProgressStyle::default_bar()
        .template("{wide_bar} {percent}% Elapsed: {elapsed_precise} Remaining: {eta_precise}"));
    render_bar.set_draw_delta((total / 1000) as u64);

    let colors = render_scene("Cornell Box with Metal Cube", width, height, samples)?;

    render_bar.finish();

    eprintln!("\nOutputting to {}.", file);

    let image = image::RgbaImage::from_raw(width as u32, height as u32, colors).unwrap();

    image.save(file)?;

    eprintln!("\nDone in {}.{:03} sec", time.elapsed().as_secs(), time.elapsed().subsec_millis());

    Ok(())
}
