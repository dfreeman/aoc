pub trait Parse {
  type Parsed;
  fn parse(&self, input: &str) -> Self::Parsed;
}

pub trait Solve {
  fn solve(&self, input: &str) -> String;
}

pub trait DefaultSolution {}

impl<T: DefaultSolution> Parse for T {
  type Parsed = String;
  fn parse(&self, input: &str) -> Self::Parsed {
    input.to_string()
  }
}

impl<T: DefaultSolution> Solve for T {
  fn solve(&self, _: &str) -> String {
    unimplemented!("Part not implemented")
  }
}
