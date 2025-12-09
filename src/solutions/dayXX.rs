use crate::prelude::*;

solution!(day = XX);

pub type Parsed = String;

#[parser]
fn parse(input: &str) -> Parsed {
  input.trim().to_string()
}

#[solver(part = 1)]
fn part_1(input: Parsed) -> usize {
  input.len()
}

#[solver(part = 2)]
fn part_2(input: Parsed) -> usize {
  input.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = indoc::indoc! {"

  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT)), 0);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT)), 0);
  }
}
