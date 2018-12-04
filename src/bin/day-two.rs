use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

fn help() {
  println!(
    "usage:
day-two <file-path>
  return frequency for input file"
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => help(),
    2 => {
      let input = read_input(&args[1]);
      println!("Checksum: {}", calc_checksum(&input))
    }
    _ => help(),
  }
}

fn read_input(path: &str) -> String {
  let path = Path::new(&path);
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(_) => s,
  }
}

struct Sums(u32, u32);

fn calc_checksum(input: &str) -> u32 {
  let sums = input
    .lines()
    .fold(Sums(0, 0), |sums, line| calc_line(&sums, line));

  sums.0 * sums.1
}

fn calc_line(sums: &Sums, line: &str) -> Sums {
  let mut counts = HashMap::new();

  for char in line.trim().chars() {
    *counts.entry(char).or_insert(0) += 1;
  }

  Sums(
    if counts.iter().any(|(_, val)| *val == 2) {
      sums.0 + 1
    } else {
      sums.0
    },
    if counts.iter().any(|(_, val)| *val == 3) {
      sums.1 + 1
    } else {
      sums.1
    },
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_checksum() {
    let checksum = calc_checksum(
      "abcdef
      bababc
      abbcde
      abcccd
      aabcdd
      abcdee
      ababab",
    );
    assert_eq!(checksum, 12);
  }
}
