use std::env;
use std::fs;

fn array_to_dxf() {
        
}

struct Config {
    airfoil_filename: String,
    output_chord_length: f64,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }

        let no_chord_length_given: bool = args.len() < 3;

        if no_chord_length_given {
            println!("No chord length given. Assuming chord length of 1.");
        }

        let output_chord_length: f64 = if no_chord_length_given { 1.0 } 
            else { args[2].clone().parse().expect("Second argument should be a float!") };

        let airfoil_filename = args[1].clone();

        Ok(Config { airfoil_filename, output_chord_length }) 
    }
}

fn main() {
    println!("Make sure your airfoil uses Selig format!");

    let args: Vec<String> = env::args().collect();
    
    let config = parse_arguments(&args);

    let airfoil_file_contents = fs::read_to_string(config.airfoil_filename)
        .expect("Should have been able to read file!");
}
