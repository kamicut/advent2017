use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1() {
    let test = "5 1 9 5\n7 5 3\n2 4 6 8";
    assert_eq!(checksum(test), 18);
  }
}

fn checksum (s: &str) -> u32 {
  let mut counter = 0;
  for line in s.lines() {
    let nums: Vec<u32> = line.split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();
    
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();
    counter += max - min;
  }

  counter
}

pub fn run (filename: &str) -> Result<u32, Box<Error>> {
    let mut f = File::open(filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let count = checksum(&contents);

    Ok(count)
}