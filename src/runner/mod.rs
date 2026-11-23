mod macros;

pub mod traits;

#[linkme::distributed_slice]
pub static SOLUTIONS: [(u8, fn(u8, &str) -> String)];

#[derive(clap::Parser)]
pub struct CliArgs {
  /// Which day(s) to run
  #[arg(short, long = "day", default_values_t = 1..=12)]
  pub days: Vec<u8>,

  /// Which part to run
  #[arg(short, long = "part", default_values_t = [1, 2])]
  pub parts: Vec<u8>,

  /// Whether the answer should be submitted automatically
  #[arg(short, long)]
  pub submit: bool,
}

pub fn run(year: u16) {
  use clap::Parser;

  let options = CliArgs::parse();
  let client = libaoc::Client::new().expect("Failed to create AoC client");
  for &day in &options.days {
    let puzzle_id = (year, day);
    let input = client
      .get_input(&puzzle_id)
      .expect("Failed to fetch puzzle input");

    for &part in &options.parts {
      let solve = SOLUTIONS
        .iter()
        .find_map(|&(d, solve)| if d == day { Some(solve) } else { None })
        .unwrap_or_else(|| panic!("No solution found for day {day}"));

      let answer = solve(part, &input);

      println!("Day {day}, Part {part}: {answer}");

      if options.submit {
        client
          .submit(&puzzle_id, Some(part), answer)
          .expect("Failed to submit answer");
      }
    }
  }
}
