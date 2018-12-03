use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

fn help() {
  println!(
    "usage:
day-one <file-path>
  return frequency for input file
day-one <cmd> <file-path>
  calibrate: find the first value that is reached twice"
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => help(),
    2 => {
      let input = read_input(&args[1]);
      println!("Frequency: {}", calc_frequency(&input))
    }
    3 => {
      let cmd = &args[1];
      let input = read_input(&args[2]);

      match &cmd[..] {
        "calibrate" => println!("Calibration: {}", calc_calibration(&input)),
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

fn calc_frequency(input: &str) -> String {
  let mut frequency = 0;
  for line in input.lines() {
    let number: i32 = line.trim().parse().unwrap();
    frequency += number;
  }
  frequency.to_string()
}

fn calc_calibration(input: &str) -> String {
  let mut lines = input.lines().cycle();
  let mut values = HashSet::new();
  let mut frequency = 0;
  values.insert(frequency);

  loop {
    let number: i32 = lines.next().unwrap().trim().parse().unwrap();
    frequency += number;
    if values.contains(&frequency) {
      break frequency.to_string();
    }
    values.insert(frequency);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_a_number() {
    let frequency = calc_frequency("+1");
    assert_eq!(frequency, "1")
  }

  #[test]
  fn test_subtract_a_number() {
    let frequency = calc_frequency("-1");
    assert_eq!(frequency, "-1")
  }

  #[test]
  fn test_multiple_numbers() {
    let frequency = calc_frequency(
      "+15
      +1
      +1
      -2",
    );
    assert_eq!(frequency, "15")
  }

  #[test]
  fn test_calibation() {
    let calibration = calc_calibration(
      "+1
       -1",
    );
    let calibration2 = calc_calibration(
      "+7
      +7
      -2
      -7
      -4",
    );

    assert_eq!(calibration, "0");
    assert_eq!(calibration2, "14");
  }
}
