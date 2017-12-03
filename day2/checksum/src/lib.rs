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

  #[test]
  fn part2() {
    let test = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    assert_eq!(checksum2(test), 9);
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

fn checksum2 (s: &str) -> u32 {
  let mut counter: u32 = 0;
  for line in s.lines() {
    let mut nums: Vec<u32> = line.split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();
    
    nums.sort();
    nums.reverse();

    let len = nums.len();
    
    for i in 0..len {
      for j in (i+1)..len {
        let num = nums[i];
        let den = nums[j];
        if den != 0 && num % den == 0 {
          counter += num / den;
        }
      }
    }
  }

  counter
}

pub fn run (filename: &str) -> Result<(u32, u32), Box<Error>> {
    let mut f = File::open(filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let count = checksum(&contents);
    let count2 = checksum2(&contents);

    Ok((count, count2))
}