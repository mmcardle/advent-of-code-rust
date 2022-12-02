// ↳ main.rs
pub mod utils;

mod day1;
mod day2;

fn main() {
  //day1::day1("inputs/day1_test_input").expect("Couldnt read from stdin");
  //day1::day1("inputs/day1_input").expect("Couldnt read from stdin");
  
  day2::day2("inputs/day2_input").expect("Couldnt read from stdin");
  day2::day2("inputs/day2_test_input").expect("Couldnt read from stdin");
}
