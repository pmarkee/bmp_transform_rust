use clap::{App, Arg, ArgMatches};
use std::str::FromStr;

pub mod bmp;

fn main() {
    let args = parse_args();
    let input_path = args.value_of("input").unwrap();
    let output_path = args.value_of("output").unwrap();
    let color =
        bmp::Color::from_str(&args.value_of("color").unwrap().to_ascii_lowercase()).unwrap();

    let bmp_file = bmp::read_bmp_file(&input_path).unwrap();
    let transformed = bmp::transform(&bmp_file, color);
    bmp::write_bmp_file(output_path, &transformed).unwrap();
}

fn parse_args() -> ArgMatches {
    App::new("My Test Program")
        .version("0.1.0")
        .author("Péter Márki")
        .about("Transforms bmp file to greyscale (or some other shade)")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .default_value("resources/in.bmp")
                .help("Path to the input file"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .default_value("resources/out.bmp")
                .help("Path to the output file"),
        )
        .arg(
            Arg::with_name("color")
                .short('c')
                .long("color")
                .takes_value(true)
                .default_value("grey")
                .possible_values(["grey", "red", "green", "blue", "yellow", "violet", "cyan"])
                .help("Color to use"),
        )
        .get_matches()
}
