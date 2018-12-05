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
  return frequency for input file
day-two <cmd> <file-path>
  prototype: return the letters in common between the prototype boxes"
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
    3 => {
      let cmd = &args[1];
      let input = read_input(&args[2]);

      match &cmd[..] {
        "prototype" => println!("Prototype: {}", calc_prototype(&input)),
        _ => help(),
      }
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
    if counts.iter().any(|(_, &val)| val == 2) {
      sums.0 + 1
    } else {
      sums.0
    },
    if counts.iter().any(|(_, &val)| val == 3) {
      sums.1 + 1
    } else {
      sums.1
    },
  )
}

fn calc_prototype(inputs: &str) -> String {
  let lines = inputs.lines().map(|l| l.trim());

  lines
    .clone()
    .find_map(|l| {
      lines.clone().find_map(|l2| {
        let inter = string_intersection(l, l2).clone();

        if l.len() - 1 == inter.len() {
          Some(inter)
        } else {
          None
        }
      })
    }).unwrap()
}

fn string_intersection(l1: &str, l2: &str) -> String {
  l1.chars()
    .zip(l2.chars())
    .filter_map(
      |(first, second)| {
        if first == second {
          Some(first)
        } else {
          None
        }
      },
    ).collect()
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

  #[test]
  fn test_calc_prototype() {
    let proto = calc_prototype(
      "abcde
      fghij
      klmno
      pqrst
      fguij
      axcye
      wvxyz",
    );
    assert_eq!(proto, "fgij");
  }
}
