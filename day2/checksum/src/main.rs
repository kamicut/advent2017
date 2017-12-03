extern crate checksum;

use std::env;
use std::process;

/// Get filename from cli args
fn parse_args(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let filename = args[1].clone();
    Ok(filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    match checksum::run(&filename) {
        Ok(count) => println!("Count: {}", count),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}