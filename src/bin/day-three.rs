use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

fn help() {
  println!(
    "usage:
day-three <file-path>
  return the overlap in square inches
day-three untouched <file-path>
  return the id of the claim that doesn't overlap"
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => help(),
    2 => {
      let input = read_input(&args[1]);
      println!("Numer of overlapping inches: {}", calc_overlap(&input));
    }
    3 => {
      let cmd = &args[1];
      let input = read_input(&args[2]);
      match &cmd[..] {
        "untouched" => println!("Id of untouched claim: {}", calc_untouched(&input)),
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

#[derive(Debug)]
struct Claim {
  id: u32,
  fabric_coords: HashSet<(u32, u32)>,
}

impl Claim {
  fn new(id: u32, x: u32, y: u32, width: u32, height: u32) -> Claim {
    let mut coords = HashSet::new();
    for i in x..x + width {
      for j in y..y + height {
        coords.insert((i, j));
      }
    }

    Claim {
      id: id,
      fabric_coords: coords,
    }
  }

  fn parse(line: &str) -> Option<Claim> {
    let mut tokens = line.trim().split_whitespace();

    let id: u32 = tokens
      .next()
      .unwrap()
      .split('#')
      .last()
      .unwrap()
      .parse()
      .unwrap();

    let _ = tokens.next().unwrap();

    let mut coords = tokens.next().unwrap().split(",");
    let x: u32 = coords.next().unwrap().parse().unwrap();
    let y: u32 = coords
      .last()
      .unwrap()
      .split(':')
      .next()
      .unwrap()
      .parse()
      .unwrap();

    let mut dims = tokens.next().unwrap().split("x");
    let width: u32 = dims.next().unwrap().parse().unwrap();
    let height: u32 = dims.last().unwrap().parse().unwrap();

    Some(Claim::new(id, x, y, width, height))
  }
}

fn overlap_map(claims: &[Claim]) -> HashMap<(u32, u32), u32> {
  let mut spaces = HashMap::new();
  for claim in claims.iter() {
    for coord in claim.fabric_coords.clone() {
      *spaces.entry(coord).or_insert(0) += 1;
    }
  }

  spaces
}

fn calc_overlap(input: &str) -> u32 {
  let claims: Vec<Claim> = input
    .lines()
    .map(|line| Claim::parse(&line).unwrap())
    .collect();
  let spaces = overlap_map(&claims);
  spaces.iter().filter(|(_, &val)| val > 1).count() as u32
}

fn calc_untouched(input: &str) -> u32 {
  let claims: Vec<Claim> = input
    .lines()
    .map(|line| Claim::parse(&line).unwrap())
    .collect();
  let spaces = overlap_map(&claims);

  claims
    .iter()
    .find(|claim| {
      claim
        .fabric_coords
        .iter()
        .all(|coord| *spaces.get(coord).unwrap() == 1u32)
    }).unwrap()
    .id
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_checksum() {
    let num_of_inches = calc_overlap(
      "#1 @ 1,3: 4x4
      #2 @ 3,1: 4x4
      #3 @ 5,5: 2x2",
    );
    assert_eq!(num_of_inches, 4);
  }

  #[test]
  fn test_calc_untouched() {
    let id_of_untouched = calc_untouched(
      "#1 @ 1,3: 4x4
      #2 @ 3,1: 4x4
      #3 @ 5,5: 2x2",
    );
    assert_eq!(id_of_untouched, 3);
  }

  #[test]
  fn test_claim_new() {
    let claim = Claim::new(1, 1, 1, 2, 2);
    let mut expected = HashSet::new();
    expected.insert((1, 1));
    expected.insert((1, 2));
    expected.insert((2, 1));
    expected.insert((2, 2));

    assert_eq!(claim.id, 1);
    assert_eq!(claim.fabric_coords, expected);
  }

  #[test]
  fn test_claim_parse() {
    let claim = Claim::parse("#1 @ 1,3: 4x4").unwrap();
    assert_eq!(claim.id, 1);
  }
}
