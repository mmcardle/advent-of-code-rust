// ↳ main.rs
pub mod utils;

mod day1;

fn main() {
  day1::day1("inputs/day1_test_input").expect("Couldnt read from stdin");
  day1::day1("inputs/day1_input").expect("Couldnt read from stdin");
}
