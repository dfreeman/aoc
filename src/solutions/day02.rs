use crate::prelude::*;

solution!(day = 2);

#[parser]
fn parse(input: &str) -> String {
  input.trim().to_string()
}

#[solver(part = 1)]
fn part_1(input: String) -> String {
  input
}

#[solver(part = 2)]
fn part_2(input: String) -> String {
  input
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = indoc::indoc! {"

  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT)), "".to_string());
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT)), "".to_string());
  }
}
