use std::env;
use std::fs;

fn array_to_dxf() {
        
}

struct Config {
    airfoil_filename: String,
    output_chord_length: f64,
}

fn parse_arguments(args: &[String]) -> Result<Config, &'static str> {
    let airfoil_filename = args[1].clone();
    let output_chord_length: f64 = args[2].clone().parse().unwrap();

    Ok( Config{ airfoil_filename, output_chord_length } )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_arguments(&args);
}
