use crate::prelude::*;
use itertools::Itertools;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

solution!(day = 11);

#[parser]
fn parse(input: &str) -> HashMap<String, Vec<String>> {
  use parse::*;

  HashMap::from_iter(input.parse_lines(separated_pair(
    alpha1.map(String::from),
    tag(": "),
    separated_list1(tag(" "), alpha1.map(String::from)),
  )))
}

#[solver(part = 1)]
fn part_1(data: HashMap<String, Vec<String>>) -> usize {
  count_paths(
    "you".to_string(),
    |server| data.get(server).unwrap_or(&vec![]).clone(),
    |server| server == "out",
  )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum State {
  Nothing,
  SeenDac,
  SeenFft,
  SeenBoth,
}

impl State {
  fn saw(&self, server: &str) -> State {
    match server {
      "dac" => self.saw_dac(),
      "fft" => self.saw_fft(),
      _ => *self,
    }
  }

  fn saw_dac(&self) -> State {
    match self {
      State::Nothing => State::SeenDac,
      State::SeenFft => State::SeenBoth,
      _ => *self,
    }
  }

  fn saw_fft(&self) -> State {
    match self {
      State::Nothing => State::SeenFft,
      State::SeenDac => State::SeenBoth,
      _ => *self,
    }
  }
}

#[solver(part = 2)]
fn part_2(data: HashMap<String, Vec<String>>) -> usize {
  count_paths(
    ("svr".to_string(), State::Nothing),
    |(server, state)| {
      data
        .get(server)
        .unwrap_or(&vec![])
        .iter()
        .map(|next| (next.clone(), state.saw(next)))
        .collect_vec()
    },
    |(server, state)| server == "out" && *state == State::SeenBoth,
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT_1: &str = indoc! {"
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT_1)), 5);
  }

  const SAMPLE_INPUT_2: &str = indoc! {"
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out
  "};

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT_2)), 2);
  }
}
