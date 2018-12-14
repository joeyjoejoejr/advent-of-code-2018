extern crate clap;
use clap::{App, Arg};

use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
  let matches = App::new("Day One")
    .version("0.1")
    .arg(
      Arg::with_name("calibrate")
        .help("flag to calibrate")
        .short("c")
        .long("calibrate"),
    ).arg(Arg::with_name("INPUT").help("Input file").required(true))
    .get_matches();

  let input = read_to_string(matches.value_of("INPUT").unwrap()).expect("Error reading file");
  let parsed_input = parse_input(&input).expect("Error parsing file");

  if matches.is_present("calibrate") {
    println!(
      "{}",
      calc_calibration(&parsed_input).expect("Can't calculate calibration")
    );
  } else {
    println!("{}", calc_frequency(&parsed_input));
  }
}

fn parse_input(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
  input
    .lines()
    .map(|line| line.trim().parse::<i32>())
    .collect()
}

fn calc_frequency(input: &[i32]) -> i32 {
  input.iter().sum()
}

fn calc_calibration(input: &[i32]) -> Option<i32> {
  let mut items = input.iter().cycle();
  let mut values = HashSet::new();
  let mut frequency = 0;
  values.insert(frequency);

  loop {
    let number: &i32 = items.next()?;
    frequency += number;
    if values.contains(&frequency) {
      break Some(frequency);
    }
    values.insert(frequency);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_a_number() {
    let frequency = calc_frequency(&[1]);
    assert_eq!(frequency, 1)
  }

  #[test]
  fn test_subtract_a_number() {
    let frequency = calc_frequency(&[-1]);
    assert_eq!(frequency, -1)
  }

  #[test]
  fn test_multiple_numbers() {
    let frequency = calc_frequency(&[15, 1, 1, 2]);
    assert_eq!(frequency, 15)
  }

  #[test]
  fn test_calibation() {
    let calibration = calc_calibration(&[1, -1]);
    let calibration2 = calc_calibration(&[7, 7, -2, -7, -4]);

    assert_eq!(calibration.unwrap(), 0);
    assert_eq!(calibration2.unwrap(), 14);
  }
}
