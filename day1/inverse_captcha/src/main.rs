extern crate inverse_captcha;

use std::env;
use std::process;

use inverse_captcha::Config;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    match inverse_captcha::run(config) {
        Ok(count) => println!("Part1: {}, Part2: {}", count.0, count.1),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}