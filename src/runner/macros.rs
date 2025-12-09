#[macro_export]
macro_rules! solution {
  (day = $day:literal) => {
    mod solution {
      use $crate::runner::traits::Solve;

      pub struct Solution<const PART: u8>;

      impl<const PART: u8> $crate::runner::traits::DefaultSolution for &Solution<PART> {}

      #[::linkme::distributed_slice($crate::runner::SOLUTIONS)]
      static SOLUTION: (u8, fn(u8, &str) -> String) = ($day, |part, input| match (part) {
        1 => (&Solution::<1>).solve(input),
        2 => (&Solution::<2>).solve(input),
        _ => panic!("Invalid part: {}", part),
      });
    }
  };
}

#[macro_export]
macro_rules! solver {
  attr(part = $part:literal $(, args = [$($args:expr),*])?) ($(#[$meta:meta])* $vis:vis fn $name:ident $params:tt $(-> $ret:ty)? $body:block) => {
    $(#[$meta])*
    $vis fn $name $params $(-> $ret)? $body

    impl $crate::runner::traits::Solve for solution::Solution<$part> {
      fn solve(&self, input: &str) -> String {
        use $crate::runner::traits::Parse;
        $name(self.parse(input)$($(, $args)*)?).to_string()
      }
    }
  }
}

#[macro_export]
macro_rules! parser {
  attr() ($item:item) => {
    #[$crate::parser(part = 1)]
    #[$crate::parser(part = 2)]
    $item
  };

  attr(part = $part:literal) ($(#[$meta:meta])* $vis:vis fn $name:ident $params:tt -> $ret:ty $body:block) => {
    $(#[$meta])?
    $vis fn $name $params -> $ret $body

    impl $crate::runner::traits::Parse for solution::Solution<$part> {
      type Parsed = $ret;
      fn parse(&self, input: &str) -> Self::Parsed {
        $name(input)
      }
    }
  }
}

#[macro_export]
macro_rules! main {
  (year = $year:literal) => {
    pub fn main() {
      $crate::runner::run($year)
    }
  };
}
