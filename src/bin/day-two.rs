extern crate structopt;

use std::collections::HashMap;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day-two")]
struct Opt {
  #[structopt(short = "p", long = "prototype")]
  prototype: bool,

  #[structopt(name = "FILE")]
  file: String,
}

fn main() {
  let opt = Opt::from_args();

  let input = read_to_string(opt.file).expect("Error reading file");
  if opt.prototype {
    println!(
      "Prototype: {}",
      calc_prototype(&input).expect("Can't find prototype")
    );
  } else {
    println!("Checksum: {}", calc_checksum(&input));
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

fn calc_prototype(inputs: &str) -> Option<String> {
  let lines = inputs.lines().map(|l| l.trim());

  lines.clone().find_map(|l| {
    lines.clone().find_map(|l2| {
      let inter = string_intersection(l, l2).clone();

      if l.len() - 1 == inter.len() {
        Some(inter)
      } else {
        None
      }
    })
  })
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
    ).unwrap();
    assert_eq!(proto, "fgij");
  }
}
