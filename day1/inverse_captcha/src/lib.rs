use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1() {
    assert_eq!(captcha("1122", 1), 3);
    assert_eq!(captcha("1111", 1), 4);
    assert_eq!(captcha("1234", 1), 0);
    assert_eq!(captcha("91212129", 1), 9);
  }

  #[test]
  fn part2() {
    assert_eq!(captcha("1212", 2), 6);
    assert_eq!(captcha("1221", 2), 0);
    assert_eq!(captcha("123425", 3), 4);
    assert_eq!(captcha("123123", 3), 12);
    assert_eq!(captcha("12131415", 4), 4);
  }
}

pub struct Config {
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

fn captcha (s: &str, steps: usize) -> u32 {
  let mut counter: u32 = 0;
  let length = s.len();

  let chars: Vec<(usize, char)> = s.char_indices().collect();
  for (i, c) in chars.clone() {
    let to_match = chars[(i + steps) % length].1;
    if c == to_match {
      counter += c.to_digit(10).unwrap();
    }
  }

  counter
}

pub fn run (config: Config) -> Result<(u32, u32), Box<Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let part1 = captcha(&contents, 1);
    let part2 = captcha(&contents, &contents.len() / 2);

    Ok((part1, part2))
}