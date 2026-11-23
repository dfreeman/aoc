# Advent of Code 2026 🎄

I'm using AoC as a chance to get my hands dirty with a bit of Rust this year. I'm sure there's plenty in here that's unidiomatic or unperformant, but it's been a fun experience so far.

Each day is a module under `src/solutions` and each part can be run individually as long as an `AOC_AUTH_TOKEN` environment variable is set for fetching input:

```sh
cargo run --release -- --day 1 --part 1
```

By default, the solution will be printed, but with `--submit` it will be sent back to Advent of Code as a submission using the given session token.
