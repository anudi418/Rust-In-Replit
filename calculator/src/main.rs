// Lesson #42
use std::env;

fn main() {
  let args = env::args();
  println!("{:?}", args);

  let first_number = 1;
  let operator = '-';
  let second_number = 10;

  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  use crate::operate;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }

  #[test]
  fn operate_handles_addition() {
    let op = operate('+', -5, 200);
    assert_eq!(op, 195);
  }
  #[test]
  fn operate_handles_subtraction() {
    let op = operate('-', -12, -12);
    assert_eq!(op, 0);
  }
  #[test]
  fn operate_handles_division() {
    let op = operate('/', -12, -1);
    assert_eq!(op, 12);
  }
  #[test]
  fn operate_handles_multiplication() {
    let op = operate('*', -12, -2);
    assert_eq!(op, 24);
  }
  #[test]
  fn operate_handles_multiplication_x() {
    let op = operate('x', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  fn operate_handles_multiplication_cap_x() {
    let op = operate('X', -12, 2);
    assert_eq!(op, -24);
  }
  #[test]
  #[should_panic]
  fn operate_panics_on_invalid_op() {
    operate('a', 1, 1);
  }
}

